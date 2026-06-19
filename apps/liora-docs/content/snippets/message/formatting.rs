//! Toast macros support standard format! arguments.

use liora_components::{toast_info, toast_success};

fn formatted_toasts() {
    let name = "Liora";
    let count = 4;
    toast_info!("{}, you have {} toast variants.", name, count);

    let component = "Message";
    let api = "toast_success!";
    toast_success!("{component} macro {api} works.");
}

fn main() {
    // In an app, call liora::init_liora(cx) before these macros run.
    let _ = formatted_toasts as fn();
}
