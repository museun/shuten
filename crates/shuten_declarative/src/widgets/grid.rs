use shuten::geom::{
    pos2f, CrossAxisAlignment, MainAxisAlignItems, MainAxisAlignment, MainAxisSize,
};

use super::Direction;
use crate::widget::prelude::*;

#[derive(Debug)]
pub struct Grid {
    direction: Direction,
    cross_axis_count: usize,
    main_axis_alignment: MainAxisAlignment,
    main_axis_size: MainAxisSize,
    main_axis_align_items: MainAxisAlignItems,
    cross_axis_alignment: CrossAxisAlignment,
}

impl Grid {
    pub const fn column(count: usize) -> Self {
        Self {
            direction: Direction::Down,
            cross_axis_count: count,
            main_axis_alignment: MainAxisAlignment::Start,
            main_axis_size: MainAxisSize::Max,
            main_axis_align_items: MainAxisAlignItems::Start,
            cross_axis_alignment: CrossAxisAlignment::Start,
        }
    }

    pub const fn row(count: usize) -> Self {
        Self {
            direction: Direction::Right,
            cross_axis_count: count,
            main_axis_alignment: MainAxisAlignment::Start,
            main_axis_size: MainAxisSize::Max,
            main_axis_align_items: MainAxisAlignItems::Start,
            cross_axis_alignment: CrossAxisAlignment::Start,
        }
    }

    pub const fn main_axis_alignment(mut self, main_axis_alignment: MainAxisAlignment) -> Self {
        self.main_axis_alignment = main_axis_alignment;
        self
    }

    pub const fn main_axis_size(mut self, main_axis_size: MainAxisSize) -> Self {
        self.main_axis_size = main_axis_size;
        self
    }

    pub const fn main_axis_align_items(
        mut self,
        main_axis_align_items: MainAxisAlignItems,
    ) -> Self {
        self.main_axis_align_items = main_axis_align_items;
        self
    }

    pub const fn cross_axis_alignment(mut self, cross_axis_alignment: CrossAxisAlignment) -> Self {
        self.cross_axis_alignment = cross_axis_alignment;
        self
    }

    pub fn show(self, children: impl FnOnce()) -> Response {
        GridWidget::show_children(children, self)
    }
}

#[derive(Debug)]
struct GridWidget {
    props: Grid,
}

impl Default for GridWidget {
    fn default() -> Self {
        Self {
            props: Grid::column(0),
        }
    }
}

impl Widget for GridWidget {
    type Props<'a> = Grid;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let cross = self.props.cross_axis_count;
        let direction = self.props.direction;

        let n = node.children().len();
        let cells = n + (cross - n % cross) % cross;
        let main = cells / cross;

        let cell_cross_max = direction.get_cross_axis(input.max) / cross as f32;
        let cell_cross_min = match self.props.cross_axis_alignment {
            CrossAxisAlignment::Stretch => cell_cross_max,
            _ => 0.0,
        };

        let mut total_main_max = direction.get_main_axis(input.max);
        if total_main_max.is_infinite() {
            total_main_max = direction.get_main_axis(input.min)
        }

        let cell_main_max = total_main_max / main as f32;
        let cell_main_min = match self.props.main_axis_align_items {
            MainAxisAlignItems::Stretch => cell_main_max,
            _ => 0.0,
        };

        let constraints = Constraints {
            min: direction.size(cell_main_min, cell_cross_min),
            max: direction.size(cell_main_max, cell_cross_max),
        };

        // TODO cache this
        let mut sizes = vec![0.0_f32; cross + main];

        for (i, &child) in node.children().iter().enumerate() {
            let size = ctx.calculate(child, constraints);

            let x = i / cross;
            let y = i % cross;

            let main_size = direction.get_main_axis(size);
            let cross_size = direction.get_cross_axis(size);

            sizes[cross + x] = sizes[cross + x].max(main_size);
            sizes[y] = sizes[y].max(cross_size);
        }

        let mut total_main_size = 0.0_f32;
        let mut max_total_cross_size = 0.0_f32;

        for main_axis_id in 0..main {
            let mut total_cross_size = 0.0;

            let start = main_axis_id * cross;
            let end = ((main_axis_id + 1) * cross).min(node.children.len());

            for (cross_axis_id, &child) in node.children[start..end].iter().enumerate() {
                let layout = &mut ctx.layout[child];
                let cross_axis_size = match self.props.cross_axis_alignment {
                    CrossAxisAlignment::Stretch => cell_cross_max,
                    _ => sizes[cross_axis_id],
                };
                let pos = direction.size(total_main_size, total_cross_size);
                layout.rect.set_pos(pos.to_pos2());

                total_cross_size += cross_axis_size;
                max_total_cross_size = max_total_cross_size.max(total_cross_size);
            }

            total_main_size += sizes[cross + main_axis_id]
        }

        let mut offset_main_global = match self.props.main_axis_alignment {
            MainAxisAlignment::Start => 0.0,
            MainAxisAlignment::Center => ((total_main_max - total_main_size) * 0.5).max(0.0),
            MainAxisAlignment::End => (total_main_max - total_main_size).max(0.0),
            e => unimplemented!("invalid main alignment: {e:?}"),
        };

        offset_main_global = match self.props.main_axis_size {
            MainAxisSize::Max => offset_main_global,
            MainAxisSize::Min => 0.0,
            _ => unreachable!(),
        };

        let offset_cross_global = match self.props.cross_axis_alignment {
            CrossAxisAlignment::Start | CrossAxisAlignment::Stretch => 0.0,
            CrossAxisAlignment::Center => {
                ((direction.get_cross_axis(input.min) - max_total_cross_size) * 0.5).max(0.0)
            }
            CrossAxisAlignment::End => {
                (direction.get_cross_axis(input.min) - max_total_cross_size).max(0.0)
            }
        };

        for (i, &child) in node.children.iter().enumerate() {
            let layout = &mut ctx.layout[child];

            let y = i % cross;
            let x = i / cross;

            let child_cross_size = direction.get_cross_axis(layout.rect.size());
            let cell_cross_size = match self.props.cross_axis_alignment {
                CrossAxisAlignment::Stretch => cell_cross_max,
                _ => sizes[y],
            };

            let offset_cross = match self.props.cross_axis_alignment {
                CrossAxisAlignment::Start | CrossAxisAlignment::Stretch => 0.0,
                CrossAxisAlignment::Center => ((cell_cross_size - child_cross_size) * 0.5).max(0.0),
                CrossAxisAlignment::End => (cell_cross_size - child_cross_size).max(0.0),
            };

            let child_main_size = direction.get_main_axis(layout.rect.size());
            let cell_main_size = match self.props.main_axis_align_items {
                MainAxisAlignItems::Start | MainAxisAlignItems::Stretch => cell_main_max,
                _ => sizes[cross + x],
            };

            let offset_main = match self.props.main_axis_align_items {
                MainAxisAlignItems::Start | MainAxisAlignItems::Stretch => 0.0,
                MainAxisAlignItems::Center => ((cell_main_size - child_main_size) * 0.5).max(0.0),
                MainAxisAlignItems::End => (cell_main_size - child_main_size).max(0.0),
            };

            let offset = pos2f(layout.rect.left(), layout.rect.top())
                + direction.size(
                    offset_main_global + offset_main,
                    offset_cross_global + offset_cross,
                );

            layout.rect.set_pos(offset);
        }

        // TODO maybe cache the `sizes`

        let cross_grid_size = match self.props.cross_axis_alignment {
            CrossAxisAlignment::Stretch => direction.get_cross_axis(input.max),
            _ => max_total_cross_size,
        };

        let main_grid_size = match self.props.main_axis_size {
            MainAxisSize::Max => total_main_max,
            MainAxisSize::Min => total_main_size,
            _ => unreachable!(),
        };

        direction.size(main_grid_size, cross_grid_size)
    }
}

pub fn grid_row(count: usize, children: impl FnOnce()) -> Response {
    Grid::row(count).show(children)
}

pub fn grid_column(count: usize, children: impl FnOnce()) -> Response {
    Grid::column(count).show(children)
}
