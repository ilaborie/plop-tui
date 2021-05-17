use std::sync::mpsc::Sender;
use std::time::Duration;

use lazy_static::lazy_static;
use log::error;

use self::action::Action;
use crate::inputs::key::Key;
use crate::io::IoEvent;

pub mod action;
pub mod ui;

lazy_static! {
    static ref QUIT: Action = Action::new("Quit", vec![Key::Ctrl('c'), Key::Char('q')]);
    static ref SLEEP: Action = Action::new("Go sleeping", vec![Key::Char('s')]);
}

#[derive()]
pub struct App {
    io_tx: Sender<IoEvent>,
    // State
    is_loading: bool,
    messages: Vec<String>,
    duration: Duration,
    initialized: bool,
    counter_sleep: u32,
    counter_tick: u64,

    actions: Vec<Action>,
}

impl App {
    pub fn new(io_tx: Sender<IoEvent>) -> Self {
        let is_loading = false;
        let messages = vec![];
        let duration = Duration::from_secs(1);
        let initialized = false;
        let counter_sleep = 0;
        let counter_tick = 0;
        let actions: Vec<Action> = vec![QUIT.clone(), SLEEP.clone()];

        Self {
            io_tx,
            is_loading,
            messages,
            duration,
            initialized,
            counter_sleep,
            counter_tick,
            actions,
        }
    }

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

    pub fn messages(&self) -> &[String] {
        self.messages.as_slice()
    }

    pub fn actions(&self) -> &[Action] {
        self.actions.as_slice()
    }

    pub fn set_sleep(&mut self, secs: u64) {
        self.duration = Duration::from_secs(secs);
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }

    pub fn do_sleep(&mut self) {
        self.dispatch(IoEvent::Sleep(self.duration))
    }

    pub fn do_add_message(&mut self, message: String) {
        if self.messages.len() > 9 {
            self.messages.remove(0);
        }
        self.messages.push(message);
    }

    pub fn update_on_tick(&mut self) {
        self.incr_tick();
    }

    // Send a network event to the network thread
    pub fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async action has finished in network.rs
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action) {
            self.is_loading = false;
            error!("Error from dispatch {}", e);
        };
    }
}
