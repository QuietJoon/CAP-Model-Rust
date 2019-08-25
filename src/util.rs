use std::sync::{Arc, Mutex};
use std::sync::mpsc::*;
use std::thread;
use std::time::Duration;

#[macro_export]
macro_rules! sleep {
    ($e:expr) => {
        thread::sleep(Duration::from_millis($e));
    };
}

pub fn send_until_success<T: Clone>(unit: T, tx: Sender<T>) {
    loop {
        if tx.send(unit.clone()).is_ok() {
            break;
        }
        // TODO: Find an appropriate value
        sleep!(1);
    }
}

pub fn recv_until_success<T>(rx: Receiver<T>) -> T {
    loop {
        let res = rx.recv();
        if res.is_ok() {
            return res.unwrap();
        }
        // TODO: Find an appropriate value
        sleep!(1);
    }
}

pub fn assign_until_success<T>(unit: T, amot: Arc<Mutex<Option<T>>>) {
    let mut datum = amot.lock().unwrap();
    *datum = Some(unit);
}

pub fn read_until_success<T: Clone>(amot: Arc<Mutex<Option<T>>>) -> T {
    loop {
        let datum = amot.try_lock();
        if datum.is_err() {
            sleep!(1);
        } else {
            let mg_datum = datum.unwrap();

            if mg_datum.is_some() {
                return mg_datum.unwrap();
            } else {
                sleep!(1);
            }
        }
    }
}
