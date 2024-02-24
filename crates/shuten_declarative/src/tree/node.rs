use crate::{tree::WidgetId, widget::ErasedWidget};

#[derive(serde::Serialize)]
pub struct Node {
    #[serde(with = "erased_widget")]
    pub(crate) widget: Box<dyn ErasedWidget>,
    pub(crate) parent: Option<WidgetId>,
    pub(crate) children: Vec<WidgetId>,
    pub(crate) next: usize,
}

mod erased_widget {
    use crate::widget::ErasedWidget;

    #[allow(clippy::borrowed_box)]
    pub fn serialize<S>(ew: &Box<dyn ErasedWidget>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let repr = format!("{ew:#?}",);
        serializer.serialize_str(&repr)
    }
}

impl Node {
    pub const fn parent(&self) -> Option<WidgetId> {
        self.parent
    }

    pub fn children(&self) -> &[WidgetId] {
        &self.children
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("widget", &self.widget)
            .field("interest", &self.widget.interest())
            .field("parent", &self.parent)
            .field("children", &self.children)
            .field("next", &self.next)
            .finish()
    }
}
