use crate::{tree::WidgetId, widget::ErasedWidget, Widget};

pub struct Node {
    pub(crate) widget: Box<dyn ErasedWidget>,
    pub(crate) parent: Option<WidgetId>,
    pub(crate) children: Vec<WidgetId>,
    pub(crate) next: usize,
}

impl Node {
    pub(super) fn new(widget: impl Widget + 'static, parent: Option<WidgetId>) -> Self {
        Self {
            widget: Box::new(widget),
            parent,
            children: Vec::new(),
            next: 0,
        }
    }

    pub const fn parent(&self) -> Option<WidgetId> {
        self.parent
    }

    pub fn children(&self) -> &[WidgetId] {
        &self.children
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("widget", &self.widget)
            .field("interest", &self.widget.interest())
            .field("parent", &self.parent)
            .field("children", &self.children)
            .field("next", &self.next)
            .finish()
    }
}
