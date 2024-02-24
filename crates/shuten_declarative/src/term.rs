use shuten::{geom::Rectf, Terminal};

use crate::Application;

pub struct Term<'a> {
    pub(crate) quit: &'a mut bool,
    pub(crate) terminal: &'a mut Terminal,
    pub(crate) application: &'a Application,
    pub(crate) frame_count: u64,
    pub(crate) blend: Option<f32>,
}

impl<'a> Term<'a> {
    pub fn rect(&self) -> Rectf {
        self.terminal.rect().into()
    }

    pub fn terminal(&mut self) -> &mut Terminal {
        self.terminal
    }

    pub const fn application(&self) -> &Application {
        self.application
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

    pub fn set_title(&self, title: impl AsRef<str>) -> std::io::Result<()> {
        self.terminal.set_title(title.as_ref())
    }
}
