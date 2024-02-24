use std::cell::Cell;

use shuten::geom::{pos2f, remap, vec2f, Rectf};

use crate::{input::KeyEventKind, tree::current_tree, widget::prelude::*};

use super::{draggable, render_cell::render_cell};

#[derive(Default, Debug)]
pub struct Scrollable;
impl Scrollable {
    pub fn show(self, children: impl FnOnce()) -> Response<ScrollableResponse> {
        ScrollableWidget::show_children(children, self)
    }
}

type ScrollableResponse = ();

// TODO horizonal scroll
// TODO disable drag-to-drag
// TODO style configuration
#[derive(Default, Debug)]
struct ScrollableWidget {
    props: Scrollable,
    pos: usize,
    len: Cell<usize>,
    height: Cell<f32>,
}

impl ScrollableWidget {
    fn scroll_up(&mut self, delta: usize) {
        self.pos = self.pos.saturating_sub(delta);
    }

    fn scroll_down(&mut self, delta: usize) {
        self.pos += delta;
    }
}

impl Widget for ScrollableWidget {
    type Props<'a> = Scrollable;
    type Response = ScrollableResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;
        let len = current_tree().get_current().children.len();
        ScrollBar {
            pos: self.pos as f32,
            max: len as f32,
        }
        .show();
    }

    fn interest(&self) -> Interest {
        Interest::MOUSE_INSIDE | Interest::KEY_INPUT
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, mut input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let (scrollbar, children) = node.children.split_first().unwrap();

        self.len.set(children.len());
        // TODO calculate the scrollbar constraints (and adjust the input
        // constraints to subtract it)
        input.max.x -= 1.0;
        let constraints = Constraints::loose(input.max);

        let height = input.max.y as usize;
        let offset = self.pos;

        let mut size = Vec2f::ZERO;

        let offset = offset.min(children.len().saturating_sub(height));
        for &child in &children[..offset] {
            ctx.layout.hide(ctx.tree, child)
        }

        for &child in &children[offset..] {
            if size.y >= input.max.y {
                break;
            }
            let y = size.y;
            size += ctx.calculate(child, constraints);
            ctx.layout.set_pos(child, pos2f(0.0, y));
        }

        let size = input.max.min(size);

        ctx.calculate(
            *scrollbar,
            Constraints {
                min: size,
                max: input.max,
            },
        );

        self.height.set(size.y);
        size + vec2f(1.0, 0.0)
    }

    fn event(&mut self, _: EventCtx<'_>, event: &Event) -> Handled {
        let delta = match event {
            Event::MouseScroll {
                modifiers, delta, ..
            } => delta.y * if modifiers.is_ctrl() { 3.0 } else { 1.0 },
            Event::MouseDrag { delta, .. } => delta.y,
            Event::KeyChanged { key, .. } => match key.kind {
                KeyEventKind::Up => -1.0,
                KeyEventKind::Down => 1.0,
                KeyEventKind::PageUp => -self.height.get(),
                KeyEventKind::PageDown => self.height.get(),
                KeyEventKind::Home => -(self.len.get() as f32),
                KeyEventKind::End => self.len.get() as f32,
                _ => return Handled::Bubble,
            },
            _ => return Handled::Bubble,
        };

        if delta.is_sign_negative() {
            self.scroll_up(delta.abs() as _)
        } else {
            self.scroll_down(delta.abs() as _)
        }
        Handled::Sink
    }
}

#[derive(Debug, Default)]
struct ScrollBar {
    pos: f32,
    max: f32,
}

impl ScrollBar {
    fn show(self) -> Response<NoResponse> {
        ScrollBarWidget::show(self)
    }
}

#[derive(Debug, Default)]
struct ScrollBarWidget {
    props: ScrollBar,
    rect: Cell<Rectf>,
}

impl Widget for ScrollBarWidget {
    type Props<'a> = ScrollBar;
    type Response = NoResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;

        render_cell(shuten::Cell::new('│').fg(0xFF0000));
        let _resp = draggable(|| {
            render_cell(shuten::Cell::new('┃').fg(u32::MAX));
        });
    }

    fn layout(&self, mut ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        let node = ctx.tree.get_current();
        let size = vec2f(input.min.x.max(1.0), input.max.y);

        let &[track, knob] = &*node.children else {
            return size;
        };

        let constraints = Constraints::tight(vec2f(1.0, size.y));
        ctx.calculate(track, constraints);
        ctx.layout.set_pos(track, pos2f(size.x, 0.0));

        ctx.calculate(knob, Constraints::none());

        let p = remap(self.props.pos, 0.0..=self.props.max, 0.0..=(size.y - 1.0))
            .clamp(0.0, size.y - 1.0);
        let pos = pos2f(size.x, p);
        ctx.layout.set_pos(knob, pos);

        size
    }

    fn paint(&self, ctx: PaintCtx<'_, '_>) {
        self.rect.set(ctx.rect);
        self.default_paint(ctx);
    }
}

pub fn scrollable(children: impl FnOnce()) -> Response<ScrollableResponse> {
    Scrollable.show(children)
}
