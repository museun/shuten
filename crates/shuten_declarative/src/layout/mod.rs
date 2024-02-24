use std::collections::VecDeque;

use crate::{
    context::LayoutCtx,
    geom::{Constraints, Pos2f, Rectf, Vec2f},
    input::{Input, Interest},
    tree::{Tree, WidgetId},
};

mod layered;
use layered::Layered;

pub(crate) type Keyboard = Layered;
pub(crate) type Mouse = Layered<Interest>;

mod node;
pub use node::Node;
use slotmap::SecondaryMap;

#[derive(serde::Serialize)]
pub struct Layout {
    pub(crate) mouse: Mouse,
    pub(crate) keyboard: Keyboard,
    pub(crate) rect: Rectf,

    #[serde(with = "crate::external::serialize_secondary_map")]
    nodes: SecondaryMap<WidgetId, Node>,
    clip_stack: Vec<WidgetId>,
}

impl std::fmt::Debug for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Layout")
            .field("mouse", &self.mouse)
            .field("keyboard", &self.keyboard)
            .field("nodes", &crate::external::SecondaryMapPrinter(&self.nodes))
            .field("clip_stack", &self.clip_stack)
            .field("rect", &self.rect)
            .finish()
    }
}

impl Layout {
    pub fn new(rect: Rectf) -> Self {
        Self {
            nodes: SecondaryMap::new(),
            clip_stack: Vec::new(),
            mouse: Mouse::default(),
            keyboard: Keyboard::default(),
            rect,
        }
    }

    pub fn resize(&mut self, rect: Rectf) {
        self.rect = rect;
    }

    pub fn get(&self, id: WidgetId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_mut(&mut self, id: WidgetId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn set_pos(&mut self, id: WidgetId, pos: Pos2f) {
        if let Some(node) = self.nodes.get_mut(id) {
            node.rect.set_pos(pos)
        }
    }

    pub fn new_layer(&mut self, tree: &Tree) {
        let id = tree.current();
        self.mouse.push_layer(id);
        self.keyboard.push_layer(id);
    }

    pub fn hide(&mut self, tree: &Tree, id: WidgetId) {
        if let Some(node) = tree.get(id) {
            for &child in &node.children {
                self.nodes.remove(child);
            }
            self.nodes.remove(id);
        }
    }

    pub fn clip(&mut self, tree: &Tree) {
        self.clip_stack.push(tree.current())
    }

    pub(crate) fn finish(&mut self, tree: &Tree, input: &Input) {
        self.cleanup(&tree.removed());
        self.calculate_all(tree, input);
    }

    pub(crate) fn calculate(
        &mut self,
        tree: &Tree,
        input: &Input,
        id: WidgetId,
        constraints: Constraints,
    ) -> Vec2f {
        tree.enter(id);
        let node = tree.get(id).unwrap();
        let size = node.widget.layout(
            LayoutCtx {
                tree,
                input,
                layout: self,
            },
            constraints,
        );

        let new_layer = self.mouse.current_layer_root() == Some(id)
            || self.keyboard.current_layer_root() == Some(id);

        let interest = node.widget.interest();
        if interest.is_mouse_any() {
            self.mouse.insert(id, interest);
        }

        if interest.is_key_input() {
            self.keyboard.insert(id, ());
        }

        if new_layer {
            self.keyboard.pop_layer();
            self.mouse.pop_layer();
        }

        let clipping = self.clip_stack.last() == Some(&id);

        let clipped_by = if clipping {
            self.clip_stack.iter().nth_back(2).copied()
        } else {
            self.clip_stack.last().copied()
        };

        self.nodes.insert(
            id,
            Node {
                rect: Rectf::from_min_size(Pos2f::ZERO, size),
                ty: node.widget.type_name(),
                clipping,
                interest,
                clipped_by,
            },
        );

        tree.exit(id);
        size
    }

    fn calculate_all(&mut self, tree: &Tree, input: &Input) {
        self.clear();

        self.calculate(
            tree,
            input,
            tree.root(),
            Constraints::tight(self.rect.size()),
        );
        self.resolve(tree);
    }

    fn clear(&mut self) {
        self.clip_stack.clear();
        self.mouse.clear();
        self.keyboard.clear();
    }

    fn cleanup(&mut self, widgets: &[WidgetId]) {
        for &id in widgets {
            self.nodes.remove(id);
        }
    }

    fn resolve(&mut self, tree: &Tree) {
        let mut queue = VecDeque::new();
        queue.push_back((tree.root(), Pos2f::ZERO));

        while let Some((id, pos)) = queue.pop_front() {
            let Some(node) = self.nodes.get_mut(id) else {
                continue;
            };
            node.rect.set_pos(node.rect.min + pos);
            let rect = node.rect;

            let node = tree.get(id).unwrap();
            queue.extend(node.children.iter().map(|&id| (id, rect.min)));
        }
    }
}

impl std::ops::Index<WidgetId> for Layout {
    type Output = Node;
    fn index(&self, index: WidgetId) -> &Self::Output {
        &self.nodes[index]
    }
}

impl std::ops::IndexMut<WidgetId> for Layout {
    fn index_mut(&mut self, index: WidgetId) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}
