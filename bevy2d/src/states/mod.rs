//! Game states

pub mod game;
pub mod gameover;
pub mod menu;

/// The game state
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    Game,
    GameOver,
}
