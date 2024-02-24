use std::sync::{Arc, Mutex, OnceLock};

use shuten::{style::Rgb, Queue};

use crate::widgets::{label, Label, List};

const MAX_LOG_ITEMS: usize = 200;
type LogQueue = Queue<LogItem, MAX_LOG_ITEMS>;

static LOGGER: OnceLock<Logger> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct LogItem {
    pub level: log::Level,
    pub timestamp: time::OffsetDateTime,
    pub data: Arc<str>,
}

impl LogItem {
    pub fn show_level(&self) {
        let (repr, color) = match self.level {
            log::Level::Error => ("Error", Rgb::from_u32(0xFF0000)),
            log::Level::Warn => ("Warn", Rgb::from_u32(0xFFFF00)),
            log::Level::Info => ("Info", Rgb::from_u32(0xFF00FF)),
            log::Level::Debug => ("Debug", Rgb::from_u32(0x00FFFF)),
            log::Level::Trace => ("Trace", Rgb::from_u32(0xFFFFFF)),
        };
        Label::new(repr).fg(color).show();
    }

    pub fn show_timestamp(&self) {
        const FMT: &[time::format_description::FormatItem<'static>] =
            time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:4]");
        label(self.timestamp.format(&FMT).unwrap());
    }

    pub fn show_data(&self) {
        label(&*self.data);
    }

    pub fn show_all(&self) {
        List::row().spacing(1.0).show(|| {
            self.show_timestamp();
            self.show_level();
            self.show_data();
        });
    }
}

pub struct Logger {
    queue: Mutex<LogQueue>,
    log_to_stderr: bool,
}

impl Logger {
    pub fn init(log_to_stderr: bool) {
        let logger = LOGGER.get_or_init(|| Self {
            queue: Mutex::default(),
            log_to_stderr,
        });
        log::set_logger(logger as _).expect("single initialization");
        log::set_max_level(log::LevelFilter::Trace)
    }

    pub fn all_logs() -> Vec<LogItem> {
        let logger = LOGGER.get().unwrap();
        let queue = logger.queue.lock().unwrap();
        queue.iter().cloned().collect()
    }

    pub fn logs_since(dt: time::OffsetDateTime) -> Vec<LogItem> {
        let logger = LOGGER.get().unwrap();
        let queue = logger.queue.lock().unwrap();
        let pos = queue
            .iter()
            .rposition(|k| k.timestamp <= dt)
            .map(|s| s + 1)
            .unwrap_or(0);
        queue.iter().skip(pos).cloned().collect()
    }

    pub fn clear_logs() {
        LOGGER.get().unwrap().queue.lock().unwrap().clear()
    }
}

impl log::Log for Logger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        static THIS: &str = concat!(env!("CARGO_PKG_NAME",), "::");
        metadata.target().starts_with(THIS)
    }

    fn log(&self, record: &log::Record) {
        let item = LogItem {
            level: record.level(),
            timestamp: time::OffsetDateTime::now_utc(),
            data: Arc::from(record.args().to_string()),
        };

        if self.log_to_stderr {
            use std::io::Write as _;
            let _ = writeln!(
                &mut std::io::stderr(),
                "{level}: {data}",
                level = record.level(),
                data = record.args()
            );
        }

        self.queue.lock().unwrap().push(item);
    }

    fn flush(&self) {}
}
