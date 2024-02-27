use std::sync::{Arc, Mutex, OnceLock};

use crate::widgets::{expanded, label, Label, List};
use shuten::{style::Rgb, Queue};

const MAX_LOG_ITEMS: usize = 200;
type LogQueue = Queue<LogItem, MAX_LOG_ITEMS>;
static LOGGER: OnceLock<Logger> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct LogItem {
    pub level: log::Level,
    pub timestamp: time::OffsetDateTime,
    pub target: Arc<str>,
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
        Label::new(repr).fg(color).underline().show();
    }

    pub fn show_timestamp(&self) {
        const FMT: &[time::format_description::FormatItem<'static>] =
            time::macros::format_description!("[hour]:[minute]:[second].[subsecond digits:4]");
        Label::new(self.timestamp.format(&FMT).unwrap())
            .italic()
            .faint()
            .show();
    }

    pub fn show_target(&self) {
        Label::new(&*self.target).bold().faint().show();
    }

    pub fn show_data(&self) {
        label(&*self.data);
    }

    pub fn show_all(&self) {
        List::row().spacing(1.0).show(|| {
            expanded(|| {
                self.show_data();
            });
            self.show_level();
            self.show_target();
            self.show_timestamp();
        });
    }
}

pub struct Logger {
    queue: Mutex<LogQueue>,
}

impl Logger {
    pub fn init() {
        let logger = LOGGER.get_or_init(|| Self {
            queue: Mutex::default(),
            // tx,
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
            target: Arc::from(record.target()),
            data: Arc::from(record.args().to_string()),
        };

        self.queue.lock().unwrap().push(item);
    }

    fn flush(&self) {}
}

// fn start_remote_log(rx: std::sync::mpsc::Receiver<LogItem>) {
//     std::thread::spawn(move || {
//         let mut queue = <Queue<LogItem, 100>>::new();
//         let listener = TcpListener::bind("127.0.0.1:52132").unwrap();

//         for mut incoming in listener.incoming().flatten() {
//             'inner: loop {
//                 let Ok(msg) = rx.recv() else { return };
//                 queue.push(msg);

//                 for msg in queue.drain(..) {
//                     let level = match msg.level {
//                         log::Level::Error => "error",
//                         log::Level::Warn => "warn",
//                         log::Level::Info => "info",
//                         log::Level::Debug => "debug",
//                         log::Level::Trace => "trace",
//                     };

//                     let json = serde_json::json! ({
//                         "level": level,
//                         "target": msg.target,
//                         "data": msg.data,
//                     });

//                     if incoming
//                         .write_all(&serde_json::to_vec(&json).unwrap())
//                         .and_then(|_| incoming.write_all(b"\n"))
//                         .and_then(|_| incoming.flush())
//                         .is_err()
//                     {
//                         break 'inner;
//                     }
//                 }
//             }
//         }
//     });
// }
