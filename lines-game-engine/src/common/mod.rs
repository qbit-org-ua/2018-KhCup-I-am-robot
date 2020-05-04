use std::fmt;

use failure;

pub const GAME_BOARD_SIZE: usize = 9;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BallColor {
    Green,
    Pink,
    Red,
    Maroon,
    Cyan,
    Blue,
    Yellow,
}

impl BallColor {
    pub fn to_char(&self) -> char {
        use self::BallColor::*;
        match *self {
            Green => 'G',
            Pink => 'P',
            Red => 'R',
            Maroon => 'M',
            Cyan => 'C',
            Blue => 'B',
            Yellow => 'Y',
        }
    }

    pub fn from_char(c: char) -> Result<Self, failure::Error> {
        use self::BallColor::*;
        Ok(match c {
            'G' => Green,
            'P' => Pink,
            'R' => Red,
            'M' => Maroon,
            'C' => Cyan,
            'B' => Blue,
            'Y' => Yellow,
            _ => {
                bail!("'{}' is not a supported color code", c);
            }
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Constructor, Into, Add, AddAssign,
         Display, FromStr)]
pub struct GameScore(u32);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct GameBoardCell(pub Option<BallColor>);

impl GameBoardCell {
    pub fn from_char(c: char) -> Result<Self, failure::Error> {
        match c {
            '_' => Ok(GameBoardCell(None)),
            _ => Ok(GameBoardCell(Some(BallColor::from_char(c)?))),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }
}

impl fmt::Display for GameBoardCell {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "{}",
            if let Some(ball_color) = self.0 {
                ball_color.to_char()
            } else {
                '_'
            }
        )?;
        Ok(())
    }
}
