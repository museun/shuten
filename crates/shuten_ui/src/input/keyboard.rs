use slotmap::SlotMap;

use crate::{
    input::{Handled, KeyPressed, Layered},
    Node, WidgetId,
};

#[derive(Default, Debug)]
pub(crate) struct Keyboard {
    pub(crate) layered: Layered,
}

impl Keyboard {
    pub fn push_layer(&mut self, id: WidgetId) {
        self.layered.push_layer(id);
    }

    pub fn dispatch(
        &self,
        event: KeyPressed,
        nodes: &mut SlotMap<WidgetId, Node>,
        stack: &mut Vec<WidgetId>,
    ) -> Handled {
        let mut resp = Handled::Bubble;
        for (id, ()) in self.layered.iter() {
            let Some(node) = nodes.get_mut(id) else {
                continue;
            };

            stack.push(id);
            resp = node.widget.on_key_pressed(event);
            assert_eq!(Some(id), stack.pop());

            if matches!(resp, Handled::Sink) {
                break;
            }
        }
        resp
    }
}
