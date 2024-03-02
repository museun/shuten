#![cfg_attr(debug_assertions, allow(dead_code, unused_variables,))]
use shuten::{Config, Terminal};

mod ui;
pub use ui::{Response, Ui};

mod interest;
pub use interest::Interest;

pub(crate) mod debug_fmt;

mod node;

pub use node::{LayoutNode, Node, WidgetId};

pub mod input;

mod widget;

use widget::ErasedWidget;
pub use widget::{NoResponse, Widget, WidgetExt};

pub mod widgets;

pub(crate) mod ext;

pub fn run(config: Config, mut app: impl FnMut(&Ui)) -> std::io::Result<()> {
    let mut terminal = Terminal::new(config)?;
    let mut ui = Ui::new(terminal.rect());

    while let Ok(ev) = terminal.wait_for_next_event() {
        if ev.is_quit() {
            break;
        }
        if !terminal.is_in_alt_screen() {
            continue;
        }

        ui.scope(|ui| {
            ui.handle_event(&ev);
            app(ui)
        });

        terminal.paint(|mut canvas| {
            canvas.erase();
            ui.paint(canvas)
        })?;
    }

    Ok(())
}
