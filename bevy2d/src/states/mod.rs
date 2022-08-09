//! Game states

pub mod game;
pub mod gameover;
pub mod menu;
pub mod paused;

/// The game state
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    Game,
    Paused,
    GameOver,
}
