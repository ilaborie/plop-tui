use std::sync::mpsc::Sender;
use std::time::Duration;

use log::{debug, error, warn};

use self::action::Actions;
use crate::app::action::Action;
use crate::inputs::key::Key;
use crate::io::IoEvent;

pub mod action;
pub mod ui;

/// The main application, containing the state
#[derive()]
pub struct App {
    /// We could dispatch an IO event
    io_tx: Sender<IoEvent>,
    /// Contextual actions
    actions: Actions,
    /// State
    is_loading: bool,
    duration: Duration,
    initialized: bool,
    counter_sleep: u32,
    counter_tick: u64,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>) -> Self {
        let is_loading = false;
        let duration = Duration::from_secs(1);
        let initialized = false;
        let counter_sleep = 0;
        let counter_tick = 0;
        let actions = vec![
            Action::Quit,
            Action::Sleep,
            Action::IncrementDelay,
            Action::DecrementDelay,
        ]
        .into();

        Self {
            io_tx,
            actions,
            is_loading,
            duration,
            initialized,
            counter_sleep,
            counter_tick,
        }
    }

    /// Check if the user want to exist the application
    pub fn should_quit(&self, key: Key) -> bool {
        if let Some(&action) = self.actions.find(key) {
            action == Action::Quit
        } else {
            false
        }
    }

    /// Handle a user action
    pub fn do_action(&mut self, key: Key) {
        if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);
            match action {
                // Sleep is an I/O action, we dispatch on the IO channel that's run on another thread
                Action::Sleep => self.dispatch(IoEvent::Sleep(self.duration)),
                // IncrementDelay and DecrementDelay is handled in the UI thread
                Action::IncrementDelay => self.set_sleep(self.duration.as_secs() + 1),
                // Note, that we clamp the duration, so we stay >= 0
                Action::DecrementDelay => self.set_sleep(self.duration.as_secs() - 1),
                // Nothing to do here for Quit, this is the special case handled into the main loop
                Action::Quit => {}
            }
        } else {
            warn!("No action accociated to {}", key)
        }
    }

    /// We could update the app or dispatch event on tick
    pub fn update_on_tick(&mut self) {
        // here we just increment a counter
        self.incr_tick();
    }

    /// Send a network event to the IO thread
    pub fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in handler.rs
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action) {
            self.is_loading = false;
            error!("Error from dispatch {}", e);
        };
    }

    // Some Getter / Setter ...

    pub fn incr_sleep(&mut self) {
        self.counter_sleep += 1;
    }

    pub fn incr_tick(&mut self) {
        self.counter_tick += 1;
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn count_sleep(&self) -> u32 {
        self.counter_sleep
    }

    pub fn count_tick(&self) -> u64 {
        self.counter_tick
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub fn duration(&self) -> &Duration {
        &self.duration
    }

    /// Set the duration, note that the duration is in 1s..10s
    pub fn set_sleep(&mut self, secs: u64) {
        self.duration = Duration::from_secs(secs.clamp(1, 10));
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }
}
