use std::cell::RefCell;
use std::rc::Rc;

use eyre::Result;
use plop_tui::app::App;
use plop_tui::start_ui;

fn main() -> Result<()> {
    let app = Rc::new(RefCell::new(App::new()));
    start_ui(app)?;
    Ok(())
}
