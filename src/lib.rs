use std::io::stdout;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::time::Duration;

use app::App;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use eyre::Result;
use io::network::Network;
use io::IoEvent;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;
use crate::inputs::events::Events;
use crate::inputs::{handle_app, InputEvent};

pub mod app;
mod inputs;
pub mod io;

#[tokio::main]
pub async fn start_tokio(io_rx: Receiver<IoEvent>, network: &mut Network) {
    while let Ok(io_event) = io_rx.recv() {
        network.handle_network_event(io_event).await;
    }
}

pub async fn start_ui(app: &Arc<Mutex<App>>) -> Result<()> {
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

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
                if key.is_exit() {
                    break;
                }
                handle_app(key, &mut app);
            }
            InputEvent::Tick => {
                app.update_on_tick();
            }
        }
        // Handle first render
        if is_first_render {
            app.dispatch(IoEvent::Initialize);
            is_first_render = false;
        }
    }

    terminal.show_cursor()?;
    close_application()?;

    Ok(())
}

fn close_application() -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
