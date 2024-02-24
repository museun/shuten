use crate::{
    geom::{Constraints, Rectf, Vec2f},
    input::Input,
    tree::{Tree, WidgetId},
};

use super::Layout;

pub struct LayoutCtx<'a> {
    pub tree: &'a Tree,
    pub input: &'a Input,
    pub layout: &'a mut Layout,
}

impl<'a> LayoutCtx<'a> {
    pub fn current_rect(&self) -> Rectf {
        let id = self.tree.current();
        self.layout.get(id).unwrap().rect
    }

    pub fn total_rect(&self) -> Rectf {
        self.layout.rect
    }

    pub fn calculate(&mut self, widget: WidgetId, constraints: Constraints) -> Vec2f {
        self.layout.calculate(
            self.tree, //
            self.input,
            widget,
            constraints,
        )
    }
}
