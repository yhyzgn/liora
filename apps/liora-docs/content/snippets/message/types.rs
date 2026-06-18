//! Toast variants backed by the global Message manager.

use liora_components::{toast_error, toast_info, toast_success, toast_warning};

fn show_toast_types() {
    toast_info!("This is an info toast");
    toast_success!("Operation completed");
    toast_warning!("Please check the input");
    toast_error!("Operation failed");
}

fn main() {
    // In an app, call MessageManager::init(cx) before these macros run.
    let _ = show_toast_types as fn();
}
