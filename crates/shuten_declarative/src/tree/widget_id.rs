use slotmap::Key;

slotmap::new_key_type! {
    pub struct WidgetId;
}

impl serde::Serialize for WidgetId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{key:?}", key = self.data()))
    }
}
