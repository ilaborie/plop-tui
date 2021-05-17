use std::cell::RefCell;
use std::io::stdout;
use std::rc::Rc;

use app::App;
use eyre::Result;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app::ui;

pub mod app;

#[allow(unreachable_code)]
pub fn start_ui(app: Rc<RefCell<App>>) -> Result<()> {
    // Configure Crossterm backend for tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let app = app.borrow();

        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;

        // TODO handle inputs here
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
