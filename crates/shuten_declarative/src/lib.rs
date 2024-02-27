use std::io::Write;

use shuten::{
    event::Event,
    renderer::{help::ExplainRenderer, metrics::FrameStats, TeeRenderer, TermRenderer},
    Terminal,
};
pub use shuten::{
    event::{Key, Modifiers, MouseButton},
    geom,
    style::{Attribute, Color, Hsl, Rgb},
    Canvas, Cell, Config,
};

pub mod input;
pub mod layout;
pub mod tree;
pub mod widgets;

mod paint;

mod widget;
#[doc(inline)]
pub use widget::{prelude, Response, Widget, WidgetExt};

pub mod context;
pub mod logger;

mod term;
pub use term::Term;

mod application;
pub(crate) use application::Application;

pub(crate) mod external;

pub fn run<R>(config: Config, ui: impl FnMut(Term<'_>) -> R) -> std::io::Result<()> {
    debug_run(config, false, ui)
}

pub fn debug_run<R>(
    config: Config,
    debug: bool,
    mut ui: impl FnMut(Term<'_>) -> R,
) -> std::io::Result<()> {
    profiling::register_thread!("shuten");

    let server_addr = format!("127.0.0.1:{}", puffin_http::DEFAULT_PORT);
    let _puffin_server = puffin_http::Server::new(&server_addr).unwrap();
    profiling::puffin::set_scopes_on(true);

    let mut terminal = Terminal::new(config)?;
    let mut application = Application::new(terminal.rect());

    let using_fixed_timer = terminal.using_fixed_timer();

    let mut quit = false;
    let mut frame_count = 0;
    let mut last_blend = None;
    let mut dump_render = false;

    let (tx, rx) = std::sync::mpsc::channel::<Vec<_>>();
    if debug {
        start_view_server(rx);
    }

    let mut out = std::fs::File::create("render.txt").unwrap();

    let mut stats = <FrameStats<30>>::new();
    while let Ok(ev) = terminal.wait_for_next_event() {
        profiling::scope!("event");
        if ev.is_quit() {
            break;
        }

        if let Event::Blend(blend) = ev {
            last_blend.replace(blend);
        }

        application.start();
        application.handle(ev);

        {
            profiling::scope!("call ui");
            let _ = ui(Term {
                terminal: &mut terminal,
                quit: &mut quit,
                application: &application,
                frame_count,
                blend: using_fixed_timer.then_some(last_blend).flatten(),
                dump_render: &mut dump_render,
                frame_stats: &mut stats,
            });
        }

        if debug {
            profiling::scope!("send debug frame");
            let data = serde_json::to_vec(&typed_json::json!({
                "frame": frame_count,
                "shuten": &application
            }))
            .unwrap();
            if tx.send(data).is_err() {
                break;
            }
        }

        application.finish();

        if terminal.is_in_alt_screen() {
            if dump_render {
                {
                    let mut canvas = terminal.canvas();
                    canvas.fill(Color::Reset);
                    application.paint(canvas);
                }

                let mut buf = vec![];
                terminal.with_writer_and_context(|writer, context| {
                    let mut explain = TeeRenderer::new(
                        ExplainRenderer::new(&mut buf), //
                        TermRenderer::new(writer),
                    );
                    context.end_frame(&mut explain)
                })?;

                out.write_all(&buf).unwrap();
                out.flush().unwrap();
            } else {
                profiling::scope!("terminal paint");
                let mut canvas = terminal.canvas();
                {
                    profiling::scope!("render");
                    canvas.fill(Color::Reset);
                    application.paint(canvas);
                }
                profiling::scope!("flush");
                terminal.flush_with_metrics(&mut stats)?;
            }
        }

        if quit {
            break;
        }

        frame_count += 1;
        profiling::finish_frame!();
    }

    Ok(())
}

fn start_view_server(rx: std::sync::mpsc::Receiver<Vec<u8>>) {
    use std::sync::mpsc::Receiver;
    fn run(mut rx: Receiver<Vec<u8>>) {
        let server = std::net::TcpListener::bind("localhost:44334").unwrap();
        loop {
            if let Ok((client, _)) = server.accept() {
                if handle(client, &mut rx).is_err() {
                    continue;
                }
            }

            if rx.recv().is_err() {
                break;
            }
        }
    }

    fn handle(mut w: impl std::io::Write, rx: &mut Receiver<Vec<u8>>) -> std::io::Result<()> {
        while let Ok(data) = rx.recv() {
            w.write_all(&data)?;
            w.write_all(b"\n")?;
            w.flush()?;
        }
        Ok(())
    }

    std::thread::spawn(move || run(rx));
}
