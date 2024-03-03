use shuten::{geom::Rectf, Canvas};

use crate::{Ui, WidgetId};

pub struct Paint;

impl Paint {
    pub fn paint_all(&mut self, ui: &Ui, canvas: &mut Canvas<'_>) {
        self.paint(ui, canvas, ui.root())
    }

    pub(crate) fn paint(&mut self, ui: &Ui, canvas: &mut Canvas<'_>, id: WidgetId) {
        let computed = ui.inner.computed.borrow();
        let Some(node) = computed.get(id) else {
            return;
        };
        let rect = node.rect;

        let nodes = ui.inner.nodes.borrow();
        let node = &nodes[id];
        ui.inner.stack.borrow_mut().push(id);
        node.widget.paint(PaintCtx {
            rect,
            ui,
            canvas: &mut canvas.crop(rect.into()),
            paint: self,
        });
        assert_eq!(Some(id), ui.inner.stack.borrow_mut().pop());
    }
}

pub struct PaintCtx<'a: 'c, 'c> {
    pub rect: Rectf,
    pub ui: &'a Ui,
    pub canvas: &'a mut Canvas<'c>,
    pub(crate) paint: &'a mut Paint,
}

impl<'a: 'c, 'c> PaintCtx<'a, 'c> {
    pub fn paint(&mut self, id: WidgetId) {
        self.paint.paint(self.ui, self.canvas, id);
    }
}
