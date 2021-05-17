use log::warn;

use self::key::Key;
use crate::app::App;

pub mod events;
pub mod key;

pub enum InputEvent {
    /// An input event occurred.
    Input(Key),
    /// An tick event occurred.
    Tick,
}

pub fn handle_app(key: Key, app: &mut App) {
    match key {
        Key::Char('s') => app.do_sleep(),
        // FIXME handle edit field
        _ => {
            warn!("Key {} match no action", key);
        }
    }
}
