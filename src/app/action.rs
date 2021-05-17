use crate::inputs::key::Key;

#[derive(Debug, Clone)]
pub struct Action {
    keys: Vec<Key>,
    help: String,
}

impl Action {
    pub fn new(help: &str, keys: Vec<Key>) -> Self {
        let help = help.to_string();
        Self { keys, help }
    }

    pub fn keys(&self) -> &[Key] {
        self.keys.as_slice()
    }

    pub fn help(&self) -> &str {
        self.help.as_str()
    }
}
