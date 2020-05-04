use std::fmt;
use std::marker::PhantomData;
use std::mem;
use std::str;

use failure;

use super::common::{BallColor, GameBoardCell, GameScore, GAME_BOARD_SIZE};
use super::location::{GameBoardLocation, GameBoardMove};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct GameBoard<'a> {
    board: [[GameBoardCell; GAME_BOARD_SIZE]; GAME_BOARD_SIZE],
    phantom: PhantomData<&'a i32>,
}

impl<'a> GameBoard<'a> {
    pub fn at(&'a self, location: &GameBoardLocation) -> &'a GameBoardCell {
        &self.board[location.y_offset()][location.x_offset()]
    }

    pub fn add_ball(
        &mut self,
        location: &GameBoardLocation,
        ball_color: BallColor,
    ) -> Result<GameScore, failure::Error> {
        ensure!(
            self.at(location).is_empty(),
            "The place {:?} is already taken by another ball.",
            location
        );
        self.board[location.y_offset()][location.x_offset()] = GameBoardCell(Some(ball_color));
        self.drop_lines_if_any(location)
    }

    pub fn move_ball(&mut self, game_move: &GameBoardMove) -> Result<GameScore, failure::Error> {
        ensure!(
            !self.at(&game_move.from).is_empty(),
            "There is no ball in {:?}",
            game_move.from
        );
        ensure!(
            self.at(&game_move.to).is_empty(),
            "The place {:?} is already taken by another ball.",
            game_move.to
        );
        let game_board_zones = self.connected_zones();
        debug_assert!(
            game_board_zones.at(&game_move.from).is_none(),
            "Sanity check failed! The 'from' location should not belong to any zone as it is taken by a ball."
        );
        debug_assert!(
            game_board_zones.at(&game_move.to).is_some(),
            "Sanity check failed! The 'to' location should belong to some zone."
        );
        let destination_zone_id = game_board_zones.at(&game_move.to);
        ensure!(
            game_move
                .from
                .neighbours()
                .any(
                    |neighbour_location| game_board_zones.at(&neighbour_location)
                        == destination_zone_id
                ),
            "There is no way to move the ball."
        );
        self.board[game_move.to.y_offset()][game_move.to.x_offset()] = mem::replace(
            &mut self.board[game_move.from.y_offset()][game_move.from.x_offset()],
            GameBoardCell(None),
        );
        self.drop_lines_if_any(&game_move.to)
    }

    fn drop_lines_if_any(
        &mut self,
        location: &GameBoardLocation,
    ) -> Result<GameScore, failure::Error> {
        let current_cell_copy = {
            let current_cell = self.at(location);
            if current_cell.is_empty() {
                return Ok(GameScore::new(0));
            }
            *current_cell
        };
        let mut number_of_dropped_balls = 0;
        for &(dx, dy) in &[(1, 0), (0, 1), (1, 1), (1, -1)] {
            let west_balls_count = location
                .walk(-dx, -dy)
                .skip(1)
                .take_while(|location| *self.at(location) == current_cell_copy)
                .count();
            let east_balls_count = location
                .walk(dx, dy)
                .skip(1)
                .take_while(|location| *self.at(location) == current_cell_copy)
                .count();
            let line_size = west_balls_count + east_balls_count + 1;
            if line_size >= 5 {
                number_of_dropped_balls += line_size;
                let mut line_start_location = *location;
                line_start_location
                    .update(
                        -dx * (west_balls_count as isize),
                        -dy * (west_balls_count as isize),
                    )
                    .expect("dx / dy manipulation should never fail.");
                for location in line_start_location.walk(dx, dy).take(line_size) {
                    self.board[location.y_offset()][location.x_offset()] = GameBoardCell(None);
                }
            }
        }
        Ok(GameScore::new(if number_of_dropped_balls < 5 {
            0
        } else {
            (1 << (number_of_dropped_balls - 4)) + 8
        }))
    }

    pub fn is_empty(&self) -> bool {
        self.board
            .iter()
            .all(|line| line.iter().all(|cell| cell.is_empty()))
    }

    pub fn is_full(&self) -> bool {
        self.board
            .iter()
            .all(|line| line.iter().all(|cell| !cell.is_empty()))
    }

    pub fn connected_zones(&self) -> GameBoardConnectedZones {
        GameBoardConnectedZones::new(self)
    }
}

impl<'a> Default for GameBoard<'a> {
    fn default() -> Self {
        Self {
            board: [[GameBoardCell(None); GAME_BOARD_SIZE]; GAME_BOARD_SIZE],
            phantom: PhantomData,
        }
    }
}

impl<'a> fmt::Display for GameBoard<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for line in &self.board {
            write!(fmt, "{}", line[0])?;
            for cell in line.iter().skip(1) {
                write!(fmt, " {}", cell)?;
            }
            write!(fmt, "\n")?;
        }
        Ok(())
    }
}

impl<'a> str::FromStr for GameBoard<'a> {
    type Err = failure::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game_board = Self::default();
        let mut lines_count = 0;
        for (line, location) in s.lines().zip(GameBoardLocation::zero().walk(0, 1)) {
            let mut line_chars = line.chars();
            for location in location.walk(1, 0) {
                let cell_char = line_chars
                    .next()
                    .ok_or_else(|| format_err!("Unexpected end of line."))?;
                game_board.board[location.y_offset()][location.x_offset()] =
                    GameBoardCell::from_char(cell_char)?;
                if location.x_offset() < GAME_BOARD_SIZE - 1 {
                    let space_char = line_chars
                        .next()
                        .ok_or_else(|| format_err!("Unexpected end of line."))?;
                    if !space_char.is_whitespace() {
                        bail!("Whitespace was expected but '{:?}' found", space_char);
                    }
                }
            }
            lines_count += 1;
        }
        if lines_count != GAME_BOARD_SIZE {
            bail!(
                "There are not enough / too many lines ({}) in the game board string.",
                lines_count
            );
        }
        Ok(game_board)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ConnectedZoneId(u8);

pub type ConnectedZoneCell = Option<ConnectedZoneId>;

#[derive(Debug)]
pub struct GameBoardConnectedZones([[ConnectedZoneCell; GAME_BOARD_SIZE]; GAME_BOARD_SIZE]);

impl GameBoardConnectedZones {
    pub fn new(game_board: &GameBoard) -> Self {
        let mut connected_zones =
            GameBoardConnectedZones([[None; GAME_BOARD_SIZE]; GAME_BOARD_SIZE]);
        let mut zone_id = ConnectedZoneId(1);
        for location in GameBoardLocation::zero().walk(0, 1) {
            for location in location.walk(1, 0) {
                if connected_zones.0[location.y_offset()][location.x_offset()].is_none() {
                    connected_zones.walk_and_mark(game_board, &location, zone_id);
                    zone_id.0 += 1;
                }
            }
        }
        connected_zones
    }

    fn walk_and_mark(
        &mut self,
        game_board: &GameBoard,
        current_location: &GameBoardLocation,
        zone_id: ConnectedZoneId,
    ) {
        if let Some(current_location_zone_id) = self.at(current_location) {
            assert_eq!(
                current_location_zone_id,
                zone_id,
                "Each cell can only belong to a single connection zones."
            );
            return;
        }
        if !game_board.at(current_location).is_empty() {
            return;
        }
        self.0[current_location.y_offset()][current_location.x_offset()] = Some(zone_id);
        for neighbour_location in current_location.neighbours() {
            self.walk_and_mark(game_board, &neighbour_location, zone_id);
        }
    }

    #[inline]
    pub fn at(&self, location: &GameBoardLocation) -> ConnectedZoneCell {
        self.0[location.y_offset()][location.x_offset()]
    }
}
