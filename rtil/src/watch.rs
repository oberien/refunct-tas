use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
}

struct Shared<T> {
    element: Mutex<Option<T>>,
    condvar: Condvar,
    tx_closed: AtomicBool,
    rx_closed: AtomicBool,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let shared = Arc::new(Shared {
        element: Mutex::new(None),
        condvar: Condvar::new(),
        tx_closed: AtomicBool::new(false),
        rx_closed: AtomicBool::new(false),
    });
    (Sender { shared: Arc::clone(&shared) }, Receiver { shared })
}

impl<T> Sender<T> {
    pub fn send(&self, element: T) -> Result<(), T> {
        if self.shared.rx_closed.load(Ordering::Relaxed) {
            return Err(element);
        }
        *self.shared.element.lock().unwrap() = Some(element);
        self.shared.condvar.notify_one();
        Ok(())
    }
}
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        self.shared.tx_closed.store(true, Ordering::Relaxed);
    }
}

impl<T> Receiver<T> {
    /// Consumes the element if there is one, otherwise waits for the next element
    pub fn read_consume(&self) -> Result<T, ()> {
        if self.shared.tx_closed.load(Ordering::Relaxed) {
            return Err(());
        }
        let mut lock = self.shared.element.lock().unwrap();
        loop {
            if let Some(element) = lock.take() {
                return Ok(element);
            } else {
                lock = self.shared.condvar.wait(lock).unwrap();
            }
        }
    }

    /// Consumes the element if there is one, otherwise waits for the next element
    pub fn _read_clone(&self) -> Result<T, ()> where T: Clone {
        if self.shared.tx_closed.load(Ordering::Relaxed) {
            return Err(());
        }
        let mut lock = self.shared.element.lock().unwrap();
        loop {
            if let Some(element) = lock.as_ref() {
                return Ok(element.clone());
            } else {
                lock = self.shared.condvar.wait(lock).unwrap();
            }
        }
    }
}
impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.shared.rx_closed.store(true, Ordering::Relaxed);
    }
}
