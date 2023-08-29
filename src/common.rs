use log::debug;
use std::process::{self, Command};

/// Check that gsutil is installed
pub fn check_gsutil() {
    debug!("Checking gsutil is installed...");
    match Command::new("gsutil").arg("version").output() {
        Ok(_) => (),
        Err(_) => {
            println!(
                "Please install gsutil! See https://cloud.google.com/storage/docs/gsutil_install"
            );
            process::exit(2);
        }
    }
}
