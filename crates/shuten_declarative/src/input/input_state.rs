use std::cell::Cell;

use shuten::{
    event::{Event as CoreEvent, Modifiers, MouseButton},
    geom::Vec2f,
};

use crate::{
    context::EventCtx,
    geom::Pos2f,
    layout::Layout,
    tree::{Node, Tree, WidgetId},
};

use super::{
    mouse::{ButtonState, Intersections, Mouse},
    Event, Handled, MouseEvent, TranslateKeyEvent, TranslateMouseEvent,
};

#[derive(Debug, Default, serde::Serialize)]
pub struct Input {
    mouse: Mouse,
    modifiers: Modifiers,
    intersections: Intersections,
    selection: Cell<Option<WidgetId>>,
    last_selection: Cell<Option<WidgetId>>,
    last_event: Option<CoreEvent>,
}

impl Input {
    pub fn selection(&self) -> Option<WidgetId> {
        self.selection.get()
    }

    pub fn set_selection(&self, id: Option<WidgetId>) {
        self.selection.set(id)
    }
}

impl Input {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn last_event(&self) -> Option<&CoreEvent> {
        self.last_event.as_ref()
    }

    pub(crate) fn start(&self, tree: &Tree, layout: &Layout) {
        // notify the focused element if they've gained or lost focus
        let mut current = self.selection.get();
        let last = self.last_selection.get();

        if current == last {
            return;
        }

        if let Some(entered) = current {
            if let Some(mut node) = tree.get_mut(entered) {
                let ctx = EventCtx {
                    tree,
                    layout,
                    input: self,
                };
                Self::emit(ctx, entered, &mut node, &Event::FocusGained);
            } else {
                self.selection.set(None);
                current = None;
            }
        }

        if let Some(exited) = last {
            if let Some(mut node) = tree.get_mut(exited) {
                let ctx = EventCtx {
                    tree,
                    layout,
                    input: self,
                };
                Self::emit(ctx, exited, &mut node, &Event::FocusLost);
            }
        }

        self.last_selection.set(current)
    }

    pub(crate) fn finish(&mut self) {
        for state in self.mouse.buttons.values_mut() {
            state.interpolate();
        }
    }

    pub(crate) fn handle(&mut self, tree: &Tree, layout: &Layout, event: &CoreEvent) -> Handled {
        self.last_event.replace(*event);

        match event {
            CoreEvent::Mouse(ev, modifiers) => {
                self.update_modifiers(*modifiers);
                self.mouse_changed(tree, layout, ev)
            }
            CoreEvent::Keyboard(key, modifiers) => {
                self.update_modifiers(*modifiers);
                self.keyboard_key_changed(tree, layout, key)
            }
            _ => Handled::Bubble,
        }
    }
}

impl Input {
    pub fn mouse_changed(
        &mut self,
        tree: &Tree,
        layout: &Layout,
        ev: &impl TranslateMouseEvent,
    ) -> Handled {
        let Some(event) = ev.translate() else {
            return Handled::Bubble;
        };

        match event {
            MouseEvent::MouseMove { pos } => {
                self.mouse_moved(tree, layout, pos);
                Handled::Bubble
            }

            MouseEvent::MouseClicked { button, .. } => {
                self.mouse_button_changed(tree, layout, button, true)
            }

            MouseEvent::MouseHeld { button, .. } => {
                self.mouse_button_changed(tree, layout, button, false)
            }

            MouseEvent::MouseDragStart { pos, button } => {
                let event = Event::MouseDrag {
                    origin: pos,
                    pos,
                    delta: Vec2f::ZERO,
                    modifiers: self.modifiers,
                    button,
                };
                self.send_mouse_event(tree, layout, &event)
            }

            MouseEvent::MouseDrag {
                origin,
                pos,
                delta,
                button,
            } => {
                let event = Event::MouseDrag {
                    origin,
                    pos,
                    delta,
                    modifiers: self.modifiers,
                    button,
                };
                self.send_mouse_event(tree, layout, &event)
            }

            MouseEvent::MouseDragReleased {
                origin,
                pos,
                delta,
                button,
            } => {
                let event = Event::MouseDragRelease {
                    origin,
                    pos,
                    delta,
                    modifiers: self.modifiers,
                    button,
                };
                self.send_mouse_event(tree, layout, &event)
            }

            MouseEvent::MouseScroll { delta, pos } => {
                let event = Event::MouseScroll {
                    pos,
                    delta,
                    modifiers: self.modifiers,
                };
                self.send_mouse_event(tree, layout, &event)
            }
        }
    }

    pub fn keyboard_key_changed(
        &self,
        tree: &Tree,
        layout: &Layout,
        key: &impl TranslateKeyEvent,
    ) -> Handled {
        let Some(key) = key.translate() else {
            return Handled::Bubble;
        };

        let event = Event::KeyChanged {
            key,
            modifiers: self.modifiers,
        };

        let mut resp = Handled::Bubble;
        for (id, _) in layout.keyboard.iter() {
            let Some(mut node) = tree.get_mut(id) else {
                continue;
            };

            let ctx = EventCtx {
                tree,
                layout,
                input: self,
            };
            resp = Self::emit(ctx, id, &mut node, &event);
            if resp.is_sink() {
                break;
            }
        }

        if let Some(id) = self.selection.get() {
            let Some(node) = layout.get(id) else {
                return Handled::Bubble;
            };

            if node.interest.is_focus_input() {
                let mut node = tree.get_mut(id).unwrap();
                let ctx = EventCtx {
                    tree,
                    layout,
                    input: self,
                };
                return Self::emit(ctx, id, &mut node, &event);
            }
        }

        resp
    }

    pub fn update_modifiers(&mut self, modifiers: Modifiers) {
        self.modifiers = modifiers;
    }

    pub fn mouse_moved(&mut self, tree: &Tree, layout: &Layout, pos: Pos2f) {
        self.mouse.pos = pos;

        let event = Event::MouseMoved {
            pos: self.mouse.pos,
        };

        // send mouse move event
        for (id, interest) in layout.mouse.iter() {
            if interest.is_mouse_move() {
                if let Some(mut node) = tree.get_mut(id) {
                    let ctx = EventCtx {
                        tree,
                        layout,
                        input: self,
                    };
                    Self::emit(ctx, id, &mut node, &event);
                }
            }
        }

        // do a hit test
        self.intersections.hit.clear();
        Self::hit_test(tree, layout, pos, &mut self.intersections.hit);

        // send mouse enter event
        for &hit in &self.intersections.hit {
            let Some(mut node) = tree.get_mut(hit) else {
                continue;
            };

            if !self.intersections.entered.contains(&hit) {
                self.intersections.entered.push(hit);
                let ctx = EventCtx {
                    tree,
                    layout,
                    input: self,
                };
                if Self::emit(ctx, hit, &mut node, &Event::MouseEnter).is_sink() {
                    self.intersections.entered_and_sunk.push(hit);
                    break;
                }
            }

            if self.intersections.entered_and_sunk.contains(&hit) {
                break;
            }
        }

        // send mouse leave event
        let mut inactive = vec![];
        for &hit in &self.intersections.entered {
            if !self.intersections.hit.contains(&hit) {
                if let Some(mut node) = tree.get_mut(hit) {
                    let ctx = EventCtx {
                        tree,
                        layout,
                        input: self,
                    };
                    Self::emit(ctx, hit, &mut node, &Event::MouseLeave);
                }
                inactive.push(hit);
            }
        }
        for inactive_id in inactive {
            self.intersections.entered.retain(|&id| id != inactive_id);
            self.intersections
                .entered_and_sunk
                .retain(|&id| id != inactive_id);
        }
    }
}

impl Input {
    fn mouse_button_changed(
        &mut self,
        tree: &Tree,
        layout: &Layout,
        button: MouseButton,
        down: bool,
    ) -> Handled {
        let state = self
            .mouse
            .buttons
            .entry(button)
            .or_insert(ButtonState::Released);

        match (state.is_down(), down) {
            (true, true) | (false, false) => {}
            (false, true) => *state = ButtonState::Down,
            (true, false) => *state = ButtonState::Up,
        }

        let event = if down {
            Event::MouseHeld {
                button,
                inside: false,
                pos: self.mouse.pos,
                modifiers: self.modifiers,
            }
        } else {
            Event::MouseRelease {
                button,
                inside: false,
                pos: self.mouse.pos,
                modifiers: self.modifiers,
            }
        };

        let resp = self.send_mouse_event(tree, layout, &event);
        for (id, interest) in layout.mouse.iter() {
            if !(interest.is_mouse_outside() && self.intersections.hit.contains(&id)) {
                continue;
            }
            let Some(mut node) = tree.get_mut(id) else {
                continue;
            };

            let ctx = EventCtx {
                tree,
                layout,
                input: self,
            };
            Self::emit(ctx, id, &mut node, &event);
        }
        resp
    }

    fn send_mouse_event(&self, tree: &Tree, layout: &Layout, event: &Event) -> Handled {
        let mut resp = Handled::Bubble;

        for &id in &self.intersections.hit {
            let Some(mut node) = tree.get_mut(id) else {
                continue;
            };

            let ctx = EventCtx {
                tree,
                layout,
                input: self,
            };
            if Self::emit(ctx, id, &mut node, event).is_sink() {
                resp = Handled::Sink;
                break;
            }
        }
        resp
    }

    fn emit(ctx: EventCtx, id: WidgetId, node: &mut Node, event: &Event) -> Handled {
        let tree = ctx.tree;
        tree.enter(id);
        let resp = node.widget.event(ctx, event);
        tree.exit(id);
        resp
    }
}

impl Input {
    fn hit_test(_tree: &Tree, layout: &Layout, pos: Pos2f, mouse_hit: &mut Vec<WidgetId>) {
        for (id, _) in layout.mouse.iter() {
            let Some(mut node) = layout.get(id) else {
                continue;
            };

            let mut rect = node.rect;
            while let Some(parent) = node.clipped_by {
                node = &layout[parent];
                rect = rect.constrain(node.rect);
            }

            if node.rect.contains(pos) {
                mouse_hit.push(id);
            }
        }
    }
}
