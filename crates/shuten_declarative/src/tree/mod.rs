use std::{
    any::TypeId,
    cell::{Ref, RefCell, RefMut},
    collections::VecDeque,
    rc::Rc,
};

use crate::widget::{DummyWidget, ErasedWidget, Response, RootWidget, Widget};

mod context;
pub use context::current_tree;
pub(crate) use context::{bind, unbind};

mod widget_id;
use slotmap::SlotMap;
pub use widget_id::WidgetId;

mod node;
pub use node::Node;

pub struct Tree {
    inner: Rc<TreeInner>,
}

impl serde::Serialize for Tree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        use crate::external::serialize_slot_map::SlotMap;
        let mut ser = serializer.serialize_struct("Tree", 4)?;
        ser.serialize_field("nodes", &SlotMap(&*self.inner.nodes.borrow()))?;
        ser.serialize_field("stack", &*self.inner.stack.borrow())?;
        ser.serialize_field("removed", &*self.inner.removed.borrow())?;
        ser.serialize_field("root", &self.inner.root)?;
        ser.end()
    }
}

impl std::fmt::Debug for Tree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tree")
            .field(
                "nodes",
                &crate::external::SlotMapPrinter(&*self.inner.nodes.borrow()),
            )
            .field("stack", &self.inner.stack)
            .field("removed", &self.inner.removed)
            .field("root", &self.inner.root)
            .finish()
    }
}

impl Tree {
    pub(crate) fn new() -> Self {
        Self {
            inner: Rc::new(TreeInner::new()),
        }
    }

    pub(crate) fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }

    pub fn root(&self) -> WidgetId {
        self.inner.root
    }

    pub fn current(&self) -> WidgetId {
        self.inner
            .stack
            .borrow()
            .last()
            .copied()
            .unwrap_or(self.inner.root)
    }

    pub fn get_current(&self) -> Ref<'_, Node> {
        let nodes = self.inner.nodes.borrow();
        let current = self.current();
        Ref::map(nodes, |nodes| &nodes[current])
    }

    pub fn get(&self, id: WidgetId) -> Option<Ref<'_, Node>> {
        let nodes = self.inner.nodes.borrow();
        Ref::filter_map(nodes, |nodes| nodes.get(id)).ok()
    }

    pub fn get_mut(&self, id: WidgetId) -> Option<RefMut<'_, Node>> {
        let nodes = self.inner.nodes.borrow_mut();
        RefMut::filter_map(nodes, |nodes| nodes.get_mut(id)).ok()
    }

    pub fn do_widget<T: Widget>(&self, props: T::Props<'_>) -> Response<T::Response> {
        let resp = self.begin_widget::<T>(props);
        self.end_widget(resp.id());
        resp
    }
}

impl Tree {
    pub(crate) fn start(&self) {
        let mut nodes = self.inner.nodes.borrow_mut();
        let root = &mut nodes[self.inner.root];
        root.next = 0;
    }

    pub(crate) fn finish(&self) {
        let mut nodes = self.inner.nodes.borrow_mut();
        let mut removed = self.inner.removed.borrow_mut();
        removed.clear();
        Self::trim_children(&mut nodes, &mut removed, self.inner.root)
    }

    pub(crate) fn enter(&self, id: WidgetId) {
        self.inner.stack.borrow_mut().push(id);
    }

    pub(crate) fn exit(&self, id: WidgetId) {
        assert_eq!(self.inner.stack.borrow_mut().pop(), Some(id));
    }

    pub(crate) fn removed(&self) -> Ref<'_, [WidgetId]> {
        Ref::map(self.inner.removed.borrow(), AsRef::as_ref)
    }

    pub(crate) fn begin_widget<T: Widget>(&self, props: T::Props<'_>) -> Response<T::Response> {
        let parent = self.current();

        let (id, mut widget) = {
            let mut nodes = self.inner.nodes.borrow_mut();

            if let Some(id) = Self::next_existing_widget(&mut nodes, parent) {
                let node = nodes.get_mut(id).unwrap();
                let widget = std::mem::replace(&mut node.widget, Box::new(DummyWidget));
                if widget.as_ref().type_id() == TypeId::of::<T>() {
                    node.next = 0;
                    (id, widget)
                } else {
                    let mut removed = self.inner.removed.borrow_mut();
                    Self::remove(&mut nodes, &mut removed, id);
                    Self::new_widget::<T>(&mut nodes, parent)
                }
            } else {
                Self::new_widget::<T>(&mut nodes, parent)
            }
        };

        self.inner.stack.borrow_mut().push(id);

        let resp = match widget.as_any_mut().downcast_mut::<T>() {
            Some(widget) => widget.update(props),
            _ => unreachable!("expected to get {}", widget.type_name()),
        };

        self.inner.nodes.borrow_mut().get_mut(id).unwrap().widget = widget;

        Response::new(id, resp)
    }

    pub(crate) fn end_widget(&self, id: WidgetId) {
        let Some(old) = self.inner.stack.borrow_mut().pop() else {
            unreachable!("called end_widget without an active widget")
        };
        assert_eq!(id, old, "end widget did not match input widget");

        let mut nodes = self.inner.nodes.borrow_mut();
        let mut removed = self.inner.removed.borrow_mut();
        Self::trim_children(&mut nodes, &mut removed, id)
    }
}

impl Tree {
    fn next_existing_widget(
        nodes: &mut SlotMap<WidgetId, Node>,
        pid: WidgetId,
    ) -> Option<WidgetId> {
        let parent = nodes.get_mut(pid).unwrap();
        let &id = parent.children.get(parent.next)?;
        parent.next += 1;
        Some(id)
    }

    fn new_widget<T: Widget>(
        nodes: &mut SlotMap<WidgetId, Node>,
        pid: WidgetId,
    ) -> (WidgetId, Box<dyn ErasedWidget>) {
        let id = nodes.insert(Node {
            widget: Box::new(DummyWidget),
            parent: Some(pid),
            children: Vec::new(),
            next: 0,
        });
        let parent = nodes.get_mut(pid).unwrap();
        if parent.next < parent.children.len() {
            parent.children[parent.next] = id;
        } else {
            parent.children.push(id)
        }
        parent.next += 1;

        #[allow(clippy::box_default)]
        (id, Box::new(T::default()) as Box<dyn ErasedWidget>)
    }

    fn trim_children(
        nodes: &mut SlotMap<WidgetId, Node>,
        removed: &mut Vec<WidgetId>,
        id: WidgetId,
    ) {
        let node = &mut nodes[id];
        if node.next >= node.children.len() {
            return;
        }

        let temp = &node.children[node.next..];
        let mut queue: VecDeque<_> = temp.iter().copied().collect::<_>();
        removed.extend_from_slice(temp);

        node.children.truncate(node.next);
        while let Some(cid) = queue.pop_front() {
            removed.push(cid);
            queue.extend(nodes.remove(cid).unwrap().children);
        }
    }

    fn remove(nodes: &mut SlotMap<WidgetId, Node>, removed: &mut Vec<WidgetId>, id: WidgetId) {
        let mut queue = VecDeque::new();
        queue.push_back(id);

        while let Some(id) = queue.pop_front() {
            removed.push(id);
            let Some(node) = nodes.get(id) else {
                continue;
            };
            let children = node.children.as_slice();
            queue.extend(children);
            nodes.remove(id);
        }
    }
}

struct TreeInner {
    nodes: RefCell<SlotMap<WidgetId, Node>>,
    stack: RefCell<Vec<WidgetId>>,
    removed: RefCell<Vec<WidgetId>>,
    root: WidgetId,
}

impl TreeInner {
    fn new() -> Self {
        let mut nodes = SlotMap::with_key();

        let root = nodes.insert(Node {
            widget: Box::new(RootWidget),
            parent: None,
            children: Vec::new(),
            next: 0,
        });
        Self {
            nodes: RefCell::new(nodes),
            stack: RefCell::default(),
            removed: RefCell::default(),
            root,
        }
    }
}
