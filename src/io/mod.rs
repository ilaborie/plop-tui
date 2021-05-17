use std::time::Duration;

pub mod network;

#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,
    Sleep(Duration),
    AddMessage(String),
}
