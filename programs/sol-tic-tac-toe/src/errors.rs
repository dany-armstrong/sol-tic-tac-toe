use anchor_lang::prelude::*;

#[error_code]
pub enum TicTacToeError {
    GameAlreadyStarted,
    GameAlreadyOver,
    TileOutOfBounds,
    TileAlreadySet,
    NotPlayerTurn,
}