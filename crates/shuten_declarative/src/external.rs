use thunderdome::Arena;

pub struct ArenaPrinter<'a, T>(pub &'a Arena<T>);
impl<'a, T: std::fmt::Debug> std::fmt::Debug for ArenaPrinter<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.0.iter().map(|(id, element)| {
                (
                    NoQuote(format!("{}v{}", id.slot(), id.generation())),
                    element,
                )
            }))
            .finish()
    }
}

pub struct NoQuote<S: AsRef<str>>(pub S);
impl<S: AsRef<str>> std::fmt::Debug for NoQuote<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.as_ref())
    }
}

pub mod serialize_arena {
    use serde::ser::SerializeSeq;
    use thunderdome::Arena;

    #[derive(serde::Serialize)]
    pub struct SerializeArena<'a, T>(#[serde(with = "self")] pub &'a Arena<T>)
    where
        T: serde::Serialize;

    pub fn serialize<S, T: ::serde::Serialize>(
        arena: &Arena<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(arena.len()))?;
        for (id, val) in arena {
            seq.serialize_element(&(format!("{}v{}", id.slot(), id.generation()), val))?;
        }
        seq.end()
    }
}
