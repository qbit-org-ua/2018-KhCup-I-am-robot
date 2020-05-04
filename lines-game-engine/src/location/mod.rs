use std;

use failure;
use failure::ResultExt;

use super::common::GAME_BOARD_SIZE;

#[cfg(test)]
mod tests;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GameBoardLocation {
    x_offset: usize,
    y_offset: usize,
}

impl GameBoardLocation {
    #[inline]
    pub fn from_offsets(x_offset: usize, y_offset: usize) -> Result<Self, failure::Error> {
        ensure!(
            x_offset < GAME_BOARD_SIZE,
            "X offset ({}) should be less than the board size ({}).",
            x_offset,
            GAME_BOARD_SIZE
        );
        ensure!(
            y_offset < GAME_BOARD_SIZE,
            "Y offset ({}) should be less than the board size ({}).",
            y_offset,
            GAME_BOARD_SIZE
        );
        Ok(Self { x_offset, y_offset })
    }

    #[inline]
    pub fn from_coords(x: usize, y: usize) -> Result<Self, failure::Error> {
        ensure!(
            x > 0 && x <= GAME_BOARD_SIZE,
            "X ({}) should be in the range from 1 to {} (the board size).",
            x,
            GAME_BOARD_SIZE
        );
        ensure!(
            y > 0 && y <= GAME_BOARD_SIZE,
            "Y ({}) should be in the range from 1 to {} (the board size).",
            y,
            GAME_BOARD_SIZE
        );
        Self::from_offsets(x - 1, y - 1)
    }

    #[inline]
    pub fn zero() -> Self {
        Self {
            x_offset: 0,
            y_offset: 0,
        }
    }

    #[inline]
    pub fn x_offset(&self) -> usize {
        self.x_offset
    }

    #[inline]
    pub fn y_offset(&self) -> usize {
        self.y_offset
    }

    pub fn update(&mut self, dx: isize, dy: isize) -> Result<(), failure::Error> {
        let dx_u: usize;
        let dy_u: usize;
        if dx > 0 {
            dx_u = dx as usize;
            ensure!(
                dx_u < GAME_BOARD_SIZE,
                "dx ({}) should be less than the board size ({})",
                dx_u,
                GAME_BOARD_SIZE
            );
            ensure!(
                GAME_BOARD_SIZE - dx_u > self.x_offset,
                "We refuse to move (X ({}) + dx ({}) would be out of the board ({}))",
                self.x_offset,
                dx_u,
                GAME_BOARD_SIZE
            );
            self.x_offset += dx_u;
        } else if dx < 0 {
            dx_u = (-dx) as usize;
            ensure!(
                dx_u < GAME_BOARD_SIZE,
                "dx (-{}) should be less than the board size ({})",
                dx_u,
                GAME_BOARD_SIZE
            );
            ensure!(
                dx_u <= self.x_offset,
                "We refuse to move (X ({}) - dx ({}) would be out of the board ({}))",
                self.x_offset,
                dx_u,
                GAME_BOARD_SIZE
            );
            self.x_offset -= dx_u;
        }
        if dy > 0 {
            dy_u = dy as usize;
            ensure!(
                dy_u < GAME_BOARD_SIZE,
                "dy ({}) should be less than the board size ({})",
                dy_u,
                GAME_BOARD_SIZE
            );
            ensure!(
                GAME_BOARD_SIZE - dy_u > self.y_offset,
                "We refuse to move (Y ({}) + dy ({}) would be out of the board ({}))",
                self.y_offset,
                dy_u,
                GAME_BOARD_SIZE
            );
            self.y_offset += dy_u;
        } else if dy < 0 {
            dy_u = (-dy) as usize;
            ensure!(
                dy_u < GAME_BOARD_SIZE,
                "dy (-{}) should be less than the board size ({})",
                dy_u,
                GAME_BOARD_SIZE
            );
            ensure!(
                dy_u <= self.y_offset,
                "We refuse to move (Y ({}) - dy ({}) would be out of the board ({}))",
                self.y_offset,
                dy_u,
                GAME_BOARD_SIZE
            );
            self.y_offset -= dy_u;
        }
        Ok(())
    }

    pub fn walk(&self, dx: isize, dy: isize) -> GameBoardWalker {
        GameBoardWalker::new(*self, dx, dy)
    }

    pub fn neighbours(
        &self,
    ) -> std::iter::Chain<
        std::iter::Chain<
            std::iter::Chain<
                std::iter::Take<std::iter::Skip<GameBoardWalker>>,
                std::iter::Take<std::iter::Skip<GameBoardWalker>>,
            >,
            std::iter::Take<std::iter::Skip<GameBoardWalker>>,
        >,
        std::iter::Take<std::iter::Skip<GameBoardWalker>>,
    > {
        self.walk(-1, 0)
            .skip(1)
            .take(1)
            .chain(self.walk(1, 0).skip(1).take(1))
            .chain(self.walk(0, -1).skip(1).take(1))
            .chain(self.walk(0, 1).skip(1).take(1))
    }
}

#[derive(Debug)]
pub struct GameBoardMove {
    pub from: GameBoardLocation,
    pub to: GameBoardLocation,
}

impl std::str::FromStr for GameBoardMove {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s.split_whitespace()
            .map(|coordinate| coordinate.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()
            .context("Move coordinates are expected to be all positive integers.")?;
        ensure!(
            coords.len() == 4,
            "It is expected 4 coordinates to load Game Board Move, but {} found ({:?}).",
            coords.len(),
            coords
        );
        Ok(Self {
            from: GameBoardLocation::from_coords(coords[0], coords[1])
                .context("'From' location of the Game Board Move is not valid.")?,
            to: GameBoardLocation::from_coords(coords[2], coords[3])
                .context("'To' location of the Game Board Move is not valid.")?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct GameBoardWalker {
    next_location: Option<GameBoardLocation>,
    dx: isize,
    dy: isize,
}

impl GameBoardWalker {
    fn new(start_location: GameBoardLocation, dx: isize, dy: isize) -> Self {
        Self {
            next_location: Some(start_location),
            dx,
            dy,
        }
    }
}

impl Iterator for GameBoardWalker {
    type Item = GameBoardLocation;

    fn next(&mut self) -> Option<Self::Item> {
        let current_location = self.next_location;
        let mut is_edge_reached = false;
        if let Some(ref mut next_location) = self.next_location {
            is_edge_reached = next_location.update(self.dx, self.dy).is_err();
        }
        if is_edge_reached {
            self.next_location = None;
        }
        current_location
    }
}
