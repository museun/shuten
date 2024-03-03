use crate::{Response, Ui, Widget, WidgetExt};

use super::Props;

pub trait PropsExt: Sized {
    fn show<W: for<'a> Widget<Props<'a> = Self>>(self, ui: &Ui) -> Response<W::Response> {
        <W as WidgetExt>::show(ui, self)
    }
    fn show_children<W: for<'a> Widget<Props<'a> = Self>, R>(
        self,
        ui: &Ui,
        show: impl FnOnce(&Ui) -> R,
    ) -> Response<W::Response> {
        <W as WidgetExt>::show_children(ui, self, show)
    }
}

impl<T: Props> PropsExt for T {}
