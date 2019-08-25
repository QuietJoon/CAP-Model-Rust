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
