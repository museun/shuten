use crate::{geom::Rectf, input::Input, layout::Layout, tree::Tree};

pub struct EventCtx<'a> {
    pub tree: &'a Tree,
    pub layout: &'a Layout,
    pub input: &'a Input,
}

impl<'a> EventCtx<'a> {
    pub fn current_rect(&self) -> Rectf {
        let id = self.tree.current();
        self.layout.get(id).unwrap().rect
    }
}
