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
            terminal.paint(|mut canvas| {
                canvas.erase();
                application.paint(canvas);
            })?;
        }

        if quit {
            break;
        }
    }

    Ok(())
}
