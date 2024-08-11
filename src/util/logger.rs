use log4rs;

pub fn init_logger() {
    static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
    if INITIALIZED.load(std::sync::atomic::Ordering::Relaxed) {
        log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    }
}
