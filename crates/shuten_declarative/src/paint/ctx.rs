use shuten::Canvas;

use crate::{
    geom::Rectf,
    layout::{Layout, Node},
    paint::Paint,
    tree::{Tree, WidgetId},
};

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
        self.layout.get(self.tree.current()).unwrap()
    }

    #[must_use]
    pub fn current_rect(&self) -> Rectf {
        self.current_layout_node().rect()
    }

    #[must_use]
    pub fn cropped_canvas(&mut self) -> Canvas {
        self.canvas.crop(self.rect.into())
    }
}
