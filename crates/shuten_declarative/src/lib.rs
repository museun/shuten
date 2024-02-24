use std::{io::Write, net::TcpListener, sync::mpsc::RecvTimeoutError};

use shuten::{
    event::Event,
    geom::{Rect, Rectf},
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
pub use widget::{NoResponse, Response, Widget, WidgetExt};

pub mod context {
    #[doc(inline)]
    pub use crate::input::ctx::EventCtx;
    #[doc(inline)]
    pub use crate::layout::ctx::LayoutCtx;
    #[doc(inline)]
    pub use crate::paint::ctx::PaintCtx;
}

pub mod logger;

pub struct Term<'a> {
    quit: &'a mut bool,
    terminal: &'a mut Terminal,
    shuten: &'a Shuten,
    frame_count: u64,
    blend: Option<f32>,
}

impl<'a> std::fmt::Debug for Term<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Term")
            .field("quit", &self.quit)
            .field("shuten", &self.shuten)
            .field("frame_count", &self.frame_count)
            .field("blend", &self.blend)
            .finish_non_exhaustive()
    }
}

impl<'a> Term<'a> {
    pub fn rect(&self) -> Rectf {
        self.terminal.rect().into()
    }

    pub fn terminal(&mut self) -> &mut Terminal {
        &mut self.terminal
    }

    pub const fn shuten(&self) -> &Shuten {
        self.shuten
    }

    pub const fn current_frame(&self) -> u64 {
        self.frame_count
    }

    pub fn blend(&self) -> f32 {
        self.blend.unwrap_or(1.0)
    }

    pub fn request_quit(&mut self) {
        *self.quit = true;
    }

    pub fn focus_previous(&self) {
        self.shuten.set_focus()
    }

    pub fn clear_focus(&self) {
        self.shuten.clear_focus()
    }

    pub fn set_title(&self, title: impl AsRef<str>) -> std::io::Result<()> {
        self.terminal.set_title(title.as_ref())
    }
}

fn start_view_server(rx: std::sync::mpsc::Receiver<Vec<u8>>) {
    use std::time::Duration;
    std::thread::spawn(move || {
        log::info!("listening for debug connections on: localhost:44334");
        let server = TcpListener::bind("localhost:44334").unwrap();
        loop {
            if let Ok((mut client, _)) = server.accept() {
                log::info!("got a new client");
                while let Ok(data) = rx.recv() {
                    if client
                        .write_all(&data)
                        .and_then(|()| client.write_all(b"\n"))
                        .and_then(|()| client.flush())
                        .is_err()
                    {
                        break;
                    }
                }
                log::info!("client disconnected");
            }

            log::info!("draining the buffer");
            if rx.recv_timeout(Duration::from_secs(1)) == Err(RecvTimeoutError::Disconnected) {
                break;
            }
        }
    });
}

pub fn debug_run<R>(
    config: Config,
    debug: bool,
    mut ui: impl FnMut(Term<'_>) -> R,
) -> std::io::Result<()> {
    let mut terminal = Terminal::new(config)?;
    let mut shuten = Shuten::new(terminal.rect());

    let mut quit = false;

    let mut frame_count = 0;
    let using_fixed_timer = terminal.using_fixed_timer();
    let mut last_blend = None;

    let (tx, rx) = std::sync::mpsc::channel::<Vec<_>>();
    if debug {
        start_view_server(rx);
    }
    while let Ok(ev) = terminal.wait_for_next_event() {
        if ev.is_quit() {
            break;
        }

        if let Event::Blend(blend) = ev {
            last_blend.replace(blend);
        }

        shuten.start();
        shuten.handle(ev);

        let _ = ui(Term {
            terminal: &mut terminal,
            quit: &mut quit,
            shuten: &shuten,
            frame_count,
            blend: using_fixed_timer.then_some(last_blend).flatten(),
        });

        if debug {
            let d = typed_json::json!({
                "frame": frame_count,
                "shuten": &shuten
            });
            let d = serde_json::to_vec(&d).unwrap();
            if tx.send(d).is_err() {
                break;
            }
        }

        shuten.finish();

        if terminal.is_in_alt_screen() {
            terminal.paint(|mut canvas| {
                canvas.fill(Color::Reset);
                shuten.paint(canvas);
            })?;
        }

        if quit {
            break;
        }

        frame_count += 1;
    }

    Ok(())
}

pub fn run<R>(config: Config, mut ui: impl FnMut(Term<'_>) -> R) -> std::io::Result<()> {
    debug_run(config, false, ui)
}

#[derive(Debug, serde::Serialize)]
pub struct Shuten {
    tree: tree::Tree,
    layout: layout::Layout,
    paint: paint::Paint,
    input: input::Input,
}

impl Shuten {
    fn new(rect: Rect) -> Self {
        Self {
            tree: tree::Tree::new(),
            layout: layout::Layout::new(rect.into()),
            paint: paint::Paint::new(),
            input: input::Input::new(),
        }
    }

    fn set_focus(&self) {
        let children = &self.tree.get_current().children;
        let id = children.first().copied();
        self.input.set_selection(id);
    }

    fn clear_focus(&self) {
        self.input.set_selection(None);
    }

    fn handle(&mut self, event: Event) -> bool {
        let resp = self.input.handle(&self.tree, &self.layout, &event);
        if let Event::Invalidate(rect) = event {
            self.layout.resize(rect.into());
        }
        resp == input::Handled::Sink
    }

    fn start(&mut self) {
        self.tree.start();
        self.input.start(&self.tree, &self.layout);
        self.paint.start();
        crate::tree::bind(&self.tree);
    }

    fn finish(&mut self) {
        crate::tree::unbind();
        self.tree.finish();
        self.layout.finish(&self.tree, &self.input);
        self.input.finish();
    }

    fn paint(&mut self, canvas: Canvas<'_>) {
        self.paint.paint_all(&self.tree, &self.layout, canvas);
    }
}

pub(crate) mod external;
