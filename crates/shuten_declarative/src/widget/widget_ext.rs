use crate::{tree::current_tree, Response, Widget};

pub trait WidgetExt: Widget {
    fn show(props: Self::Props<'_>) -> Response<Self::Response>
    where
        Self: Sized,
    {
        current_tree().widget::<Self>(props)
    }

    fn show_children(children: impl FnOnce(), props: Self::Props<'_>) -> Response<Self::Response>
    where
        Self: Sized,
    {
        let tree = current_tree();
        let resp = tree.begin_widget::<Self>(props);
        children();
        tree.end_widget(resp.id());
        resp
    }
}

impl<T: Widget + Sized> WidgetExt for T {}
