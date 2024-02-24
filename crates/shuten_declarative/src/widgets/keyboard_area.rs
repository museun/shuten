use shuten::event::Modifiers;

use crate::{
    input::{KeyEventKind, Keybind},
    widget::prelude::*,
};

#[derive(Debug, Clone)]
pub struct KeyResponse {
    pub keybind: Option<Keybind>,
}

impl KeyResponse {
    pub fn is_keybind(&self, keybind: Keybind) -> bool {
        self.keybind.filter(|&c| c == keybind).is_some()
    }
}

#[derive(Default, Debug)]
pub struct KeyboardArea;

impl KeyboardArea {
    pub fn show(self) -> Response<KeyResponse> {
        KeyboardWidget::show(self)
    }

    pub fn show_children(self, children: impl FnOnce()) -> Response<KeyResponse> {
        KeyboardWidget::show_children(children, self)
    }
}

#[derive(Default, Debug)]
struct KeyboardWidget {
    props: KeyboardArea,
    key: Option<KeyEventKind>,
    modifiers: Option<Modifiers>,
}

impl Widget for KeyboardWidget {
    type Props<'a> = KeyboardArea;
    type Response = KeyResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
        let modifiers = self.modifiers.take().unwrap_or_default();
        KeyResponse {
            keybind: self.key.take().map(|key| Keybind::new(key, modifiers)),
        }
    }

    fn interest(&self) -> Interest {
        Interest::KEY_INPUT
    }

    fn event(&mut self, _: EventCtx<'_>, event: &Event) -> Handled {
        if let &Event::KeyChanged { key, modifiers } = event {
            self.key = Some(key.kind);
            self.modifiers = Some(modifiers);
        }

        Handled::Bubble
    }
}

pub fn keyboard_area(children: impl FnOnce()) -> Response<KeyResponse> {
    KeyboardArea.show_children(children)
}

pub fn key_pressed(keybind: Keybind) -> bool {
    KeyboardArea.show().is_keybind(keybind)
}
