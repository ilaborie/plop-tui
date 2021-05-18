use std::sync::Arc;
use std::time::Duration;

use eyre::Result;
use log::{error, info};
use tokio::time::sleep;

use super::IoEvent;
use crate::app::App;

/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler<'a> {
    app: &'a Arc<tokio::sync::Mutex<App>>,
}

impl<'a> IoAsyncHandler<'a> {
    pub fn new(app: &'a Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Sleep(duration) => self.do_sleep(duration).await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        let mut app = self.app.lock().await;
        app.loaded();
    }

    /// We use dummy implementation here, just wait 1s
    async fn do_initialize(&mut self) -> Result<()> {
        info!("🚀 Initialize the application");
        let mut app = self.app.lock().await;
        sleep(Duration::from_secs(1)).await;
        app.initialize(); // we could update the app state
        info!("👍 Application initialized");

        Ok(())
    }

    /// Just take a little break
    async fn do_sleep(&mut self, duration: Duration) -> Result<()> {
        info!("😴 Go sleeping for {:?}...", duration);
        sleep(duration).await;
        info!("⏰ Wake up !");
        let mut app = self.app.lock().await;
        app.incr_sleep();

        Ok(())
    }
}
