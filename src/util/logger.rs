use log::info;
use log4rs;
use std::sync::OnceLock;

static INITIALIZED: OnceLock<bool> = OnceLock::new();

pub fn init_logger() {
    // static INITIALIZED: Arc<Mutex<AtomicBool>> = Arc::new(Mutex::new(AtomicBool::new(false)));
    let _ = INITIALIZED.get_or_init(|| {
        log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();

        info!("inited");
        return true;
    });
}
