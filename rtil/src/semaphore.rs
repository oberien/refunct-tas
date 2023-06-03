use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    mutex: Mutex<i64>,
    condvar: Condvar,
}

impl Semaphore {
    pub const fn new(count: i64) -> Semaphore {
        Semaphore {
            mutex: Mutex::new(count),
            condvar: Condvar::new(),
        }
    }

    pub fn try_acquire(&self) -> bool {
        let mut guard = self.mutex.lock().unwrap();
        if *guard <= 0 {
            false
        } else {
            *guard -= 1;
            true
        }
    }

    pub fn _acquire(&self) {
        let mut guard = self.mutex.lock().unwrap();
        if *guard <= 0 {
            guard = self.condvar.wait_while(guard, |count| *count <= 0).unwrap();
        }
        *guard -= 1;
    }

    pub fn release(&self) {
        *self.mutex.lock().unwrap() += 1;
        self.condvar.notify_one();
    }
}
