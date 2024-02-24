use slotmap::{SecondaryMap, SlotMap};

pub struct SlotMapPrinter<'a, K: slotmap::Key, V>(pub &'a SlotMap<K, V>);

impl<'a, K: slotmap::Key, V: std::fmt::Debug> std::fmt::Debug for SlotMapPrinter<'a, K, V> {
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

pub struct SecondaryMapPrinter<'a, K: slotmap::Key, V>(pub &'a SecondaryMap<K, V>);

impl<'a, K: slotmap::Key, V: std::fmt::Debug> std::fmt::Debug for SecondaryMapPrinter<'a, K, V> {
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

    pub fn serialize<S, K: slotmap::Key, T: ::serde::Serialize>(
        arena: &slotmap::SlotMap<K, T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(arena.len()))?;
        for (id, val) in arena {
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

    pub fn serialize<S, K: slotmap::Key, T: ::serde::Serialize>(
        arena: &slotmap::SecondaryMap<K, T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(arena.len()))?;
        for (id, val) in arena {
            seq.serialize_element(&(format!("{:?}", id.data()), val))?;
        }
        seq.end()
    }
}
