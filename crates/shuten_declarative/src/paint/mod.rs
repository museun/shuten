use shuten::Canvas;

use crate::{
    geom::Rectf,
    layout::Layout,
    tree::{Tree, WidgetId},
};

pub mod ctx;
pub use ctx::PaintCtx;

#[derive(Debug, serde::Serialize)]
pub struct Paint {
    clip_stack: Vec<Rectf>,
}

impl Paint {
    pub(crate) const fn new() -> Self {
        Self {
            clip_stack: Vec::new(),
        }
    }

    pub(crate) fn start(&mut self) {
        self.clip_stack.clear();
    }

    fn paint(&mut self, tree: &Tree, layout: &Layout, canvas: &mut Canvas<'_>, id: WidgetId) {
        let Some(node) = layout.get(id) else { return };

        if node.clipping {
            self.push_clip(node.rect)
        }

        tree.enter(id);
        let ctx = PaintCtx {
            tree,
            layout,
            paint: self,
            rect: layout.get(id).unwrap().rect,
            canvas,
        };
        tree.get(id).unwrap().widget.paint(ctx);
        tree.exit(id);

        if node.clipping {
            self.pop_clip();
        }
    }

    pub(crate) fn paint_all(&mut self, tree: &Tree, layout: &Layout, mut canvas: Canvas<'_>) {
        self.paint(tree, layout, &mut canvas, tree.root())
    }

    fn push_clip(&mut self, mut region: Rectf) {
        if let Some(previous) = self.clip_stack.last() {
            region = region.constrain(*previous);
        }
        self.clip_stack.push(region);
    }

    fn pop_clip(&mut self) {
        debug_assert!(
            self.clip_stack.pop().is_some(),
            "cannot pop clip without a push clip",
        )
    }
}
