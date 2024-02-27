macro_rules! widgets {
    ($($ident:ident)*) => {
        $(
            mod $ident;
            #[doc(inline)]
            pub use self::$ident::*;
        )*
    };
}

widgets! {
    align
    button
    color_box
    constrained
    draggable
    flexible
    float
    keyboard_area
    label
    list
    margin
    max_size
    min_size
    mouse_area
    offset
    reflow
    render_cell
    slider
    spacer
    state
    toggle
    unconstrained
}

// TODO: a style widget

pub mod selected_view;

pub mod scrollable;

// mod wrap;
// mod divider;
// mod panel;
pub mod text_input;

// TODO this (needs grid support)
pub mod metrics;

pub mod grid;
