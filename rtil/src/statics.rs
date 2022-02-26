use std::sync::{Mutex, MutexGuard};
use std::ops::{Deref, DerefMut};
use std::fs::{File, OpenOptions};

macro_rules! log {
    () => {{
        use ::std::io::Write;
        let mut lock = ::statics::LOGFILE.lock().unwrap();
        writeln!(&mut lock, "").unwrap();
        lock.flush().unwrap();
    }};
    ($fmt:expr) => {{
        use ::std::io::Write;
        let mut lock = crate::statics::LOGFILE.lock().unwrap();
        writeln!(&mut lock, $fmt).unwrap();
        lock.flush().unwrap();
    }};
    ($fmt:expr, $($vars:tt)*) => {{
        use ::std::io::Write;
        let mut lock = crate::statics::LOGFILE.lock().unwrap();
        writeln!(&mut lock, $fmt, $($vars)*).unwrap();
        lock.flush().unwrap();
    }};
}

lazy_static::lazy_static! {
    pub static ref LOGFILE: Mutex<File> = {
        let mut path = ::std::env::temp_dir();
        path.push("refunct-tas.log");
        Mutex::new(OpenOptions::new()
        .create(true).write(true).truncate(true)
        .open(path).unwrap())
    };
}

pub struct Static<T> {
    val: Mutex<Option<T>>,
}

impl<T> Static<T> {
    pub fn new() -> Static<T> {
        Static {
            val: Mutex::new(None),
        }
    }

    pub fn set(&self, val: T) {
        *self.val.lock().unwrap() = Some(val);
    }
    
    pub fn get(&self) -> MutexGuardWrapper<T> {
        MutexGuardWrapper::new(self.val.lock().unwrap())
    }

    pub fn is_some(&self) -> bool {
        self.val.lock().unwrap().is_some()
    }

    pub fn is_none(&self) -> bool {
        !self.is_some()
    }
}

pub struct MutexGuardWrapper<'a, T: 'a> {
    guard: MutexGuard<'a, Option<T>>,
}

impl<'a, T> MutexGuardWrapper<'a, T> {
    fn new(guard: MutexGuard<'a, Option<T>>) -> MutexGuardWrapper<T> {
        MutexGuardWrapper {
            guard: guard,
        }
    }
}

impl<'a, T> Deref for MutexGuardWrapper<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.guard.as_ref().unwrap()
    }
}

impl<'a, T> DerefMut for MutexGuardWrapper<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.as_mut().unwrap()
    }
}
