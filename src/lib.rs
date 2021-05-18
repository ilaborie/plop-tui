use std::io::stdout;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Duration;

use app::App;
use crossterm::terminal::enable_raw_mode;
use eyre::Result;
use io::handler::IoAsyncHandler;
use io::IoEvent;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;
use crate::inputs::events::Events;
use crate::inputs::InputEvent;

pub mod app;
mod inputs;
pub mod io;

/// Into the IO thread, handle IO events
#[tokio::main]
pub async fn start_tokio(io_rx: Receiver<IoEvent>, network: &mut IoAsyncHandler) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_io_event(io_event).await;
    }
}

/// Into the main (UI) thread, just a classic TUI application
pub async fn start_ui(app: &Arc<Mutex<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    // User event handler
    let tick_rate = Duration::from_millis(200);
    let events = Events::new(tick_rate);

    let mut is_first_render = true;
    loop {
        let mut app = app.lock().await;

        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;

        // Input
        match events.next()? {
            InputEvent::Input(key) => {
                // break the loop to finish the application
                if app.should_quit(key) {
                    break;
                }
                // OK process that event
                app.do_action(key);
            }
            InputEvent::Tick => app.update_on_tick(),
        }
        // Handle first render
        if is_first_render {
            // Here we assume the the first load is a long task
            app.dispatch(IoEvent::Initialize);
            is_first_render = false;
        }
    }

    // Restore the terminal and close application
    terminal.show_cursor()?;
    enable_raw_mode()?;

    Ok(())
}
