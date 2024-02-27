use crate::tree::Tree;
use std::cell::RefCell;

thread_local! {
    static CONTEXT: RefCell<Option<Tree>> = const { RefCell::new(None) };
}

pub fn bind(tree: &Tree) {
    CONTEXT.with(move |current| {
        let mut current = current.borrow_mut();
        assert!(current.is_none(), "tree is already in progress");
        *current = Some(tree.clone())
    })
}

pub fn unbind() {
    CONTEXT.with(|current| {
        let mut current = current.borrow_mut();
        assert!(
            current.take().is_some(),
            "cannot stop a tree without one existing"
        )
    })
}

#[profiling::function]
pub fn current_tree() -> Tree {
    CONTEXT.with(|c| {
        c.borrow()
            .as_ref()
            .expect("cannot get access to tree without one in progress")
            .clone()
    })
}
