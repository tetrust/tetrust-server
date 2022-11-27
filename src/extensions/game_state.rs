use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub struct GameState {
    pub tx: broadcast::Sender<String>,
}
