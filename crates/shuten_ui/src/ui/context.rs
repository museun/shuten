use crate::Ui;
use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<Option<Ui>> = const { RefCell::new(None) }
}

pub(crate) fn bind(ui: &Ui) {
    CONTEXT.with(move |current| {
        let mut current = current.borrow_mut();
        assert!(current.is_none(), "ui is already bound");
        *current = Some(ui.clone())
    });
}
pub(crate) fn unbind() {
    CONTEXT.with(move |current| {
        let mut current = current.borrow_mut();
        assert!(
            current.take().is_some(),
            "cannot unbind ui without it being bound"
        )
    });
}

pub fn current() -> Ui {
    CONTEXT.with(move |current| {
        current
            .borrow()
            .as_ref()
            .expect("cannot get access to without it being bound")
            .clone()
    })
}
