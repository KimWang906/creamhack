mod custom_widgets;
/// Module: dreamhack
///
/// This module is for handling Dreamhack API.
mod dreamhack;
mod termui;
use color_eyre::Result;
use termui::App;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}
