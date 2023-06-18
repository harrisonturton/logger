use log::{info, debug, trace, error, warn};

fn main() {
    logger::init().unwrap();

    info!("Information");
    debug!("Debug log");
    trace!("This is a trace");
    error!("This is an error");
    warn!("Warning for something");
}
