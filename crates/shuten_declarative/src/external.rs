use slotmap::{SecondaryMap, SlotMap};

pub struct SlotMapPrinter<'a, K, V>(pub &'a SlotMap<K, V>)
where
    K: slotmap::Key;

impl<'a, K, V> std::fmt::Debug for SlotMapPrinter<'a, K, V>
where
    K: slotmap::Key,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(
                self.0
                    .iter()
                    .map(|(id, element)| (NoQuote(format!("{:?}", id.data())), element)),
            )
            .finish()
    }
}

pub struct SecondaryMapPrinter<'a, K, V>(pub &'a SecondaryMap<K, V>)
where
    K: slotmap::Key;

impl<'a, K, V> std::fmt::Debug for SecondaryMapPrinter<'a, K, V>
where
    K: slotmap::Key,
    V: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(
                self.0
                    .iter()
                    .map(|(id, element)| (NoQuote(format!("{:?}", id.data())), element)),
            )
            .finish()
    }
}

pub struct NoQuote<S: AsRef<str>>(pub S);

impl<S: AsRef<str>> std::fmt::Debug for NoQuote<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

pub mod serialize_slot_map {
    use serde::ser::SerializeSeq;

    #[derive(serde::Serialize)]
    pub struct SlotMap<'a, K, V>(#[serde(with = "self")] pub &'a slotmap::SlotMap<K, V>)
    where
        K: slotmap::Key,
        V: serde::Serialize;

    pub fn serialize<S, K, T>(
        map: &slotmap::SlotMap<K, T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
        K: slotmap::Key,
        T: ::serde::Serialize,
    {
        let mut seq = serializer.serialize_seq(Some(map.len()))?;
        for (id, val) in map {
            seq.serialize_element(&(format!("{:?}", id.data()), val))?;
        }
        seq.end()
    }
}

pub mod serialize_secondary_map {
    use serde::ser::SerializeSeq;

    #[derive(serde::Serialize)]
    pub struct SecondaryMap<'a, K, V>(#[serde(with = "self")] pub &'a slotmap::SecondaryMap<K, V>)
    where
        K: slotmap::Key,
        V: serde::Serialize;

    pub fn serialize<S, K, T>(
        map: &slotmap::SecondaryMap<K, T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
        K: slotmap::Key,
        T: ::serde::Serialize,
    {
        let mut seq = serializer.serialize_seq(Some(map.len()))?;
        for (id, val) in map {
            seq.serialize_element(&(format!("{:?}", id.data()), val))?;
        }
        seq.end()
    }
}

pub mod erased_widget {
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
