use std::sync::mpsc::channel;
use std::sync::Arc;

use chrono::Local;
use eyre::Result;
use plop_tui::app::App;
use plop_tui::io::network::Network;
use plop_tui::io::IoEvent;
use plop_tui::{start_tokio, start_ui};
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<()> {
    let (log_tx, log_rx) = channel::<String>();
    let (sync_io_tx, sync_io_rx) = channel::<IoEvent>();

    let app = Arc::new(Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);

    // Configue log
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{} [{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(log_tx)
        .apply()?;

    // Handle IO
    std::thread::spawn(move || {
        let mut network = Network::new(&app);
        start_tokio(sync_io_rx, &mut network);
    });

    // Handle Logs
    std::thread::spawn(move || {
        while let Ok(message) = log_rx.recv() {
            if let Err(err) = sync_io_tx.send(IoEvent::AddMessage(message)) {
                eprintln!("{:?}", err)
            }
        }
    });

    start_ui(&app_ui).await?;

    Ok(())
}
