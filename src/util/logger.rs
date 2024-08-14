use log4rs;
use std::sync::atomic::{AtomicBool, Ordering};

pub fn init_logger() {
    static INITIALIZED: AtomicBool = AtomicBool::new(false);
    if !INITIALIZED.load(Ordering::Relaxed) {
        log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
        INITIALIZED.store(true, Ordering::Relaxed);
    }
}
