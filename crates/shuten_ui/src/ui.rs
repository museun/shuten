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
    rc::Rc,
};

use crate::{
    debug_fmt,
    input::{Handled, Input},
    node::LayoutNode,
    widget::ErasedWidget,
    widgets::{list::List, *},
    Node, Widget, WidgetId,
};

mod layout;
pub use layout::{Layout, LayoutCtx};

mod nodes;
pub use nodes::Nodes;

pub(crate) mod paint;
pub use paint::PaintCtx;

mod context;

mod response;
pub use response::Response;

mod placeholder_widget;
use placeholder_widget::Placeholder;

mod root_widget;
use root_widget::Root;

pub struct Ui {
    inner: Rc<Inner>,
}

impl std::fmt::Debug for Ui {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let this = &self.inner;

        f.debug_struct("Ui")
            .field("nodes", &debug_fmt::slot_map(&this.nodes.borrow()))
            .field(
                "computed",
                &debug_fmt::secondary_map(&this.computed.borrow()),
            )
            .field("input", &this.input.borrow())
            .field("rect", &this.rect)
            .finish()
    }
}

impl Ui {
    pub fn panel<R>(&self, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self.flex(1, |ui| {
            ui.filled(0x111111, |ui| ui.border(BorderStyle::THIN, show));
        })
    }

    pub fn border<R>(&self, style: BorderStyle, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self::border(self, style, show)
    }

    pub fn list<R>(&self, list: List, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self::list(self, list, show)
    }

    pub fn horizontal<R>(&self, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self::row(self, show)
    }

    pub fn vertical<R>(&self, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self::column(self, show)
    }

    pub fn center<R>(&self, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self.align(Align2::CENTER_CENTER, show)
    }

    pub fn align<R>(&self, align: Align2, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self::align(self, align, show)
    }

    pub fn filled<R>(&self, bg: impl Into<Rgb>, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self::filled(self, bg, show)
    }

    pub fn margin<R>(&self, margin: Margin, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self::margin(self, margin, show)
    }

    pub fn expanded<R>(&self, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self::expanded(self, show)
    }

    pub fn flex<R>(&self, factor: u16, show: impl FnOnce(&Ui) -> R) -> Response<()> {
        self::flex(self, factor, show)
    }
}

impl Ui {
    pub fn spacer(&self) -> Response {
        self::spacer(self)
    }

    pub fn label<T>(&self, label: impl Into<Styled<T>>) -> Response
    where
        T: Label,
    {
        self::label(self, label)
    }

    pub fn rect(&self, bg: impl Into<Rgb>, min_size: Vec2f) -> Response {
        self::filled_rect(self, bg, min_size)
    }

    // pub fn progress<T>(&self) {}

    pub fn slider<T>(&self, _value: &mut T, _range: RangeInclusive<T>) -> Response<()>
    where
        T: PartialEq<T> + Num,
    {
        todo!();
    }

    pub fn state<T>(&self, state: impl FnOnce() -> T + 'static) -> Response<StateResponse<T>>
    where
        T: Stateful,
    {
        self::state(self, state)
    }
}

impl Ui {
    pub fn instance() -> Self {
        context::current()
    }

    pub fn root(&self) -> WidgetId {
        self.inner.root
    }

    pub fn size(&self) -> Vec2f {
        self.inner.rect.get().size()
    }

    pub fn blend(&self) -> f32 {
        self.inner.last_blend.get()
    }

    pub fn current(&self) -> WidgetId {
        self.inner
            .stack
            .borrow()
            .last()
            .copied()
            .unwrap_or(self.root())
    }

    pub fn get(&self, id: WidgetId) -> Option<Ref<'_, Node>> {
        let nodes = self.inner.nodes.borrow();
        Ref::filter_map(nodes, |nodes| nodes.get(id)).ok()
    }

    pub fn get_mut(&self, id: WidgetId) -> Option<RefMut<'_, Node>> {
        let nodes = self.inner.nodes.borrow_mut();
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
    pub(crate) fn new(rect: impl Into<Rectf>) -> Self {
        let mut nodes = SlotMap::with_key();
        let inner = Inner {
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
            rect: Cell::new(rect.into()),

            last_blend: Cell::default(),
        };

        Self {
            inner: Rc::new(inner),
        }
    }

    pub(self) fn clone(&self) -> Self {
        Self {
            inner: Rc::clone(&self.inner),
        }
    }

    pub(crate) fn handle_event(&self, event: &Event) -> bool {
        if let &Event::Invalidate(rect) = event {
            self.inner.rect.set(rect.into());
            return true;
        }

        let last_blend = if let &Event::Blend(blend) = event {
            blend
        } else {
            0.0
        };
        self.inner.last_blend.set(last_blend);

        self.inner.input.borrow_mut().handle(
            event,
            &mut self.inner.nodes.borrow_mut(),
            &mut self.inner.computed.borrow_mut(),
            &mut self.inner.stack.borrow_mut(),
        ) == Handled::Sink
    }

    pub(crate) fn begin_widget<W: Widget>(&self, props: W::Props<'_>) -> Response<W::Response> {
        let parent = self.current();
        let (id, mut widget) = self.update_widget::<W>(parent);

        self.inner.stack.borrow_mut().push(id);
        let resp = {
            let Some(widget) = widget.as_any_mut().downcast_mut::<W>() else {
                unreachable!("expected to get: {}", widget.type_name())
            };
            widget.update(props)
        };

        self.inner.nodes.borrow_mut()[id].widget = widget;
        Response::new(id, resp)
    }

    pub(crate) fn end_widget(&self, id: WidgetId) {
        let Some(old) = self.inner.stack.borrow_mut().pop() else {
            unreachable!("called end widget without an active widget")
        };
        assert_eq!(id, old, "end widget did not match input widget");

        Self::cleanup(
            &mut self.inner.nodes.borrow_mut(),
            &mut self.inner.removed.borrow_mut(),
            id,
        );
    }

    fn update_widget<W: Widget>(&self, parent: WidgetId) -> (WidgetId, Box<dyn ErasedWidget>) {
        let mut nodes = self.inner.nodes.borrow_mut();

        let Some(id) = Self::append_widget(&mut nodes, parent) else {
            let (id, widget) = Self::allocate_widget::<W>(&mut nodes, parent);
            return (id, widget);
        };

        let Some(node) = nodes.get_mut(id) else {
            unreachable!("node {id:?} must exist")
        };

        let widget = std::mem::replace(&mut node.widget, Box::new(Placeholder));
        if widget.as_ref().type_id() != TypeId::of::<W>() {
            Self::remove_widget(&mut nodes, &mut self.inner.removed.borrow_mut(), id);
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
    pub(crate) fn scope<R>(&self, f: impl FnOnce(&Self) -> R) -> R {
        self.begin();
        let resp = f(self);
        self.end();
        resp
    }

    pub(crate) fn begin(&self) {
        context::bind(self);
        self.inner.nodes.borrow_mut()[self.root()].next = 0;
        self.inner.input.borrow_mut().start();
    }

    pub(crate) fn end(&self) {
        self.inner.removed.borrow_mut().clear();
        Self::cleanup(
            &mut self.inner.nodes.borrow_mut(),   //
            &mut self.inner.removed.borrow_mut(), //
            self.inner.root,
        );
        let mut input = self.inner.input.borrow_mut();
        input.end();

        let (mut mouse, mut keyboard) =
            RefMut::map_split(input, |input| (&mut input.mouse, &mut input.keyboard));

        let _ = Layout {
            nodes: &mut self.inner.nodes.borrow_mut(),
            computed: &mut self.inner.computed.borrow_mut(),
            stack: &mut self.inner.stack.borrow_mut(),
            mouse: &mut mouse,
            keyboard: &mut keyboard,
        }
        .compute(
            self.root(),
            Constraints::tight(self.inner.rect.get().size()),
        );

        self.resolve();

        context::unbind();
    }

    fn resolve(&self) {
        let mut queue = VecDeque::new();
        queue.push_back((self.root(), Pos2f::ZERO));

        let nodes = self.inner.nodes.borrow();
        let mut computed = self.inner.computed.borrow_mut();
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
        self.paint_node(self.root(), &mut canvas)
    }

    fn paint_node(&mut self, id: WidgetId, canvas: &mut Canvas<'_>) {
        paint::Paint.paint(self, canvas, id);
    }
}

struct Inner {
    nodes: RefCell<SlotMap<WidgetId, Node>>,
    computed: RefCell<SecondaryMap<WidgetId, LayoutNode>>,
    input: RefCell<Input>,
    root: WidgetId,
    stack: RefCell<Vec<WidgetId>>,
    removed: RefCell<Vec<WidgetId>>,
    rect: Cell<Rectf>,
    last_blend: Cell<f32>,
}
