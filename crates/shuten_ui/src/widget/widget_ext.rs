use crate::{Response, Ui};

use super::Widget;

pub trait WidgetExt: Widget + Sized {
    fn show(ui: &Ui, props: Self::Props<'_>) -> Response<Self::Response> {
        ui.widget::<Self>(props)
    }

    // TODO return the closure value wrapped in the Response
    fn show_children<R>(
        ui: &Ui,
        props: Self::Props<'_>,
        children: impl FnOnce(&Ui) -> R,
    ) -> Response<Self::Response> {
        let resp = ui.begin_widget::<Self>(props);
        let _inner = children(ui);
        ui.end_widget(resp.id());
        resp
    }
}

impl<T: Widget> WidgetExt for T {}
