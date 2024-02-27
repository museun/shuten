use shuten::{event::Event, Terminal};
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

pub fn run<R>(config: Config, mut ui: impl FnMut(Term<'_>) -> R) -> std::io::Result<()> {
    let mut terminal = Terminal::new(config)?;
    let mut application = Application::new(terminal.rect());

    let using_fixed_timer = terminal.using_fixed_timer();

    let mut quit = false;
    let mut last_blend = None;

    while let Ok(ev) = terminal.wait_for_next_event() {
        if ev.is_quit() {
            break;
        }

        if let Event::Blend(blend) = ev {
            last_blend.replace(blend);
        }

        application.start();
        application.handle(ev);

        let _ = ui(Term {
            terminal: &mut terminal,
            quit: &mut quit,
            application: &application,
            blend: using_fixed_timer.then_some(last_blend).flatten(),
        });

        application.finish();

        if terminal.is_in_alt_screen() {
            terminal.paint(|canvas| {
                application.paint(canvas);
            })?;
        }

        if quit {
            break;
        }
    }

    Ok(())
}

// fn start_view_server(rx: std::sync::mpsc::Receiver<Vec<u8>>) {
//     use std::sync::mpsc::Receiver;
//     fn run(mut rx: Receiver<Vec<u8>>) {
//         let server = std::net::TcpListener::bind("localhost:44334").unwrap();
//         loop {
//             if let Ok((client, _)) = server.accept() {
//                 if handle(client, &mut rx).is_err() {
//                     continue;
//                 }
//             }

//             if rx.recv().is_err() {
//                 break;
//             }
//         }
//     }

//     fn handle(mut w: impl std::io::Write, rx: &mut Receiver<Vec<u8>>) -> std::io::Result<()> {
//         while let Ok(data) = rx.recv() {
//             w.write_all(&data)?;
//             w.write_all(b"\n")?;
//             w.flush()?;
//         }
//         Ok(())
//     }

//     std::thread::spawn(move || run(rx));
// }
