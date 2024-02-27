use shuten::{event::Event, geom::Rect, Canvas};

use crate::{
    input::{Handled, Input},
    layout::Layout,
    paint::Paint,
    tree::Tree,
};

pub struct Application {
    tree: Tree,
    layout: Layout,
    paint: Paint,
    input: Input,
}

impl Application {
    pub fn new(rect: Rect) -> Self {
        Self {
            tree: Tree::new(),
            layout: Layout::new(rect.into()),
            paint: Paint::new(),
            input: Input::new(),
        }
    }

    pub fn handle(&mut self, event: Event) -> bool {
        let resp = self.input.handle(&self.tree, &self.layout, &event);
        if let Event::Invalidate(rect) = event {
            self.layout.resize(rect.into());
        }
        resp == Handled::Sink
    }

    pub fn start(&mut self) {
        self.tree.start();
        self.input.start(&self.tree, &self.layout);
        self.paint.start();
        crate::tree::bind(&self.tree);
    }

    pub fn finish(&mut self) {
        crate::tree::unbind();
        self.tree.finish();
        self.layout.finish(&self.tree, &self.input);
        self.input.finish();
    }

    pub fn paint(&mut self, canvas: Canvas<'_>) {
        self.paint.paint_all(&self.tree, &self.layout, canvas);
    }
}
