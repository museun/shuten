use crate::tree::WidgetId;

#[derive(Debug)]
pub struct Layered<T = ()> {
    layers: Vec<Vec<(WidgetId, T)>>,
    stack: Vec<(WidgetId, usize)>,
}

impl<T> Default for Layered<T> {
    fn default() -> Self {
        Self {
            layers: Vec::new(),
            stack: Vec::new(),
        }
    }
}

impl<T> Layered<T> {
    pub fn clear(&mut self) {
        self.layers.clear();
        self.stack.clear();
    }

    pub fn insert(&mut self, id: WidgetId, data: T) {
        self.stack
            .last()
            .and_then(|(_, index)| self.layers.get_mut(*index))
            .unwrap()
            .push((id, data))
    }

    pub fn current_layer_root(&self) -> Option<WidgetId> {
        self.stack.last().map(|&(id, _)| id)
    }

    pub fn push_layer(&mut self, id: WidgetId) {
        let index = self.layers.len();
        self.layers.push(vec![]);
        self.stack.push((id, index))
    }

    pub fn pop_layer(&mut self) {
        debug_assert!(self.stack.pop().is_some(), "cannot pop without a push")
    }

    pub fn iter(&self) -> impl Iterator<Item = (WidgetId, &T)> + '_ {
        self.layers
            .iter()
            .rev()
            .flat_map(|layer| layer.iter().map(|(id, data)| (*id, data)))
    }
}
