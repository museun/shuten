use shuten::geom::{
    vec2f, Constraints, CrossAxisAlignment, FlexFit, Flow, MainAxisAlignment, MainAxisSize, Vec2f,
};

use crate::{ui::LayoutCtx, NoResponse, Ui, Widget};

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Direction {
    Horizontal,
    Vertical,
}

impl Direction {
    pub const fn size(&self, main: f32, cross: f32) -> Vec2f {
        match self {
            Self::Horizontal => vec2f(main, cross),
            Self::Vertical => vec2f(cross, main),
        }
    }

    pub const fn get_main_axis(&self, size: Vec2f) -> f32 {
        match self {
            Self::Horizontal => size.x,
            Self::Vertical => size.y,
        }
    }

    pub const fn get_cross_axis(&self, size: Vec2f) -> f32 {
        match self {
            Self::Horizontal => size.y,
            Self::Vertical => size.x,
        }
    }
}

#[derive(Debug)]
pub struct List {
    direction: Direction,
    spacing: f32,
    main_axis_size: MainAxisSize,
    main_axis_alignment: MainAxisAlignment,
    cross_axis_alignment: CrossAxisAlignment,
}

impl Default for List {
    fn default() -> Self {
        Self::row()
    }
}

impl List {
    pub const fn new(direction: Direction) -> Self {
        Self {
            direction,
            spacing: 0.0,
            main_axis_size: MainAxisSize::Max,
            main_axis_alignment: MainAxisAlignment::Start,
            cross_axis_alignment: CrossAxisAlignment::Start,
        }
    }

    pub const fn row() -> Self {
        Self::new(Direction::Horizontal)
    }

    pub const fn column() -> Self {
        Self::new(Direction::Vertical)
    }

    pub const fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub const fn main_axis_size(mut self, main_axis_size: MainAxisSize) -> Self {
        self.main_axis_size = main_axis_size;
        self
    }

    pub const fn main_axis_alignment(mut self, main_axis_alignment: MainAxisAlignment) -> Self {
        self.main_axis_alignment = main_axis_alignment;
        self
    }

    pub const fn cross_axis_alignment(mut self, cross_axis_alignment: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = cross_axis_alignment;
        self
    }
}

#[derive(Default, Debug)]
pub struct ListWidget {
    props: List,
}

impl Widget for ListWidget {
    type Response = NoResponse;
    type Props<'a> = List;

    fn update(&mut self, _: &Ui, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn flex(&self) -> (u16, FlexFit) {
        (self.props.cross_axis_alignment.flex(), FlexFit::Tight)
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        let node = ctx.nodes.current();

        let mut total_main_axis_size =
            self.props.spacing * node.children().len().saturating_sub(1) as f32;
        let mut max_cross_axis_size = 0.0_f32;

        let direction = self.props.direction;
        let cross_axis_max = direction.get_cross_axis(input.max);
        let cross_axis_min = match self.props.cross_axis_alignment {
            CrossAxisAlignment::Stretch => cross_axis_max,
            _ => 0.0,
        };

        let mut main_axis_max = direction.get_main_axis(input.max);
        if main_axis_max.is_infinite() {
            main_axis_max = direction.get_main_axis(input.min)
        }

        let mut total_flex = 0;
        for &child_id in node.children() {
            let child = &ctx.nodes[child_id];
            let (flex, _) = child.widget.flex();
            total_flex += flex;

            if flex != 0 {
                continue;
            }

            if child.widget.flow().is_relative() {
                continue;
            }

            let constraints = Constraints {
                min: direction.size(0.0, cross_axis_min),
                max: direction.size(f32::INFINITY, cross_axis_max),
            };
            let size = ctx.layout.compute(child_id, constraints);
            total_main_axis_size += direction.get_main_axis(size);
            max_cross_axis_size = max_cross_axis_size.max(direction.get_cross_axis(size));
        }

        let remaining_main_axis = (main_axis_max - total_main_axis_size).max(0.0);
        for &child_id in node.children() {
            let child = &ctx.nodes[child_id];
            let (flex, fit) = child.widget.flex();
            if flex == 0 {
                continue;
            }

            if child.widget.flow().is_relative() {
                continue;
            }

            // TODO css-flow (if its relative skip it here)
            let main_axis_size = flex as f32 * remaining_main_axis / total_flex as f32;
            let (min, max) = match fit {
                FlexFit::Loose => (
                    direction.size(0.0, cross_axis_max),
                    direction.size(main_axis_size, cross_axis_max),
                ),
                FlexFit::Tight => (
                    direction.size(main_axis_size, cross_axis_min),
                    direction.size(main_axis_size, cross_axis_max),
                ),
            };
            let constraints = Constraints { min, max };
            let size = ctx.layout.compute(child_id, constraints);
            total_main_axis_size += direction.get_main_axis(size);
            max_cross_axis_size = max_cross_axis_size.max(direction.get_cross_axis(size));
        }

        let cross_size = max_cross_axis_size.max(direction.get_cross_axis(input.min));
        let main_axis_size = match self.props.main_axis_size {
            MainAxisSize::Max => total_main_axis_size,
            MainAxisSize::Min => {
                let max = direction.get_main_axis(input.max);
                if max.is_infinite() {
                    total_main_axis_size
                } else {
                    total_main_axis_size.max(max)
                }
            }
            _ => unreachable!(),
        };

        let container = input.constrain(direction.size(main_axis_size, cross_size));

        for &child_id in node.children() {
            let child = &ctx.nodes[child_id];
            let Flow::Relative { anchor, offset } = child.widget.flow() else {
                continue;
            };

            ctx.layout.compute(child_id, Constraints::none());
            let anchor = container * vec2f(anchor.x.factor(), anchor.y.factor());
            ctx.layout
                .set_pos(child_id, (anchor + offset.resolve(container)).to_pos2());
        }

        let (leading, mut between) = spacing(
            self.props.main_axis_alignment,
            node.children().len(),
            main_axis_size,
            total_main_axis_size,
        );

        between += self.props.spacing;
        let mut next = leading;

        for &child_id in node.children() {
            if ctx.nodes[child_id].widget.flow().is_relative() {
                continue;
            }

            let layout = &mut ctx.layout[child_id];
            let size = layout.rect.size();
            let main = direction.get_main_axis(size);
            let cross = direction.get_cross_axis(size);

            let cross = match self.props.cross_axis_alignment {
                CrossAxisAlignment::Start | CrossAxisAlignment::Stretch => 0.0,
                CrossAxisAlignment::Center => (cross_size - cross) / 2.0,
                CrossAxisAlignment::End => cross_size - cross,
            };

            layout.rect.set_pos(direction.size(next, cross).to_pos2());
            next += main + between
        }

        container
    }
}

fn spacing(
    alignment: MainAxisAlignment,
    children: usize,
    main_size: f32,
    total_size: f32,
) -> (f32, f32) {
    match alignment {
        MainAxisAlignment::Start => (0.0, 0.0),
        MainAxisAlignment::Center => ((total_size - main_size) / 2.0, 0.0),
        MainAxisAlignment::End => (total_size - main_size, 0.0),
        MainAxisAlignment::SpaceAround if children == 0 => (0.0, 0.0),
        MainAxisAlignment::SpaceAround => {
            let space = (total_size - main_size) / children as f32;
            (space * 0.5, space)
        }
        MainAxisAlignment::SpaceBetween if children <= 1 => (0.0, 0.0),
        MainAxisAlignment::SpaceBetween => {
            (0.0, (total_size - main_size) / (children as f32 - 1.0))
        }
        MainAxisAlignment::SpaceEvenly => {
            let space = (total_size - main_size) / (children as f32 + 1.0);
            (space, space)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spacing() {
        for alignment in [
            MainAxisAlignment::Start,
            MainAxisAlignment::Center,
            MainAxisAlignment::End,
            MainAxisAlignment::SpaceAround,
            MainAxisAlignment::SpaceBetween,
            MainAxisAlignment::SpaceEvenly,
        ] {
            let (leading, between) = super::spacing(alignment, 5, 10.0, 30.0);
            eprintln!("{alignment:?}: {leading:.2?} | {between:.2?}");
        }
    }
}
