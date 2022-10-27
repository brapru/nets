mod app;
mod os;
mod ui;

use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tick_rate = Duration::from_millis(1000);
    ui::start_ui(tick_rate);

    Ok(())
}
