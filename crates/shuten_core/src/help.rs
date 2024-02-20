//! Render helpers
//!
//! While these may be seemingly useless, they can be used to explain what a [`Context::end_frame`](crate::Context::end_frame) does
//!
//! Provided render helpers:
//!
//! | name | property |
//! | --- | --- |
//! | [`explain_frame`] | explain, in a human form, what operations are required to update the screen |
//! | [`debug_frame`] | show a somewhat readable form of the ansi escape sequences required to update the screen |
//!
pub(crate) mod explain;
pub use explain::explain_frame;

pub(crate) mod debug;
pub use debug::debug_frame;
