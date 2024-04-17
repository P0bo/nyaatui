use std::{env, io::stdout};

use app::App;
use ratatui::{backend::CrosstermBackend, Terminal};

mod app;
mod client;
mod clip;
mod config;
mod macros;
mod source;
mod theme;
mod util;
mod widget;

#[tokio::main()]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Use real command line package
    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg == "--version" || arg == "-V" || arg == "-v" {
            println!(
                "nyaa v{}",
                option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN")
            );
            return Ok(());
        }
    }
    util::setup_terminal()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::default();

    app.run_app(&mut terminal).await?;

    util::reset_terminal()?;
    terminal.show_cursor()?;

    Ok(())
}
