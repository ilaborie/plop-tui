use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread;

use eyre::Result;
use log::LevelFilter;
use plop_tui::app::App;
use plop_tui::io::handler::IoAsyncHandler;
use plop_tui::io::IoEvent;
use plop_tui::{start_tokio, start_ui};

#[tokio::main]
async fn main() -> Result<()> {
    // IO event should be handled to another thread that the UI thread
    let (sync_io_tx, sync_io_rx) = channel::<IoEvent>();

    // App is updated into the IO thread and the UI thread
    // See [Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
    // Note that's a Tokio Mutex
    let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);

    // Configue log with [tui_logger](https://github.com/gin66/tui-logger)
    // Log will be captured and send to a specfic widget
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    // Handle IO in a specifc thread
    thread::spawn(move || {
        let mut network = IoAsyncHandler::new(&app);
        start_tokio(sync_io_rx, &mut network);
    });

    start_ui(&app_ui).await?;

    Ok(())
}
