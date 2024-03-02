use shuten::geom::Rectf;

slotmap::new_key_type! {
    pub struct WidgetId;
}

use crate::{debug_fmt, ErasedWidget, Interest};

pub struct Node {
    pub(crate) widget: Box<dyn ErasedWidget>,
    pub(crate) parent: Option<WidgetId>,
    pub(crate) children: Vec<WidgetId>,
    pub(crate) next: usize,
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("widget", &self.widget)
            .field("parent", &self.parent.map(debug_fmt::id))
            .field("children", &debug_fmt::vec(&self.children))
            .field("next", &self.next)
            .finish()
    }
}

impl Node {
    pub fn children(&self) -> &[WidgetId] {
        &self.children
    }
}

#[derive(Debug)]
pub struct LayoutNode {
    pub rect: Rectf,
    pub interest: Interest,
}
