use shuten::geom::{vec2f, CrossAxisAlignment, MainAxisAlignment, MainAxisSize};

use crate::widget::prelude::*;

pub fn row(children: impl FnOnce()) -> Response {
    List::row().show(children)
}

pub fn column(children: impl FnOnce()) -> Response {
    List::column().show(children)
}

#[derive(Debug)]
pub struct List {
    direction: Direction,
    spacing: f32,
    main_axis_size: MainAxisSize,
    main_axis_alignment: MainAxisAlignment,
    cross_axis_alignment: CrossAxisAlignment,
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
        Self::new(Direction::Right)
    }

    pub const fn column() -> Self {
        Self::new(Direction::Down)
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

    pub fn show(self, children: impl FnOnce()) -> Response {
        ListWidget::show_children(children, self)
    }
}

#[derive(Debug)]
struct ListWidget {
    props: List,
}

impl Default for ListWidget {
    fn default() -> Self {
        Self { props: List::row() }
    }
}

impl Widget for ListWidget {
    type Props<'a> = List;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn flex(&self) -> (u16, FlexFit) {
        (self.props.cross_axis_alignment.flex(), FlexFit::Tight)
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let direction = self.props.direction;
        let total_item_spacing = self.props.spacing * node.children.len().saturating_sub(1) as f32;

        let mut total_main_axis_size = total_item_spacing;
        let mut max_cross_axis_size = 0.0_f32;

        let cross_axis_max = direction.get_cross_axis(input.max);
        let cross_axis_min = match self.props.cross_axis_alignment {
            CrossAxisAlignment::Stretch => cross_axis_max,
            _ => 0.0,
        };

        let mut main_axis_max = direction.get_main_axis(input.max);
        if main_axis_max.is_infinite() {
            main_axis_max = direction.get_main_axis(input.min);
        }

        let mut total_flex = 0;
        for &child_id in &node.children {
            let child = ctx.tree.get(child_id).unwrap();
            let (flex, _) = child.widget.flex();
            total_flex += flex;

            if flex != 0 {
                continue;
            }
            if child.widget.flow() != Flow::Inline {
                continue;
            }

            let constraints = Constraints {
                min: direction.size(0.0, cross_axis_min),
                max: direction.size(f32::INFINITY, cross_axis_max),
            };

            let size = ctx.calculate(child_id, constraints);
            total_main_axis_size += direction.get_main_axis(size);
            max_cross_axis_size = max_cross_axis_size.max(direction.get_cross_axis(size));
        }

        let remaining_main_axis = (main_axis_max - total_main_axis_size).max(0.0);
        for &child_id in &node.children {
            let child = ctx.tree.get(child_id).unwrap();
            let (flex, fit) = child.widget.flex();
            if flex == 0 {
                continue;
            }
            if child.widget.flow() != Flow::Inline {
                continue;
            }

            let main_axis_size = flex as f32 * remaining_main_axis / total_flex as f32;
            let constraints = match fit {
                FlexFit::Loose => Constraints {
                    min: direction.size(0.0, cross_axis_min),
                    max: direction.size(main_axis_size, cross_axis_max),
                },
                FlexFit::Tight => Constraints {
                    min: direction.size(main_axis_size, cross_axis_min),
                    max: direction.size(main_axis_size, cross_axis_max),
                },
            };

            let size = ctx.calculate(child_id, constraints);
            total_main_axis_size += direction.get_main_axis(size);
            max_cross_axis_size = max_cross_axis_size.max(direction.get_cross_axis(size));
        }

        let cross_size = max_cross_axis_size.max(direction.get_cross_axis(input.min));
        let main_axis_size = match self.props.main_axis_size {
            MainAxisSize::Min => total_main_axis_size,
            MainAxisSize::Max => {
                let main_max = direction.get_main_axis(input.max);
                if main_max.is_finite() {
                    total_main_axis_size.max(main_max)
                } else {
                    total_main_axis_size
                }
            }
            _ => unreachable!(),
        };

        let container_size = input.constrain(direction.size(main_axis_size, cross_size));

        for &child_id in &node.children {
            let child = ctx.tree.get(child_id).unwrap();
            let Flow::Relative { anchor, offset } = child.widget.flow() else {
                continue;
            };

            ctx.calculate(child_id, Constraints::none());
            let anchor = container_size * vec2f(anchor.x.factor(), anchor.y.factor());
            let offset = offset.resolve(container_size);
            ctx.layout.set_pos(child_id, (anchor + offset).to_pos2());
        }

        let (leading, mut between) = match self.props.main_axis_alignment {
            MainAxisAlignment::Start => (0.0, 0.0),
            MainAxisAlignment::Center => ((main_axis_size - total_main_axis_size) / 2.0, 0.0),
            MainAxisAlignment::End => (main_axis_size - total_main_axis_size, 0.0),
            MainAxisAlignment::SpaceAround if node.children.is_empty() => (0.0, 0.0),
            MainAxisAlignment::SpaceAround => {
                let space = (main_axis_size - total_main_axis_size) / node.children.len() as f32;
                (space * 0.5, space)
            }
            MainAxisAlignment::SpaceBetween if node.children.len() <= 1 => (0.0, 0.0),
            MainAxisAlignment::SpaceBetween => (
                0.0,
                (main_axis_size - total_main_axis_size) / (node.children.len() as f32 - 1.0),
            ),
            MainAxisAlignment::SpaceEvenly => {
                let space =
                    (main_axis_size - total_main_axis_size) / (node.children.len() as f32 + 1.0);
                (space, space)
            }
        };

        between += self.props.spacing;
        let mut next_main = leading;

        for &child_id in &node.children {
            let child = ctx.tree.get(child_id).unwrap();
            if child.widget.flow() != Flow::Inline {
                continue;
            }

            let child_layout = &mut ctx.layout[child_id];
            let child_size = child_layout.rect.size();
            let child_main = direction.get_main_axis(child_size);
            let child_cross = direction.get_cross_axis(child_size);

            let cross = match self.props.cross_axis_alignment {
                CrossAxisAlignment::Start | CrossAxisAlignment::Stretch => 0.0,
                CrossAxisAlignment::Center => (cross_size - child_cross) / 2.0,
                CrossAxisAlignment::End => cross_size - child_cross,
            };

            let delta = direction.size(next_main, cross).to_pos2();
            child_layout.rect.set_pos(delta);
            next_main += child_main + between;
        }

        container_size
    }
}

/// The direction that a container will lay out its children
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    /// Lay out the children top-to-bottom
    Down,
    /// Lay out the children left-to-right
    Right,
}

impl Direction {
    /// Given the main and cross values, get the size for this direction
    #[must_use]
    pub const fn size(&self, main: f32, cross: f32) -> Vec2f {
        match self {
            Self::Down => vec2f(cross, main),
            Self::Right => vec2f(main, cross),
        }
    }

    /// The the main axis value from the provided size
    #[must_use]
    pub const fn get_main_axis(&self, size: Vec2f) -> f32 {
        match self {
            Self::Down => size.y,
            Self::Right => size.x,
        }
    }

    /// The the cross axis value from the provided size
    #[must_use]
    pub const fn get_cross_axis(&self, size: Vec2f) -> f32 {
        match self {
            Self::Down => size.x,
            Self::Right => size.y,
        }
    }
}
