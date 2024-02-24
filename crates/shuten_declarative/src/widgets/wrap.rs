#![cfg_attr(debug_assertions, allow(dead_code, unused_variables,))]
use crate::{
    context::LayoutCtx,
    geom::{Constraints, Vec2f},
    widget::Response,
    Widget, WidgetExt,
};

use super::{Direction, NoResponse};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum WrapAlignment {
    Start,
    Center,
    End,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum WrapCrossAlignment {
    Start,
    Center,
    End,
}

pub struct Wrap {
    direction: Direction,
    spacing: Vec2f,
    alignment: WrapAlignment,
    run_alignment: WrapAlignment,
    cross_axis_alignment: WrapCrossAlignment,
}

impl Wrap {
    pub const fn new(direction: Direction) -> Self {
        Self {
            direction,
            alignment: WrapAlignment::Start,
            run_alignment: WrapAlignment::Start,
            cross_axis_alignment: WrapCrossAlignment::Start,
            spacing: Vec2f::splat(0.0),
        }
    }

    pub const fn horizontal() -> Self {
        Self::new(Direction::Right)
    }

    pub const fn vertical() -> Self {
        Self::new(Direction::Down)
    }

    pub const fn spacing(mut self, spacing: Vec2f) -> Self {
        self.spacing = spacing;
        self
    }

    pub const fn alignment(mut self, alignment: WrapAlignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub const fn run_alignment(mut self, run_alignment: WrapAlignment) -> Self {
        self.run_alignment = run_alignment;
        self
    }

    pub const fn cross_axis_alignment(mut self, cross_axis_alignment: WrapCrossAlignment) -> Self {
        self.cross_axis_alignment = cross_axis_alignment;
        self
    }

    pub fn show(self, children: impl FnOnce()) -> Response {
        WrapWidget::show_children(children, self)
    }
}

pub struct WrapWidget {
    props: Wrap,
}

impl Default for WrapWidget {
    fn default() -> Self {
        Self {
            props: Wrap::horizontal(),
        }
    }
}

impl Widget for WrapWidget {
    type Props<'a> = Wrap;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        todo!();
    }
}

pub fn wrap(direction: Direction, children: impl FnOnce()) -> Response {
    Wrap::new(direction).show(children)
}

pub fn wrap_horizontal(children: impl FnOnce()) -> Response {
    Wrap::horizontal().show(children)
}

pub fn wrap_vertical(children: impl FnOnce()) -> Response {
    Wrap::vertical().show(children)
}
