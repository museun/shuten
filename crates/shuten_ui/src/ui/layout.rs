use shuten::geom::{Constraints, Pos2f, Rectf, Vec2f};
use slotmap::{SecondaryMap, SlotMap};

use crate::{
    input::{Keyboard, Mouse},
    ui::Nodes,
    LayoutNode, Node, WidgetId,
};

pub struct Layout<'a> {
    pub(crate) nodes: &'a SlotMap<WidgetId, Node>,
    pub(crate) computed: &'a mut SecondaryMap<WidgetId, LayoutNode>,
    pub(crate) stack: &'a mut Vec<WidgetId>,
    pub(crate) mouse: &'a mut Mouse,
    pub(crate) keyboard: &'a mut Keyboard,
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
