//! Alert with a detailed description.

use liora_components::{Alert, AlertType};

fn alert_description() -> Alert {
    // Add a secondary line when the message needs extra context.
    Alert::new("Warning")
        .alert_type(AlertType::Warning)
        .description("More detailed description of the warning.")
}

fn main() {
    let _ = alert_description();
}
