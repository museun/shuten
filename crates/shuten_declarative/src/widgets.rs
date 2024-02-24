use crate::NoResponse;
use crate::{geom::Vec2f, widget::Response, Color};

mod align;
mod button;
mod color_box;
mod constrained;
mod draggable;
mod flexible;
mod float;
mod keyboard_area;
mod label;
mod list;
mod log_view;
mod margin;
mod max_size;
mod min_size;
mod mouse_area;
mod offset;
mod reflow;
mod render_cell;
mod slider;
mod spacer;
mod state;
mod toggle;
mod unconstrained;

pub mod text_input;

// mod wrap;

// BUG: this isn't working
// mod divider;
// BUG: this isn't working
// mod panel;

#[doc(inline)]
pub use self::align::*;

#[doc(inline)]
pub use self::button::*;

#[doc(inline)]
pub use self::color_box::*;

#[doc(inline)]
pub use self::constrained::*;

#[doc(inline)]
pub use self::draggable::*;

#[doc(inline)]
pub use self::flexible::*;

#[doc(inline)]
pub use self::float::*;

#[doc(inline)]
pub use self::keyboard_area::*;

#[doc(inline)]
pub use self::label::*;

#[doc(inline)]
pub use self::list::*;

#[doc(inline)]
pub use self::log_view::*;

#[doc(inline)]
pub use self::margin::*;

#[doc(inline)]
pub use self::max_size::*;

#[doc(inline)]
pub use self::min_size::*;

#[doc(inline)]
pub use self::mouse_area::*;

#[doc(inline)]
pub use self::offset::*;

#[doc(inline)]
pub use self::reflow::*;

#[doc(inline)]
pub use self::render_cell::*;

#[doc(inline)]
pub use self::slider::*;

#[doc(inline)]
pub use self::spacer::*;

#[doc(inline)]
pub use self::state::*;

#[doc(inline)]
pub use self::toggle::*;

#[doc(inline)]
pub use self::unconstrained::*;

pub fn container(bg: impl Into<Color>, children: impl FnOnce()) -> Response<NoResponse> {
    ColorBox::new(bg, Vec2f::ZERO).show_children(children)
}

pub mod scrollable;
