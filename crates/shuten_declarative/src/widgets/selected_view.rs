use std::borrow::Cow;

use shuten::geom::{MainAxisAlignment, Pos2f};

use crate::widget::prelude::*;

use super::{max_height, mouse_area, Label, List};

pub trait Selectable: std::fmt::Debug + PartialEq + Default + Copy + 'static {}
impl<T: std::fmt::Debug + PartialEq + Default + Copy + 'static> Selectable for T {}

#[derive(Debug)]
pub struct SelectedViewLabel<T: Selectable> {
    pub label: Cow<'static, str>,
    pub view: T,
}

impl<T: Selectable> SelectedViewLabel<T> {
    pub fn new(label: impl Into<Cow<'static, str>>, view: T) -> Self {
        Self {
            label: label.into(),
            view,
        }
    }
}

impl<T, L> From<(T, L)> for SelectedViewLabel<T>
where
    T: Selectable,
    L: Into<Cow<'static, str>>,
{
    fn from((view, label): (T, L)) -> Self {
        Self::new(label, view)
    }
}

#[derive(Default, Debug)]
pub struct SelectedView<T: Selectable> {
    view: T,
    labels: Vec<SelectedViewLabel<T>>,
}

impl<T: Selectable> SelectedView<T> {
    pub fn new(view: T, labels: impl IntoIterator<Item = SelectedViewLabel<T>>) -> Self {
        Self {
            view,
            labels: labels.into_iter().collect(),
        }
    }

    pub fn show(self, children: impl FnOnce()) -> Response<SelectedViewResponse<T>> {
        SelectedViewWidget::show_children(children, self)
    }
}

pub struct SelectedViewResponse<T> {
    pub selected: T,
}

#[derive(Default, Debug)]
struct SelectedViewWidget<T: Selectable> {
    props: SelectedView<T>,
}

impl<T: Selectable> Widget for SelectedViewWidget<T> {
    type Props<'a> = SelectedView<T>;
    type Response = SelectedViewResponse<T>;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        let mut old = std::mem::replace(&mut self.props, props);

        max_height(1.0, || {
            List::row()
                .main_axis_alignment(MainAxisAlignment::SpaceBetween)
                .show(|| {
                    for repr in &old.labels {
                        let resp = mouse_area(|| {
                            if repr.view == old.view {
                                Label::new(&*repr.label).italic().fg(0xFF0000).show();
                            } else {
                                Label::new(&*repr.label).faint().show();
                            }
                        });

                        if resp.clicked {
                            self.props.view = repr.view;
                            old.view = repr.view;
                        }
                    }
                });
        });

        Self::Response { selected: old.view }
    }

    fn layout(&self, ctx: LayoutCtx, input: Constraints) -> Vec2f {
        self.default_layout(ctx, Constraints::tight(input.max))
    }
}

pub fn selected_view<T, L>(
    view: T,
    labels: impl IntoIterator<Item = L>,
    children: impl FnOnce(),
) -> Response<SelectedViewResponse<T>>
where
    T: Selectable,
    L: Into<SelectedViewLabel<T>>,
{
    SelectedView::new(view, labels.into_iter().map(|label| label.into())).show(children)
}
