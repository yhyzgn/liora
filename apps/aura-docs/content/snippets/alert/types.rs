//! Alert semantic types.

use aura_components::{Alert, AlertType, Space};

fn alert_types() -> Space {
    // Use semantic alert types to map the same layout to different theme tokens.
    Space::new()
        .vertical()
        .gap_md()
        .child(Alert::new("Info Alert").alert_type(AlertType::Info))
        .child(Alert::new("Success Alert").alert_type(AlertType::Success))
        .child(Alert::new("Warning Alert").alert_type(AlertType::Warning))
        .child(Alert::new("Error Alert").alert_type(AlertType::Error))
}

fn main() {
    let _ = alert_types();
}
