use slotmap::SlotMap;

use crate::{Node, WidgetId};

// TODO rename this
#[derive(Copy, Clone)]
pub struct Nodes<'a> {
    pub(crate) nodes: &'a SlotMap<WidgetId, Node>,
    pub(crate) current: WidgetId,
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
