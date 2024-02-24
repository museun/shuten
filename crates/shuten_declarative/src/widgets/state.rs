use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use crate::{
    context::LayoutCtx,
    geom::{Constraints, Vec2f},
    widget::{Response, Widget, WidgetExt as _},
};

struct State<T: 'static> {
    default: Box<dyn FnOnce() -> T>,
}

impl<T: 'static + std::fmt::Debug> std::fmt::Debug for State<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("State")
            .field("type", &std::any::type_name::<T>())
            .finish()
    }
}

impl<T: 'static + std::fmt::Debug> State<T> {
    fn new(default: impl FnOnce() -> T + 'static) -> Self {
        Self {
            default: Box::new(default),
        }
    }

    fn show(self) -> Response<StateResponse<T>> {
        <StateWidget<T>>::show(self)
    }
}

#[derive(Debug)]
struct StateWidget<T: 'static> {
    props: Option<Rc<RefCell<T>>>,
}

impl<T: 'static + std::fmt::Debug> Default for StateWidget<T> {
    fn default() -> Self {
        Self { props: None }
    }
}

impl<T: 'static + std::fmt::Debug> Widget for StateWidget<T> {
    type Props<'a> = State<T>;
    type Response = StateResponse<T>;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        let value = self
            .props
            .get_or_insert_with(|| {
                let default = (props.default)();
                Rc::new(RefCell::new(default))
            })
            .clone();

        StateResponse { value }
    }

    fn layout(&self, _: LayoutCtx<'_>, _: Constraints) -> Vec2f {
        Vec2f::ZERO
    }
}

pub struct StateResponse<T: 'static> {
    value: Rc<RefCell<T>>,
}

impl<T: 'static> StateResponse<T> {
    #[must_use]
    pub fn borrow(&self) -> Ref<'_, T> {
        self.value.borrow()
    }

    #[must_use]
    pub fn borrow_mut(&self) -> RefMut<'_, T> {
        self.value.borrow_mut()
    }

    pub fn set_if(&self, value: Option<T>) {
        if let Some(value) = value {
            self.set(value)
        }
    }

    pub fn set(&self, value: T) {
        self.value.replace(value);
    }
}

impl<T: Clone + 'static> StateResponse<T> {
    pub fn cloned(&self) -> T {
        self.borrow().clone()
    }
}

impl<T: Copy + 'static> StateResponse<T> {
    pub fn get(&self) -> T {
        *self.borrow()
    }

    pub fn modify(&self, update: impl FnOnce(T) -> T) {
        let mut value = self.borrow_mut();
        *value = update(*value);
    }
}

pub fn state<T: 'static + std::fmt::Debug>(
    default: impl FnOnce() -> T + 'static,
) -> Response<StateResponse<T>> {
    State::new(default).show()
}
