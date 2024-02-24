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
    selection: Option<WidgetId>,
    last_selection: Option<WidgetId>,
}

impl Input {
    pub fn selection(&self) -> Option<WidgetId> {
        self.selection
    }

    pub fn set_selection(&mut self, id: Option<WidgetId>) {
        self.selection = id
    }
}

impl Input {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn start(&mut self, tree: &Tree, layout: &Layout) {
        self.notify(tree, layout)
    }

    pub(crate) fn finish(&mut self) {
        for state in self.mouse.buttons.values_mut() {
            state.interpolate();
        }
    }

    pub(crate) fn handle(&mut self, tree: &Tree, layout: &Layout, event: &CoreEvent) -> Handled {
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
                self.mouse_drag(tree, layout, pos, pos, Vec2f::ZERO, button)
            }

            MouseEvent::MouseDrag {
                origin,
                pos,
                delta,
                button,
            } => self.mouse_drag(tree, layout, origin, pos, delta, button),

            MouseEvent::MouseDragReleased {
                origin,
                pos,
                delta,
                button,
            } => self.mouse_drag_release(tree, layout, origin, pos, delta, button),

            MouseEvent::MouseScroll { delta, pos } => self.mouse_scroll(tree, layout, pos, delta),
        }
    }

    pub fn keyboard_key_changed(
        &mut self,
        tree: &Tree,
        layout: &Layout,
        key: &impl TranslateKeyEvent,
    ) -> Handled {
        let Some(key) = key.translate() else {
            return Handled::Bubble;
        };

        for (id, _) in layout.keyboard.iter() {
            if let Some(mut node) = tree.get_mut(id) {
                let event = Event::KeyChanged {
                    key,
                    modifiers: self.modifiers,
                };
                // TODO this swallows focused events
                return self.emit(tree, layout, id, &mut node, &event);
            }
        }

        if let Some(id) = self.selection {
            let Some(node) = layout.get(id) else {
                return Handled::Bubble;
            };

            if node.interest.is_focus_input() {
                let mut node = tree.get_mut(id).unwrap();
                let event = Event::KeyChanged {
                    key,
                    modifiers: self.modifiers,
                };
                return self.emit(tree, layout, id, &mut node, &event);
            }
        }

        Handled::Bubble
    }

    pub fn update_modifiers(&mut self, modifiers: Modifiers) {
        self.modifiers = modifiers;
    }

    pub fn mouse_moved(&mut self, tree: &Tree, layout: &Layout, pos: Pos2f) {
        let _ = self.mouse.pos.replace(pos);
        self.send_mouse_move(tree, layout);
        self.mouse_hit_test(tree, layout);
        self.send_mouse_enter(tree, layout);
        self.send_mouse_leave(tree, layout);
    }

    fn send_mouse_move(&mut self, tree: &Tree, layout: &Layout) {
        let event = Event::MouseMoved {
            pos: self.mouse.pos,
        };
        for (id, interest) in layout.mouse.iter() {
            if interest.is_mouse_move() {
                if let Some(mut node) = tree.get_mut(id) {
                    self.emit(tree, layout, id, &mut node, &event);
                }
            }
        }
    }

    fn send_mouse_enter(&mut self, tree: &Tree, layout: &Layout) {
        let mut hit = std::mem::take(&mut self.intersections.hit);

        for &hit in &hit {
            let Some(mut node) = tree.get_mut(hit) else {
                continue;
            };

            if !self.intersections.entered.contains(&hit) {
                self.intersections.entered.push(hit);
                if self
                    .emit(tree, layout, hit, &mut node, &Event::MouseEnter)
                    .is_sink()
                {
                    self.intersections.entered_and_sunk.push(hit);
                    break;
                }
            }

            if self.intersections.entered_and_sunk.contains(&hit) {
                break;
            }
        }

        std::mem::swap(&mut self.intersections.hit, &mut hit);
    }

    fn send_mouse_leave(&mut self, tree: &Tree, layout: &Layout) {
        let mut dead = vec![];
        let mut entered = std::mem::take(&mut self.intersections.entered);
        for &hit in &entered {
            if !self.intersections.hit.contains(&hit) {
                if let Some(mut node) = tree.get_mut(hit) {
                    self.emit(tree, layout, hit, &mut node, &Event::MouseLeave);
                }
                dead.push(hit);
            }
        }
        std::mem::swap(&mut self.intersections.entered, &mut entered);

        for dead in dead {
            self.intersections.entered.retain(|&id| id != dead);
            self.intersections.entered_and_sunk.retain(|&id| id != dead);
        }
    }

    fn mouse_hit_test(&mut self, tree: &Tree, layout: &Layout) {
        self.intersections.hit.clear();
        if let Some(pos) = self.mouse.pos {
            Self::hit_test(tree, layout, pos, &mut self.intersections.hit)
        }
    }

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

        self.button_change(tree, layout, button, down)
    }

    fn button_change(
        &mut self,
        tree: &Tree,
        layout: &Layout,
        button: MouseButton,
        down: bool,
    ) -> Handled {
        let mut resp = Handled::Bubble;

        let mut hit = std::mem::take(&mut self.intersections.hit);
        for &id in &hit {
            let Some(mut node) = tree.get_mut(id) else {
                continue;
            };
            let event = if down {
                Event::MouseHeld {
                    button,
                    inside: true,
                    pos: self.mouse.pos.unwrap(),
                    modifiers: self.modifiers,
                }
            } else {
                Event::MouseRelease {
                    button,
                    inside: true,
                    pos: self.mouse.pos.unwrap(),
                    modifiers: self.modifiers,
                }
            };
            if self.emit(tree, layout, id, &mut node, &event) == Handled::Sink {
                resp = Handled::Sink;
                break;
            }
        }
        std::mem::swap(&mut self.intersections.hit, &mut hit);

        for (id, interest) in layout.mouse.iter() {
            if !(interest.is_mouse_outside() && self.intersections.hit.contains(&id)) {
                continue;
            }

            let Some(mut node) = tree.get_mut(id) else {
                continue;
            };
            let event = if down {
                Event::MouseHeld {
                    button,
                    inside: false,
                    pos: self.mouse.pos.unwrap(),
                    modifiers: self.modifiers,
                }
            } else {
                Event::MouseRelease {
                    button,
                    inside: false,
                    pos: self.mouse.pos.unwrap(),
                    modifiers: self.modifiers,
                }
            };

            self.emit(tree, layout, id, &mut node, &event);
        }

        resp
    }

    fn mouse_scroll(&mut self, tree: &Tree, layout: &Layout, pos: Pos2f, delta: Vec2f) -> Handled {
        let mut resp = Handled::Bubble;
        let mut hit = std::mem::take(&mut self.intersections.hit);
        for &id in &hit {
            let Some(mut node) = tree.get_mut(id) else {
                continue;
            };

            let event = Event::MouseScroll {
                pos,
                delta,
                modifiers: self.modifiers,
            };
            if self.emit(tree, layout, id, &mut node, &event).is_sink() {
                resp = Handled::Sink;
                break;
            }
        }

        std::mem::swap(&mut self.intersections.hit, &mut hit);
        resp
    }

    fn mouse_drag(
        &mut self,
        tree: &Tree,
        layout: &Layout,
        origin: Pos2f,
        pos: Pos2f,
        delta: Vec2f,
        button: MouseButton,
    ) -> Handled {
        let mut resp = Handled::Bubble;
        let mut hit = std::mem::take(&mut self.intersections.hit);
        for &id in &hit {
            let Some(mut node) = tree.get_mut(id) else {
                continue;
            };
            let event = Event::MouseDrag {
                origin,
                pos,
                delta,
                modifiers: self.modifiers,
                button,
            };
            if self.emit(tree, layout, id, &mut node, &event).is_sink() {
                resp = Handled::Sink;
                break;
            }
        }
        std::mem::swap(&mut self.intersections.hit, &mut hit);
        resp
    }

    fn mouse_drag_release(
        &mut self,
        tree: &Tree,
        layout: &Layout,
        origin: Pos2f,
        pos: Pos2f,
        delta: Vec2f,
        button: MouseButton,
    ) -> Handled {
        let mut resp = Handled::Bubble;
        let mut hit = std::mem::take(&mut self.intersections.hit);
        for &id in &hit {
            let Some(mut node) = tree.get_mut(id) else {
                continue;
            };

            let event = Event::MouseDragRelease {
                origin,
                pos,
                delta,
                modifiers: self.modifiers,
                button,
            };
            if self.emit(tree, layout, id, &mut node, &event) == Handled::Sink {
                resp = Handled::Sink;
                break;
            }
        }
        std::mem::swap(&mut self.intersections.hit, &mut hit);
        resp
    }

    fn notify(&mut self, tree: &Tree, layout: &Layout) {
        let mut current = self.selection;
        let last = self.last_selection;

        if current == last {
            return;
        }

        if let Some(entered) = current {
            if let Some(mut node) = tree.get_mut(entered) {
                self.emit(tree, layout, entered, &mut node, &Event::FocusGained);
            } else {
                self.selection = None;
                current = None;
            }
        }

        if let Some(exited) = last {
            if let Some(mut node) = tree.get_mut(exited) {
                self.emit(tree, layout, exited, &mut node, &Event::FocusLost);
            }
        }

        self.last_selection = current
    }

    fn emit(
        &mut self,
        tree: &Tree,
        layout: &Layout,
        id: WidgetId,
        node: &mut Node,
        event: &Event,
    ) -> Handled {
        let ctx = EventCtx {
            tree,
            layout,
            input: self,
        };

        tree.enter(id);
        let resp = node.widget.event(ctx, event);
        tree.exit(id);
        resp
    }
}
