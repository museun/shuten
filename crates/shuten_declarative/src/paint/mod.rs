use std::borrow::Cow;

use shuten::{
    geom::{pos2, vec2, Rect},
    Canvas, Cell,
};

use crate::{
    context::PaintCtx,
    geom::Rectf,
    layout::Layout,
    tree::{Tree, WidgetId},
};

#[derive(Debug)]
pub struct Paint {
    clip_stack: Vec<Rectf>,
    pub(crate) debug_layer: Vec<Cow<'static, str>>,
}

impl Paint {
    pub(crate) const fn new() -> Self {
        Self {
            clip_stack: Vec::new(),
            debug_layer: Vec::new(),
        }
    }

    pub(crate) fn start(&mut self) {
        self.clip_stack.clear();
    }

    pub(crate) fn paint(
        &mut self,
        tree: &Tree,
        layout: &Layout,
        canvas: &mut Canvas<'_>,
        id: WidgetId,
    ) {
        // if this node doesn't exist in the layout, don't paint it
        let Some(node) = layout.get(id) else { return };

        if node.clipping {
            self.push_clip(node.rect)
        }

        tree.enter(id);
        let ctx = PaintCtx {
            tree,
            layout,
            paint: self,
            rect: layout[id].rect,
            canvas,
        };
        tree.get(id).unwrap().widget.paint(ctx);
        tree.exit(id);

        if node.clipping {
            self.pop_clip();
        }
    }

    pub(crate) fn paint_all(&mut self, tree: &Tree, layout: &Layout, mut canvas: Canvas<'_>) {
        use shuten::style::Rgb;
        use std::f32::consts::PI;
        fn next_color(n: f32) -> Rgb {
            let h = n * ((1.0 + 5.0_f32.sqrt()) / 2.0);
            let h = (h + 0.5) * -1.0;
            let r = (PI * h).sin();
            let g = (PI * (h + 0.3)).sin();
            let b = (PI * (h + 0.6)).sin();
            Rgb::from_float([r * r, g * g, b * b])
        }

        self.paint(tree, layout, &mut canvas, tree.root());

        let rect = canvas.area();
        let mut start = rect.left_top();
        'outer: for (i, debug) in self.debug_layer.drain(..).enumerate() {
            if debug.trim().is_empty() {
                continue;
            }

            let h = debug.lines().count();
            let w = debug.lines().map(|s| s.len()).max().unwrap_or(1);
            canvas.fill_rect(
                Rect::from_min_size(start, vec2(w as u16, h as u16)),
                next_color(i as f32),
            );

            for ch in debug.chars() {
                if start.x >= rect.right() || ch == '\n' {
                    start.x = rect.left();
                    start.y += 1;
                    if ch == '\n' {
                        continue;
                    }
                }

                if start.y > rect.bottom() {
                    break 'outer;
                }
                canvas.put(start, Cell::new(ch).fg(0xFF0000));
                start += pos2(1, 0);
            }

            start.y += 1;
            start.x = rect.left();
        }
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
