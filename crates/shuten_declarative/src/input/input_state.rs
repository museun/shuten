use std::cell::{Cell, RefCell};

use shuten::{
    event::{Event as CoreEvent, Modifiers, MouseButton},
    geom::Vec2f,
};

use crate::{
    geom::Pos2f,
    layout::Layout,
    tree::{Node, Tree, WidgetId},
};

use super::{
    mouse::{ButtonState, Intersections, Mouse},
    Event, EventCtx, Handled, MouseEvent, TranslateKeyEvent, TranslateMouseEvent,
};

#[derive(Debug, Default, serde::Serialize)]
pub struct Input {
    mouse: RefCell<Mouse>,
    modifiers: Cell<Modifiers>,
    intersections: RefCell<Intersections>,
    selection: Cell<Option<WidgetId>>,
    last_selection: Cell<Option<WidgetId>>,
}

impl Input {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub fn selection(&self) -> Option<WidgetId> {
        self.selection.get()
    }

    pub fn set_selection(&self, id: Option<WidgetId>) {
        self.selection.set(id)
    }

    pub(crate) fn start(&self, tree: &Tree, layout: &Layout) {
        self.notify(tree, layout)
    }

    pub(crate) fn finish(&self) {
        let mut mouse = self.mouse.borrow_mut();
        for state in mouse.buttons.values_mut() {
            state.interpolate();
        }
    }

    pub(crate) fn handle(&self, tree: &Tree, layout: &Layout, event: &CoreEvent) -> Handled {
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
        &self,
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
        &self,
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
                    modifiers: self.modifiers.get(),
                };
                // TODO this swallows focused events
                return self.event(tree, layout, id, &mut node, &event);
            }
        }

        if let Some(id) = self.selection.get() {
            let Some(node) = layout.get(id) else {
                return Handled::Bubble;
            };

            if node.interest.is_focus_input() {
                let mut node = tree.get_mut(id).unwrap();
                let event = Event::KeyChanged {
                    key,
                    modifiers: self.modifiers.get(),
                };
                return self.event(tree, layout, id, &mut node, &event);
            }
        }

        Handled::Bubble
    }

    pub fn update_modifiers(&self, modifiers: Modifiers) {
        self.modifiers.set(modifiers)
    }

    pub fn mouse_moved(&self, tree: &Tree, layout: &Layout, pos: Pos2f) {
        let _ = self.mouse.borrow_mut().pos.replace(pos);
        self.send_mouse_move(tree, layout);
        self.mouse_hit_test(tree, layout);
        self.send_mouse_enter(tree, layout);
        self.send_mouse_leave(tree, layout);
    }

    fn send_mouse_move(&self, tree: &Tree, layout: &Layout) {
        let mouse = self.mouse.borrow();
        let event = Event::MouseMoved { pos: mouse.pos };
        for (id, interest) in layout.mouse.iter() {
            if interest.is_mouse_move() {
                if let Some(mut node) = tree.get_mut(id) {
                    self.event(tree, layout, id, &mut node, &event);
                }
            }
        }
    }

    fn send_mouse_enter(&self, tree: &Tree, layout: &Layout) {
        let intersections = &mut *self.intersections.borrow_mut();
        for &hit in &intersections.hit {
            if let Some(mut node) = tree.get_mut(hit) {
                if !intersections.entered.contains(&hit) {
                    intersections.entered.push(hit);
                    let resp = self.event(tree, layout, hit, &mut node, &Event::MouseEnter);
                    if resp == Handled::Sink {
                        intersections.entered_and_sunk.push(hit);
                        break;
                    }
                } else if intersections.entered_and_sunk.contains(&hit) {
                    break;
                }
            }
        }
    }

    fn send_mouse_leave(&self, tree: &Tree, layout: &Layout) {
        let intersections = &mut *self.intersections.borrow_mut();
        let mut dead = vec![];
        for &hit in &intersections.entered {
            if !intersections.hit.contains(&hit) {
                if let Some(mut node) = tree.get_mut(hit) {
                    self.event(tree, layout, hit, &mut node, &Event::MouseLeave);
                }
                dead.push(hit);
            }
        }

        for dead in dead {
            intersections.entered.retain(|&id| id != dead);
            intersections.entered_and_sunk.retain(|&id| id != dead);
        }
    }

    fn mouse_hit_test(&self, tree: &Tree, layout: &Layout) {
        let mut intersections = self.intersections.borrow_mut();
        let mouse = self.mouse.borrow();
        intersections.hit.clear();
        if let Some(pos) = mouse.pos {
            Self::hit_test(tree, layout, pos, &mut intersections.hit)
        }
    }

    fn hit_test(_tree: &Tree, layout: &Layout, pos: Pos2f, mouse_hit: &mut Vec<WidgetId>) {
        for (id, _) in layout.mouse.iter() {
            let Some(mut node) = layout.get(id) else {
                continue;
            };

            let mut rect = node.rect;
            while let Some(parent) = node.clipped_by {
                node = layout.get(parent).unwrap();
                rect = rect.constrain(node.rect);
            }

            if node.rect.contains(pos) {
                mouse_hit.push(id);
            }
        }
    }

    fn mouse_button_changed(
        &self,
        tree: &Tree,
        layout: &Layout,
        button: MouseButton,
        down: bool,
    ) -> Handled {
        {
            let mut mouse = self.mouse.borrow_mut();
            let state = mouse.buttons.entry(button).or_insert(ButtonState::Released);
            match (state.is_down(), down) {
                (true, true) | (false, false) => {}
                (false, true) => *state = ButtonState::Down,
                (true, false) => *state = ButtonState::Up,
            }
        }
        self.button_change(tree, layout, button, down)
    }

    fn button_change(
        &self,
        tree: &Tree,
        layout: &Layout,
        button: MouseButton,
        down: bool,
    ) -> Handled {
        let mouse = self.mouse.borrow();
        let intersections = self.intersections.borrow();
        let mut resp = Handled::Bubble;

        for &id in &intersections.hit {
            if let Some(mut node) = tree.get_mut(id) {
                let event = if down {
                    Event::MouseHeld {
                        button,
                        inside: true,
                        pos: mouse.pos.unwrap(),
                        modifiers: self.modifiers.get(),
                    }
                } else {
                    Event::MouseRelease {
                        button,
                        inside: true,
                        pos: mouse.pos.unwrap(),
                        modifiers: self.modifiers.get(),
                    }
                };
                if self.event(tree, layout, id, &mut node, &event) == Handled::Sink {
                    resp = Handled::Sink;
                    break;
                }
            }
        }

        for (id, interest) in layout.mouse.iter() {
            if interest.is_mouse_outside() && intersections.hit.contains(&id) {
                if let Some(mut node) = tree.get_mut(id) {
                    let event = if down {
                        Event::MouseHeld {
                            button,
                            inside: false,
                            pos: mouse.pos.unwrap(),
                            modifiers: self.modifiers.get(),
                        }
                    } else {
                        Event::MouseRelease {
                            button,
                            inside: false,
                            pos: mouse.pos.unwrap(),
                            modifiers: self.modifiers.get(),
                        }
                    };

                    self.event(tree, layout, id, &mut node, &event);
                }
            }
        }

        resp
    }

    fn mouse_scroll(&self, tree: &Tree, layout: &Layout, pos: Pos2f, delta: Vec2f) -> Handled {
        let intersections = self.intersections.borrow();
        let mut resp = Handled::Bubble;
        for &id in &intersections.hit {
            if let Some(mut node) = tree.get_mut(id) {
                let event = Event::MouseScroll {
                    pos,
                    delta,
                    modifiers: self.modifiers.get(),
                };

                if self.event(tree, layout, id, &mut node, &event) == Handled::Sink {
                    resp = Handled::Sink;
                    break;
                }
            }
        }
        resp
    }

    fn mouse_drag(
        &self,
        tree: &Tree,
        layout: &Layout,
        origin: Pos2f,
        pos: Pos2f,
        delta: Vec2f,
        button: MouseButton,
    ) -> Handled {
        let intersections = self.intersections.borrow();
        let mut resp = Handled::Bubble;
        for &id in &intersections.hit {
            if let Some(mut node) = tree.get_mut(id) {
                let event = Event::MouseDrag {
                    origin,
                    pos,
                    delta,
                    modifiers: self.modifiers.get(),
                    button,
                };
                if self.event(tree, layout, id, &mut node, &event) == Handled::Sink {
                    resp = Handled::Sink;
                    break;
                }
            }
        }
        resp
    }

    fn mouse_drag_release(
        &self,
        tree: &Tree,
        layout: &Layout,
        origin: Pos2f,
        pos: Pos2f,
        delta: Vec2f,
        button: MouseButton,
    ) -> Handled {
        let intersections = self.intersections.borrow();
        let mut resp = Handled::Bubble;
        for &id in &intersections.hit {
            if let Some(mut node) = tree.get_mut(id) {
                let event = Event::MouseDragRelease {
                    origin,
                    pos,
                    delta,
                    modifiers: self.modifiers.get(),
                    button,
                };

                if self.event(tree, layout, id, &mut node, &event) == Handled::Sink {
                    resp = Handled::Sink;
                    break;
                }
            }
        }
        resp
    }

    fn notify(&self, tree: &Tree, layout: &Layout) {
        let mut current = self.selection.get();
        let last = self.last_selection.get();

        if current == last {
            return;
        }

        if let Some(entered) = current {
            if let Some(mut node) = tree.get_mut(entered) {
                self.event(tree, layout, entered, &mut node, &Event::FocusGained);
            } else {
                self.selection.set(None);
                current = None;
            }
        }

        if let Some(exited) = last {
            if let Some(mut node) = tree.get_mut(exited) {
                self.event(tree, layout, exited, &mut node, &Event::FocusLost);
            }
        }

        self.last_selection.set(current)
    }

    fn event(
        &self,
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
