use std::sync::Mutex;
use std::fs::{File, OpenOptions};
use once_cell::sync::Lazy;

macro_rules! log {
    () => {{
        use ::std::io::Write;
        let mut lock = ::statics::LOGFILE.lock().unwrap();
        writeln!(&mut lock, "").unwrap();
        lock.flush().unwrap();
    }};
    ($fmt:literal) => {{
        use ::std::io::Write;
        let mut lock = crate::log::LOGFILE.lock().unwrap();
        writeln!(&mut lock, $fmt).unwrap();
        lock.flush().unwrap();
    }};
    ($fmt:literal, $($vars:tt)*) => {{
        use ::std::io::Write;
        let buffer = format!($fmt, $($vars)*);
        let mut lock = crate::log::LOGFILE.lock().unwrap();
        writeln!(&mut lock, "{}", buffer).unwrap();
        lock.flush().unwrap();
    }};
}

pub static LOGFILE: Lazy<Mutex<File>> = Lazy::new(|| {
    let mut path = ::std::env::temp_dir();
    path.push("refunct-tas.log");
    Mutex::new(OpenOptions::new()
    .create(true).write(true).truncate(true)
    .open(path).unwrap())
});
