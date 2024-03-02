use shuten::{
    event::{Event, Modifiers, MouseButton, MouseEvent},
    geom::{Pos2f, Vec2f},
};
use slotmap::{SecondaryMap, SlotMap};

use super::{ErasedWidget, LayoutNode, Node};
use crate::WidgetId;

mod events;
pub use events::*;

mod mouse;
use mouse::ButtonState;
pub(crate) use mouse::Mouse;

mod keyboard;
pub(crate) use keyboard::Keyboard;

mod layered;
use layered::Layered;

#[derive(Copy, Clone, Debug, PartialEq, Default)]
pub enum Handled {
    Sink,
    #[default]
    Bubble,
}

#[derive(Default, Debug)]
struct Intersections {
    hit: Vec<WidgetId>,
    entered: Vec<WidgetId>,
    entered_and_sunk: Vec<WidgetId>,
}

#[derive(Debug, Default)]
pub struct Input {
    pub(crate) mouse: Mouse,
    pub(crate) keyboard: Keyboard,
    modifiers: Modifiers,
    intersections: Intersections,
    // focus: Option<WidgetId>,
    // last_focus: Option<WidgetId>,
    last_event: Option<Event>,
}

impl Input {
    pub(crate) fn start(&mut self) {
        // notify focus
    }

    pub(crate) fn end(&mut self) {
        // interpolate
        self.mouse.layered.clear();
        self.keyboard.layered.clear();
    }

    pub(crate) fn handle(
        &mut self,
        event: &Event,
        nodes: &mut SlotMap<WidgetId, Node>,
        layout: &mut SecondaryMap<WidgetId, LayoutNode>,
        stack: &mut Vec<WidgetId>,
    ) -> Handled {
        self.last_event = Some(*event);
        match *event {
            Event::Mouse(event, modifiers) => {
                self.modifiers = modifiers;
                self.mouse_event(event, nodes, layout, stack)
            }
            Event::Keyboard(key, modifiers) => {
                self.modifiers = modifiers;
                let event = KeyPressed {
                    key,
                    modifiers: self.modifiers,
                };
                self.keyboard.dispatch(event, nodes, stack)
            }
            _ => Handled::Bubble,
        }
    }

    fn mouse_event(
        &mut self,
        event: MouseEvent,
        nodes: &mut SlotMap<WidgetId, Node>,
        layout: &mut SecondaryMap<WidgetId, LayoutNode>,
        stack: &mut Vec<WidgetId>,
    ) -> Handled {
        self.mouse.pos = event.pos().into();

        let mut ctx = MouseContext {
            nodes,
            layout,
            stack,
            mouse: &mut self.mouse,
            intersections: &mut self.intersections,
        };

        match event {
            MouseEvent::Move { pos } => ctx.mouse_move(MouseMove { pos: pos.into() }),

            MouseEvent::Clicked { pos, button } => ctx.mouse_button(
                MouseClick {
                    pos: pos.into(),
                    button,
                    modifiers: self.modifiers,
                },
                ButtonState::Released,
                button,
                |widget, event| widget.on_mouse_click(event),
            ),

            MouseEvent::Held { pos, button } => ctx.mouse_button(
                MouseHeld {
                    pos: pos.into(),
                    button,
                    modifiers: self.modifiers,
                },
                ButtonState::Held,
                button,
                |widget, event| widget.on_mouse_held(event),
            ),

            MouseEvent::DragStart { pos, button } => ctx.mouse_drag(
                MouseDrag {
                    released: false,
                    origin: pos.into(),
                    pos: pos.into(),
                    delta: Vec2f::ZERO,
                    button,
                    modifiers: self.modifiers,
                },
                ButtonState::Held,
            ),

            MouseEvent::Drag {
                origin,
                pos,
                delta,
                button,
            } => ctx.mouse_drag(
                MouseDrag {
                    released: false,
                    origin: origin.into(),
                    pos: pos.into(),
                    delta: delta.into(),
                    button,
                    modifiers: self.modifiers,
                },
                ButtonState::Held,
            ),

            MouseEvent::DragReleased {
                origin,
                pos,
                delta,
                button,
            } => ctx.mouse_drag(
                MouseDrag {
                    released: true,
                    origin: origin.into(),
                    pos: pos.into(),
                    delta: delta.into(),
                    button,
                    modifiers: self.modifiers,
                },
                ButtonState::Released,
            ),

            MouseEvent::Scroll { dir, pos } => ctx.mouse_scroll(MouseScroll {
                pos: pos.into(),
                delta: dir.into(),
                modifiers: self.modifiers,
            }),

            _ => Handled::Bubble,
        }
    }
}

struct MouseContext<'a> {
    nodes: &'a mut SlotMap<WidgetId, Node>,
    layout: &'a mut SecondaryMap<WidgetId, LayoutNode>,
    mouse: &'a mut Mouse,
    stack: &'a mut Vec<WidgetId>,
    intersections: &'a mut Intersections,
}

impl<'a> MouseContext<'a> {
    fn mouse_move(&mut self, event: MouseMove) -> Handled {
        for (id, interest) in self.mouse.layered.iter() {
            if !interest.is_mouse_move() {
                continue;
            }
            if let Some(node) = self.nodes.get_mut(id) {
                self.stack.push(id);
                node.widget.on_mouse_move(event);
                assert_eq!(Some(id), self.stack.pop());
            }
        }

        self.intersections.hit.clear();
        self.hit_test(event.pos);

        for &hit in &self.intersections.hit {
            let Some(node) = self.nodes.get_mut(hit) else {
                continue;
            };

            if !self.intersections.entered.contains(&hit) {
                self.intersections.entered.push(hit);
                self.stack.push(hit);
                let resp = node.widget.on_mouse_enter(event);
                assert_eq!(Some(hit), self.stack.pop());
                if matches!(resp, Handled::Sink) {
                    self.intersections.entered_and_sunk.push(hit);
                    break;
                }
            }

            if self.intersections.entered_and_sunk.contains(&hit) {
                break;
            }
        }

        let mut inactive = vec![];
        for &hit in &self.intersections.hit {
            if self.intersections.hit.contains(&hit) {
                continue;
            }
            if let Some(node) = self.nodes.get_mut(hit) {
                self.stack.push(hit);
                node.widget.on_mouse_leave(event);
                assert_eq!(Some(hit), self.stack.pop());
            }
            inactive.push(hit)
        }

        for inactive in inactive {
            self.intersections.entered.retain(|&id| id != inactive);
            self.intersections
                .entered_and_sunk
                .retain(|&id| id != inactive)
        }

        Handled::Bubble
    }

    fn mouse_button<T: Copy>(
        &mut self,
        event: T,
        state: ButtonState,
        button: MouseButton,
        mut f: impl FnMut(&mut Box<dyn ErasedWidget>, T) -> Handled,
    ) -> Handled {
        self.mouse.buttons.insert(button, state);

        let resp = self.send_mouse_event(event, &mut f);

        for (id, interest) in self.mouse.layered.iter() {
            if !(interest.is_mouse_outside() && self.intersections.hit.contains(&id)) {
                continue;
            }

            let Some(node) = self.nodes.get_mut(id) else {
                continue;
            };
            self.stack.push(id);
            f(&mut node.widget, event);
            assert_eq!(Some(id), self.stack.pop());
        }

        resp
    }

    fn mouse_drag(&mut self, event: MouseDrag, button: ButtonState) -> Handled {
        self.mouse.buttons.insert(event.button, button);
        self.send_mouse_event(event, |widget, event| widget.on_mouse_drag(event))
    }

    fn mouse_scroll(&mut self, event: MouseScroll) -> Handled {
        self.send_mouse_event(event, |widget, event| widget.on_mouse_scroll(event))
    }

    fn send_mouse_event<T: Copy>(
        &mut self,
        event: T,
        mut f: impl FnMut(&mut Box<dyn ErasedWidget>, T) -> Handled,
    ) -> Handled {
        let mut resp = Handled::Bubble;
        for &id in &self.intersections.hit {
            let Some(node) = self.nodes.get_mut(id) else {
                continue;
            };
            self.stack.push(id);
            resp = f(&mut node.widget, event);
            assert_eq!(Some(id), self.stack.pop());
            if matches!(resp, Handled::Sink) {
                break;
            }
        }
        resp
    }

    fn hit_test(&mut self, pos: Pos2f) {
        for (id, _) in self.mouse.layered.iter() {
            let Some(node) = self.layout.get(id) else {
                continue;
            };

            // TODO if clipped constrain by the layout node size
            if node.rect.contains(pos) {
                self.intersections.hit.push(id);
            }
        }
    }
}
