use shuten::Queue;

use crate::input::Keybind;
use crate::logger::{LogItem, Logger};

use crate::widget::prelude::*;
use crate::widgets::{container, scrollable::scrollable, toggle_bind};

#[derive(Debug)]
pub struct LogView;

impl LogView {
    pub fn show(self) -> Response<NoResponse> {
        LogViewWidget::show(self)
    }
}

#[derive(Debug)]
struct LogViewWidget {
    #[allow(dead_code)]
    logs: Queue<LogItem, 100>,
    start: time::OffsetDateTime,
}

impl Default for LogViewWidget {
    fn default() -> Self {
        Self {
            logs: Queue::new(),
            start: time::OffsetDateTime::UNIX_EPOCH,
        }
    }
}

impl Widget for LogViewWidget {
    type Props<'a> = LogView;
    type Response = NoResponse;

    fn update(&mut self, _: Self::Props<'_>) -> Self::Response {
        let logs = Logger::logs_since(self.start);
        if let Some(log) = logs.last() {
            self.start = log.timestamp;
        }
        self.logs.extend(logs);

        scrollable(|| {
            for log in &self.logs {
                log.show_all();
            }
        });
    }
}

pub fn log_view(keybind: Keybind) -> Response<NoResponse> {
    toggle_bind(keybind, || {
        container(0x292353, || {
            LogView.show();
        });
    })
    .map(drop)
}
