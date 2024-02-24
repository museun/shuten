#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct WidgetId(pub(in crate::tree) thunderdome::Index);

impl serde::Serialize for WidgetId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&format!("{self:?}"))
    }
}

impl std::fmt::Debug for WidgetId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}v{}", self.0.slot(), self.0.generation())
    }
}

impl WidgetId {
    pub(crate) const fn get(self) -> thunderdome::Index {
        self.0
    }
}
