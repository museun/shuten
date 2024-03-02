use shuten::{
    event::Event,
    geom::{Align2, Constraints, Margin, Num, Pos2f, Rectf, Vec2f},
    style::Rgb,
    Canvas,
};
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::TypeId,
    cell::{Cell, Ref, RefCell, RefMut},
    collections::VecDeque,
    ops::RangeInclusive,
};

use crate::{
    debug_fmt,
    input::{Handled, Input, Keyboard, Mouse},
    node::LayoutNode,
    widget::ErasedWidget,
    widgets::{
        filled::Filled, AlignWidget, BorderStyle, BorderWidget, FilledWidget, Flex, FlexWidget,
        Label, LabelWidget, List, ListWidget, MarginWidget, State, StateResponse, StateWidget,
        Stateful, Styled,
    },
    Node, Widget, WidgetExt, WidgetId,
};

mod response;
pub use response::Response;

mod placeholder_widget;
use placeholder_widget::Placeholder;

mod root_widget;
use root_widget::Root;

pub struct Ui {
    nodes: RefCell<SlotMap<WidgetId, Node>>,
    computed: RefCell<SecondaryMap<WidgetId, LayoutNode>>,
    input: RefCell<Input>,
    root: WidgetId,
    stack: RefCell<Vec<WidgetId>>,
    removed: RefCell<Vec<WidgetId>>,
    rect: Rectf,
    last_blend: f32,
}

impl std::fmt::Debug for Ui {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Ui")
            .field("nodes", &debug_fmt::slot_map(&self.nodes.borrow()))
            .field(
                "computed",
                &debug_fmt::secondary_map(&self.computed.borrow()),
            )
            .field("input", &self.input.borrow())
            .field("rect", &self.rect)
            .finish()
    }
}

impl Ui {
    pub(crate) fn new(rect: impl Into<Rectf>) -> Self {
        let mut nodes = SlotMap::with_key();
        Self {
            root: nodes.insert(Node {
                widget: Box::new(Root),
                parent: None,
                children: Vec::new(),
                next: 0,
            }),
            nodes: RefCell::new(nodes),
            computed: RefCell::default(),
            input: RefCell::default(),

            stack: RefCell::default(),
            removed: RefCell::default(),
            rect: rect.into(),

            last_blend: 0.0,
        }
    }
}

impl Ui {
    pub const fn root(&self) -> WidgetId {
        self.root
    }

    pub fn size(&self) -> Vec2f {
        self.rect.size()
    }

    pub const fn blend(&self) -> f32 {
        self.last_blend
    }

    pub fn current(&self) -> WidgetId {
        self.stack.borrow().last().copied().unwrap_or(self.root)
    }

    pub fn get(&self, id: WidgetId) -> Option<Ref<'_, Node>> {
        let nodes = self.nodes.borrow();
        Ref::filter_map(nodes, |nodes| nodes.get(id)).ok()
    }

    pub fn get_mut(&self, id: WidgetId) -> Option<RefMut<'_, Node>> {
        let nodes = self.nodes.borrow_mut();
        RefMut::filter_map(nodes, |nodes| nodes.get_mut(id)).ok()
    }

    pub fn get_current(&self) -> Ref<'_, Node> {
        self.get(self.current()).unwrap()
    }

    pub fn widget<W: Widget>(&self, props: W::Props<'_>) -> Response<W::Response> {
        let resp = self.begin_widget::<W>(props);
        self.end_widget(resp.id());
        resp
    }
}

impl Ui {
    pub(crate) fn handle_event(&mut self, event: &Event) -> bool {
        if let &Event::Invalidate(rect) = event {
            self.rect = rect.into();
            return true;
        }

        self.last_blend = if let &Event::Blend(blend) = event {
            blend
        } else {
            0.0
        };

        self.input.get_mut().handle(
            event,
            self.nodes.get_mut(),
            self.computed.get_mut(),
            self.stack.get_mut(),
        ) == Handled::Sink
    }

    pub(crate) fn begin_widget<W: Widget>(&self, props: W::Props<'_>) -> Response<W::Response> {
        let parent = self.current();
        let (id, mut widget) = self.update_widget::<W>(parent);

        self.stack.borrow_mut().push(id);
        let resp = {
            let Some(widget) = widget.as_any_mut().downcast_mut::<W>() else {
                unreachable!("expected to get: {}", widget.type_name())
            };
            widget.update(self, props)
        };

        self.nodes.borrow_mut()[id].widget = widget;
        Response::new(id, resp)
    }

    pub(crate) fn end_widget(&self, id: WidgetId) {
        let Some(old) = self.stack.borrow_mut().pop() else {
            unreachable!("called end widget without an active widget")
        };
        assert_eq!(id, old, "end widget did not match input widget");

        Self::cleanup(
            &mut self.nodes.borrow_mut(),
            &mut self.removed.borrow_mut(),
            id,
        );
    }

    fn update_widget<W: Widget>(&self, parent: WidgetId) -> (WidgetId, Box<dyn ErasedWidget>) {
        let mut nodes = self.nodes.borrow_mut();

        let Some(id) = Self::append_widget(&mut nodes, parent) else {
            let (id, widget) = Self::allocate_widget::<W>(&mut nodes, parent);
            return (id, widget);
        };

        let Some(node) = nodes.get_mut(id) else {
            unreachable!("node {id:?} must exist")
        };

        let widget = std::mem::replace(&mut node.widget, Box::new(Placeholder));
        if widget.as_ref().type_id() != TypeId::of::<W>() {
            Self::remove_widget(&mut nodes, &mut self.removed.borrow_mut(), id);
            return Self::allocate_widget::<W>(&mut nodes, parent);
        }

        node.next = 0;
        (id, widget)
    }

    fn append_widget(nodes: &mut SlotMap<WidgetId, Node>, id: WidgetId) -> Option<WidgetId> {
        let parent = &mut nodes[id];
        let &id = parent.children.get(parent.next)?;
        parent.next += 1;
        Some(id)
    }

    fn allocate_widget<W: Widget>(
        nodes: &mut SlotMap<WidgetId, Node>,
        parent: WidgetId,
    ) -> (WidgetId, Box<dyn ErasedWidget>) {
        let id = nodes.insert(Node {
            widget: Box::new(Placeholder),
            parent: Some(parent),
            children: Vec::new(),
            next: 0,
        });

        let parent = &mut nodes[parent];
        if parent.next < parent.children.len() {
            parent.children[parent.next] = id;
        } else {
            parent.children.push(id)
        }
        parent.next += 1;
        (id, <Box<W>>::default() as Box<dyn ErasedWidget>)
    }

    fn remove_widget(
        nodes: &mut SlotMap<WidgetId, Node>,
        removed: &mut Vec<WidgetId>,
        id: WidgetId,
    ) {
        let mut queue = VecDeque::from_iter([id]);
        while let Some(id) = queue.pop_front() {
            removed.push(id);

            if let Some(node) = nodes.remove(id) {
                queue.extend(node.children());
                if let Some(parent) = node.parent {
                    nodes[parent].children.retain(|&child| child != id);
                }
            }
        }
    }

    fn cleanup(nodes: &mut SlotMap<WidgetId, Node>, removed: &mut Vec<WidgetId>, start: WidgetId) {
        let node = &mut nodes[start];
        if node.next >= node.children.len() {
            return;
        }

        let children = &node.children[node.next..];
        let mut queue = VecDeque::from_iter(children.iter().copied());
        removed.extend_from_slice(children);
        node.children.truncate(node.next);

        while let Some(id) = queue.pop_front() {
            removed.push(id);
            let Some(next) = nodes.remove(id) else {
                unreachable!("child: {id:?} should exist for {start:?}")
            };
            queue.extend(next.children())
        }
    }
}

impl Ui {
    pub(crate) fn scope<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        self.begin();
        let resp = f(self);
        self.end();
        resp
    }

    pub(crate) fn begin(&mut self) {
        self.nodes.get_mut()[self.root].next = 0;
        self.input.get_mut().start();
    }

    pub(crate) fn end(&mut self) {
        self.removed.get_mut().clear();
        Self::cleanup(
            self.nodes.get_mut(),   //
            self.removed.get_mut(), //
            self.root,
        );
        let input = self.input.get_mut();
        input.end();

        let _ = Layout {
            nodes: self.nodes.get_mut(),
            computed: self.computed.get_mut(),
            stack: self.stack.get_mut(),
            mouse: &mut input.mouse,
            keyboard: &mut input.keyboard,
        }
        .compute(self.root, Constraints::tight(self.rect.size()));

        self.resolve();
    }

    fn resolve(&mut self) {
        let mut queue = VecDeque::new();
        queue.push_back((self.root, Pos2f::ZERO));

        let nodes = self.nodes.get_mut();
        let computed = self.computed.get_mut();
        while let Some((id, pos)) = queue.pop_front() {
            let Some(node) = computed.get_mut(id) else {
                continue;
            };

            node.rect.set_pos(node.rect.min + pos);
            let rect = node.rect;

            queue.extend(nodes[id].children().iter().map(|&id| (id, rect.min)));
        }
    }

    pub(crate) fn paint(&mut self, mut canvas: Canvas) {
        self.paint_node(self.root, &mut canvas)
    }

    fn paint_node(&mut self, id: WidgetId, canvas: &mut Canvas<'_>) {
        Paint.paint(self, canvas, id);
    }
}

impl Ui {
    // TODO border
    pub fn panel<R>(&self, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self.flex(1, |ui| {
            ui.filled(0x111111, |ui| ui.border(BorderStyle::THIN, show));
        })
    }

    pub fn border<R>(&self, style: BorderStyle, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        BorderWidget::show_children(self, style, show)
    }

    pub fn list<R>(&self, list: List, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        ListWidget::show_children(self, list, show)
    }

    pub fn horizontal<R>(&self, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        ListWidget::show_children(self, List::row(), show)
    }

    pub fn vertical<R>(&self, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        ListWidget::show_children(self, List::column(), show)
    }

    pub fn center<R>(&self, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self.align(Align2::CENTER_CENTER, show)
    }

    pub fn align<R>(&self, align: Align2, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        AlignWidget::show_children(self, align, show)
    }

    pub fn filled<R>(&self, bg: impl Into<Rgb>, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        FilledWidget::show_children(self, Filled::bg(bg), show)
    }

    pub fn margin<R>(&self, margin: Margin, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        MarginWidget::show_children(self, margin, show)
    }

    pub fn expanded<R>(&self, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        FlexWidget::show_children(self, Flex::expanded(), show)
    }

    pub fn flex<R>(&self, factor: u16, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        FlexWidget::show_children(self, Flex::new(factor), show)
    }
}

impl Ui {
    pub fn spacer(&self) {
        FlexWidget::show(self, Flex::expanded());
    }

    pub fn label<T: Label>(&self, label: impl Into<Styled<T>>) -> Response {
        LabelWidget::show(self, label.into().into_static())
    }

    pub fn rect(&self, bg: impl Into<Rgb>, min_size: Vec2f) -> Response {
        FilledWidget::show_children(self, Filled::bg(bg).min_size(min_size), |ui| {})
    }

    pub fn progress<T>(&self) -> () {}

    pub fn slider<T>(&self, value: &mut T, range: RangeInclusive<T>) -> Response<()>
    where
        T: PartialEq<T> + Num,
    {
        todo!();
    }
}

impl Ui {
    pub fn state<T: Stateful>(
        &self,
        state: impl FnOnce() -> T + 'static,
    ) -> Response<StateResponse<T>> {
        StateWidget::show(self, State::new(state))
    }
}

#[derive(Copy, Clone)]
pub struct Nodes<'a> {
    nodes: &'a SlotMap<WidgetId, Node>,
    current: WidgetId,
}

impl<'a> Nodes<'a> {
    pub fn children(&self) -> &[WidgetId] {
        self.current().children()
    }

    pub fn current(&self) -> &Node {
        &self.nodes[self.current]
    }

    pub fn current_id(&self) -> WidgetId {
        self.current
    }

    pub fn get(&self, id: WidgetId) -> Option<&Node> {
        self.nodes.get(id)
    }
}

impl<'a> std::ops::Index<WidgetId> for Nodes<'a> {
    type Output = Node;
    fn index(&self, index: WidgetId) -> &Self::Output {
        &self.nodes[index]
    }
}

pub struct Layout<'a> {
    nodes: &'a SlotMap<WidgetId, Node>,
    computed: &'a mut SecondaryMap<WidgetId, LayoutNode>,
    stack: &'a mut Vec<WidgetId>,
    mouse: &'a mut Mouse,
    keyboard: &'a mut Keyboard,
}

impl<'a> Layout<'a> {
    pub fn set_pos(&mut self, child: WidgetId, pos: Pos2f) {
        let Some(node) = self.computed.get_mut(child) else {
            return;
        };
        node.rect.set_pos(pos)
    }

    pub fn new_layer(&mut self, id: WidgetId) {
        self.mouse.push_layer(id);
        self.keyboard.push_layer(id);
    }

    pub(crate) fn get(&self, id: WidgetId) -> Option<&LayoutNode> {
        self.computed.get(id)
    }

    pub fn compute(&mut self, child: WidgetId, input: Constraints) -> Vec2f {
        let Some(node) = self.nodes.get(child) else {
            return Vec2f::ZERO;
        };

        self.stack.push(child);

        let interest = node.widget.interest();
        if interest.is_mouse_any() {
            self.mouse.layered.insert(child, interest);
        }
        if interest.is_key_input() {
            self.keyboard.layered.insert(child, ());
        }

        // TODO: update the interaction layers here

        let widget = &node.widget;
        let size = {
            let ctx = LayoutCtx {
                nodes: Nodes {
                    nodes: self.nodes,
                    current: child,
                },
                layout: self,
            };
            widget.layout(ctx, input)
        };

        self.computed.insert(
            child,
            LayoutNode {
                rect: Rectf::from_min_size(Pos2f::ZERO, size),
                interest: widget.interest(),
            },
        );

        assert_eq!(self.stack.pop(), Some(child));

        size
    }
}

impl<'a> std::ops::Index<WidgetId> for Layout<'a> {
    type Output = LayoutNode;
    fn index(&self, index: WidgetId) -> &Self::Output {
        &self.computed[index]
    }
}

impl<'a> std::ops::IndexMut<WidgetId> for Layout<'a> {
    fn index_mut(&mut self, index: WidgetId) -> &mut Self::Output {
        &mut self.computed[index]
    }
}

pub struct LayoutCtx<'a: 'b, 'b> {
    pub nodes: Nodes<'a>,
    pub layout: &'b mut Layout<'a>,
}

pub struct Paint;

impl Paint {
    pub fn paint_all(&mut self, ui: &Ui, canvas: &mut Canvas<'_>) {
        self.paint(ui, canvas, ui.root)
    }

    fn paint(&mut self, ui: &Ui, canvas: &mut Canvas<'_>, id: WidgetId) {
        let computed = ui.computed.borrow();
        let Some(node) = computed.get(id) else {
            return;
        };
        let rect = node.rect;

        let nodes = ui.nodes.borrow();
        let node = &nodes[id];
        ui.stack.borrow_mut().push(id);
        node.widget.paint(PaintCtx {
            rect,
            ui,
            canvas: &mut canvas.crop(rect.into()),
            paint: self,
        });
        assert_eq!(Some(id), ui.stack.borrow_mut().pop());
    }
}

pub struct PaintCtx<'a: 'c, 'c> {
    pub rect: Rectf,
    pub ui: &'a Ui,
    pub canvas: &'a mut Canvas<'c>,
    pub(crate) paint: &'a mut Paint,
}

impl<'a: 'c, 'c> PaintCtx<'a, 'c> {
    pub fn paint(&mut self, id: WidgetId) {
        self.paint.paint(self.ui, self.canvas, id);
    }
}

// mod local {
//     use std::cell::RefCell;

//     use crate::Ui;
//     thread_local! {static CONTEXT: RefCell<Option<Ui>> = const { RefCell::new(None) } }

//     pub fn bind(ui: &Ui) {
//         CONTEXT.with(move |current| {
//             let mut current = current.borrow_mut();
//             assert!(
//                 current.is_none(),
//                 "cannot bind a ui with it already being bound"
//             );
//             *current = Some(ui.clone());
//         })
//     }

//     pub fn unbind() {
//         CONTEXT.with(|current| {
//             let mut current = current.borrow_mut();
//             assert!(
//                 current.take().is_some(),
//                 "cannot unbind ui without it being bound"
//             )
//         })
//     }

//     pub fn current() -> Ui {
//         CONTEXT.with(|current| {
//             current
//                 .borrow()
//                 .as_ref()
//                 .expect("cannot get ui without one being bound")
//                 .clone()
//         })
//     }
// }

#[cfg(test)]
mod tests {
    use crate::widgets::*;
    use shuten::{geom::*, style::*};

    use super::*;
    #[test]
    fn asdf() {
        let mut ui = Ui::new(rect(vec2(80, 25)));
        ui.scope(|ui| {
            ui.center(|ui| {
                SizedWidget::show_children(ui, Sized::max(vec2f(f32::INFINITY, 3.0)), |ui| {
                    SliderWidget::show(ui, Slider::new(0.5, 0.0..=1.0));
                });
            });
        });

        eprintln!("{ui:#?}");
    }
}
