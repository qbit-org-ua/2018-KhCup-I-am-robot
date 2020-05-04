#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate failure;

#[cfg(test)]
#[macro_use]
extern crate quickcheck;

mod board;
mod common;
mod location;

pub use common::{BallColor, GameScore, GAME_BOARD_SIZE};
pub use board::GameBoard;
pub use location::{GameBoardLocation, GameBoardMove};
