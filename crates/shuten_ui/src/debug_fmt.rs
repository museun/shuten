use crate::WidgetId;

use slotmap::{Key, SecondaryMap, SlotMap};
use std::fmt::Debug;

pub const fn str(s: &str) -> impl Debug + '_ {
    struct NoQuote<'a>(&'a str);
    impl<'a> Debug for NoQuote<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.0)
        }
    }
    NoQuote(s)
}

pub const fn id(id: WidgetId) -> impl Debug {
    struct ShortId(WidgetId);
    impl Debug for ShortId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0.data())
        }
    }
    ShortId(id)
}

pub const fn vec(list: &Vec<WidgetId>) -> impl Debug + '_ {
    struct Inner<'a>(&'a Vec<WidgetId>);
    impl<'a> Debug for Inner<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_list()
                .entries(self.0.iter().map(|&id| self::id(id)))
                .finish()
        }
    }
    Inner(list)
}

pub const fn slot_map<T: Debug>(map: &SlotMap<WidgetId, T>) -> impl Debug + '_ {
    struct Inner<'a, T>(&'a SlotMap<WidgetId, T>);
    impl<'a, T: Debug> Debug for Inner<'a, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_map()
                .entries(self.0.iter().map(|(k, v)| (self::id(k), v)))
                .finish()
        }
    }
    Inner(map)
}

pub fn secondary_map<T: Debug>(map: &SecondaryMap<WidgetId, T>) -> impl Debug + '_ {
    struct Inner<'a, T>(&'a SecondaryMap<WidgetId, T>);
    impl<'a, T: Debug> Debug for Inner<'a, T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_map()
                .entries(self.0.iter().map(|(k, v)| (id(k), v)))
                .finish()
        }
    }
    Inner(map)
}
