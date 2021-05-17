use std::sync::Arc;
use std::time::Duration;

use log::info;
use tokio::sync::Mutex;
use tokio::time::sleep;

use super::IoEvent;
use crate::app::App;

pub struct Network<'a> {
    app: &'a Arc<Mutex<App>>,
}

impl<'a> Network<'a> {
    pub fn new(app: &'a Arc<Mutex<App>>) -> Self {
        Self { app }
    }

    pub async fn handle_network_event(&mut self, io_event: IoEvent) {
        match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Sleep(duration) => self.do_sleep(duration).await,
            IoEvent::AddMessage(message) => self.do_add_message(message).await,
        };

        let mut app = self.app.lock().await;
        app.loaded();
    }

    async fn do_initialize(&mut self) {
        info!("🚀 Initialize the application");
        let mut app = self.app.lock().await;
        sleep(Duration::from_secs(1)).await;
        app.initialize();
        info!("👍 application started");
    }

    async fn do_sleep(&mut self, duration: Duration) {
        info!("😴 Go sleeping for {:?}...", duration);
        sleep(duration).await;
        info!("⏰  spleeping done.");
        let mut app = self.app.lock().await;
        app.incr_sleep();
    }

    async fn do_add_message(&mut self, message: String) {
        let mut app = self.app.lock().await;
        app.do_add_message(message);
    }
}
