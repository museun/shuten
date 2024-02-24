use shuten::Canvas;

use crate::{
    geom::{Constraints, Rectf, Vec2f},
    input::Input,
    layout::{Layout, Node},
    paint::Paint,
    tree::{Tree, WidgetId},
};

pub struct EventCtx<'a> {
    pub tree: &'a Tree,
    pub layout: &'a Layout,
    pub input: &'a Input,
}

impl<'a> EventCtx<'a> {
    pub fn current_rect(&self) -> Rectf {
        self.layout[self.tree.current()].rect
    }
}

pub struct LayoutCtx<'a> {
    pub tree: &'a Tree,
    pub input: &'a Input,
    pub layout: &'a mut Layout,
}

impl<'a> LayoutCtx<'a> {
    pub fn current_rect(&self) -> Rectf {
        self.layout[self.tree.current()].rect
    }

    pub fn total_rect(&self) -> Rectf {
        self.layout.rect
    }

    pub fn calculate(&mut self, widget: WidgetId, constraints: Constraints) -> Vec2f {
        self.layout
            .calculate(self.tree, self.input, widget, constraints)
    }
}

pub struct PaintCtx<'a, 'c>
where
    'c: 'a,
{
    pub tree: &'a Tree,
    pub layout: &'a Layout,
    pub canvas: &'a mut Canvas<'c>,
    pub rect: Rectf,
    pub(super) paint: &'a mut Paint,
}

impl<'a, 'c> PaintCtx<'a, 'c>
where
    'c: 'a,
{
    pub fn paint(&mut self, id: WidgetId) {
        self.paint.paint(self.tree, self.layout, self.canvas, id)
    }

    #[must_use]
    pub fn current_layout_node(&self) -> &Node {
        &self.layout[self.tree.current()]
    }

    #[must_use]
    pub fn current_rect(&self) -> Rectf {
        self.current_layout_node().rect
    }

    #[must_use]
    pub fn cropped_canvas(&mut self) -> Canvas {
        self.canvas.crop(self.rect.into())
    }
}
