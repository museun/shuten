use shuten::event::MouseButton;

use crate::{
    context::{LayoutCtx, PaintCtx},
    geom::{Constraints, Vec2f},
    input::{Event, EventCtx, Handled, Interest},
    widget::Response,
    Widget, WidgetExt,
};

use super::label;

#[derive(Debug)]
pub struct TextInput {
    data: String,
    placeholder: String,
}

impl TextInput {
    pub fn placeholder(placeholder: &str) -> Self {
        Self {
            data: String::new(),
            placeholder: placeholder.to_string(),
        }
    }

    pub fn text(mut self, s: impl ToString) -> Self {
        self.data = s.to_string();
        self
    }

    pub fn show(self) -> Response<TextInputResponse> {
        TextInputWidget::show(self)
    }
}

#[derive(Debug)]
pub struct TextInputResponse {
    pub text: Option<String>,
    pub submit: bool,
}

#[derive(Debug)]
struct TextInputWidget {
    props: TextInput,
    diff: Option<String>,
    submit: bool,
}

impl Default for TextInputWidget {
    fn default() -> Self {
        Self {
            props: TextInput::placeholder("placeholder"),
            diff: Default::default(),
            submit: Default::default(),
        }
    }
}

impl TextInputWidget {
    fn delete(&mut self, _: i32) {}
}

impl Widget for TextInputWidget {
    type Props<'a> = TextInput;
    type Response = TextInputResponse;

    fn update(&mut self, props: Self::Props<'_>) -> Self::Response {
        self.props = props;

        let mut text = self.diff.as_ref().unwrap_or(&self.props.data);
        let use_placeholder = text.is_empty();
        if use_placeholder {
            text = &self.props.placeholder
        }

        label(text);

        Self::Response {
            text: self.diff.take(),
            submit: std::mem::take(&mut self.submit),
        }
    }

    fn paint(&self, mut ctx: PaintCtx<'_, '_>) {
        let node = ctx.tree.get_current();
        for &child in &node.children {
            ctx.paint(child)
        }
    }

    fn interest(&self) -> Interest {
        Interest::FOCUS_INPUT | Interest::MOUSE_INSIDE
    }

    fn event(&mut self, ctx: EventCtx<'_>, event: &Event) -> Handled {
        use crate::input::KeyEventKind as E;

        if let Event::MouseRelease {
            inside: true,
            button: MouseButton::Primary,
            ..
        } = event
        {
            ctx.input.set_selection(Some(ctx.tree.current()));
            return Handled::Sink;
        }

        if let Event::KeyChanged { key, modifiers: _ } = event {
            match key.kind {
                E::Char(ch) if ch.is_control() => {}

                E::Char(ch) => {
                    self.diff
                        .get_or_insert_with(|| self.props.data.clone())
                        .push(ch);
                    return Handled::Sink;
                }

                E::Backspace => {
                    self.delete(1);
                    return Handled::Sink;
                }

                E::Delete => {
                    self.delete(-1);
                    return Handled::Sink;
                }

                E::Enter => {
                    ctx.input.set_selection(None);
                    self.submit = true;
                    return Handled::Sink;
                }

                // TODO movement keys
                _ => {}
            }
        }

        Handled::Bubble
    }

    fn layout(&self, ctx: LayoutCtx<'_>, input: Constraints) -> Vec2f {
        ctx.layout.clip(ctx.tree);
        self.default_layout(ctx, input)
    }
}

pub fn text_input(placeholder: &str) -> Response<TextInputResponse> {
    TextInput::placeholder(placeholder).show()
}
