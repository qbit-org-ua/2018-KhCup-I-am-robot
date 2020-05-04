use std::collections::BinaryHeap;
use std::io::BufRead;

const GAME_BOARD_SIZE: usize = 9;

type ZoneId = u8;
type BallColor = u8;

#[derive(Debug, Copy, Clone)]
enum GameBoardCell {
    Empty(ZoneId),
    Ball(BallColor),
}

type GameBoard = [[GameBoardCell; GAME_BOARD_SIZE]; GAME_BOARD_SIZE];

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct LineCandidate {
    length: usize,
    zone_id: ZoneId,
    line_index: usize,
    row_index: usize,
    ball_color: BallColor,
}

fn mark_zones(game_board: &mut GameBoard, line_index: usize, row_index: usize, zone_id: ZoneId) {
    if let GameBoardCell::Empty(0) = game_board[line_index][row_index] {
        game_board[line_index][row_index] = GameBoardCell::Empty(zone_id);
        if line_index > 0 {
            mark_zones(game_board, line_index - 1, row_index, zone_id);
        }
        if line_index < GAME_BOARD_SIZE - 1 {
            mark_zones(game_board, line_index + 1, row_index, zone_id);
        }
        if row_index > 0 {
            mark_zones(game_board, line_index, row_index - 1, zone_id);
        }
        if row_index < GAME_BOARD_SIZE - 1 {
            mark_zones(game_board, line_index, row_index + 1, zone_id);
        }
    }
}

fn distance(a: usize, b: usize) -> usize {
    if a > b {
        a - b
    } else {
        b - a
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut game_board = [[GameBoardCell::Empty(0); GAME_BOARD_SIZE]; GAME_BOARD_SIZE];

    // Read Game Board
    for (line, game_board_line) in stdin.lock().lines().zip(game_board.iter_mut()) {
        for (character, game_board_cell) in line.unwrap()
            .split_whitespace()
            .zip(game_board_line.iter_mut())
        {
            *game_board_cell = match character.bytes().next().unwrap() {
                b'_' => GameBoardCell::Empty(0),
                b'R' => GameBoardCell::Ball(1),
                b'G' => GameBoardCell::Ball(2),
                b'B' => GameBoardCell::Ball(3),
                b'Y' => GameBoardCell::Ball(4),
                b'M' => GameBoardCell::Ball(5),
                b'P' => GameBoardCell::Ball(6),
                b'C' => GameBoardCell::Ball(7),
                c => panic!("Unknown Game Board symbol char({:?})'", c),
            };
        }
    }

    // Mark connected empty zones (DFS)
    let mut zone_id: ZoneId = 1;
    for line_index in 0..GAME_BOARD_SIZE {
        for row_index in 0..GAME_BOARD_SIZE {
            if let GameBoardCell::Empty(0) = game_board[line_index][row_index] {
                mark_zones(&mut game_board, line_index, row_index, zone_id);
                zone_id += 1;
            }
        }
    }

    // Find the longest lines
    let mut best_lines = BinaryHeap::new();
    for empty_zone_line_index in 0..GAME_BOARD_SIZE {
        for empty_zone_row_index in 0..GAME_BOARD_SIZE {
            if let GameBoardCell::Empty(current_zone_id) =
                game_board[empty_zone_line_index][empty_zone_row_index]
            {
                if empty_zone_row_index < GAME_BOARD_SIZE - 1 {
                    if let GameBoardCell::Ball(first_ball_color) =
                        game_board[empty_zone_line_index][empty_zone_row_index + 1]
                    {
                        let length = (empty_zone_row_index + 2..GAME_BOARD_SIZE)
                            .take_while(|row_index| {
                                if let GameBoardCell::Ball(ball_color) =
                                    game_board[empty_zone_line_index][*row_index]
                                {
                                    ball_color == first_ball_color
                                } else {
                                    false
                                }
                            })
                            .count() + 1;
                        best_lines.push(LineCandidate {
                            length,
                            zone_id: current_zone_id,
                            line_index: empty_zone_line_index,
                            row_index: empty_zone_row_index,
                            ball_color: first_ball_color,
                        });
                    }
                }
                if empty_zone_row_index < GAME_BOARD_SIZE - 1
                    && empty_zone_line_index < GAME_BOARD_SIZE - 1
                {
                    if let GameBoardCell::Ball(first_ball_color) =
                        game_board[empty_zone_line_index + 1][empty_zone_row_index + 1]
                    {
                        let length = (empty_zone_line_index + 2..GAME_BOARD_SIZE)
                            .zip(empty_zone_row_index + 2..GAME_BOARD_SIZE)
                            .take_while(|&(line_index, row_index)| {
                                if let GameBoardCell::Ball(ball_color) =
                                    game_board[line_index][row_index]
                                {
                                    ball_color == first_ball_color
                                } else {
                                    false
                                }
                            })
                            .count() + 1;
                        best_lines.push(LineCandidate {
                            length,
                            zone_id: current_zone_id,
                            line_index: empty_zone_line_index,
                            row_index: empty_zone_row_index,
                            ball_color: first_ball_color,
                        });
                    }
                }
                if empty_zone_line_index < GAME_BOARD_SIZE - 1 {
                    if let GameBoardCell::Ball(first_ball_color) =
                        game_board[empty_zone_line_index + 1][empty_zone_row_index]
                    {
                        let length = (empty_zone_line_index + 2..GAME_BOARD_SIZE)
                            .take_while(|&line_index| {
                                if let GameBoardCell::Ball(ball_color) =
                                    game_board[line_index][empty_zone_row_index]
                                {
                                    ball_color == first_ball_color
                                } else {
                                    false
                                }
                            })
                            .count() + 1;
                        best_lines.push(LineCandidate {
                            length,
                            zone_id: current_zone_id,
                            line_index: empty_zone_line_index,
                            row_index: empty_zone_row_index,
                            ball_color: first_ball_color,
                        });
                    }
                }
                if empty_zone_row_index > 0 {
                    if let GameBoardCell::Ball(first_ball_color) =
                        game_board[empty_zone_line_index][empty_zone_row_index - 1]
                    {
                        let length = if empty_zone_row_index > 1 {
                            (0..empty_zone_row_index - 2)
                                .rev()
                                .take_while(|row_index| {
                                    if let GameBoardCell::Ball(ball_color) =
                                        game_board[empty_zone_line_index][*row_index]
                                    {
                                        ball_color == first_ball_color
                                    } else {
                                        false
                                    }
                                })
                                .count()
                        } else {
                            0
                        } + 1;
                        best_lines.push(LineCandidate {
                            length,
                            zone_id: current_zone_id,
                            line_index: empty_zone_line_index,
                            row_index: empty_zone_row_index,
                            ball_color: first_ball_color,
                        });
                    }
                }
                if empty_zone_row_index > 0 && empty_zone_line_index > 0 {
                    if let GameBoardCell::Ball(first_ball_color) =
                        game_board[empty_zone_line_index - 1][empty_zone_row_index - 1]
                    {
                        let length = if empty_zone_line_index > 1 && empty_zone_row_index > 1 {
                            (0..empty_zone_line_index - 2)
                                .rev()
                                .zip((0..empty_zone_row_index - 2).rev())
                                .take_while(|&(line_index, row_index)| {
                                    if let GameBoardCell::Ball(ball_color) =
                                        game_board[line_index][row_index]
                                    {
                                        ball_color == first_ball_color
                                    } else {
                                        false
                                    }
                                })
                                .count()
                        } else {
                            0
                        } + 1;
                        best_lines.push(LineCandidate {
                            length,
                            zone_id: current_zone_id,
                            line_index: empty_zone_line_index,
                            row_index: empty_zone_row_index,
                            ball_color: first_ball_color,
                        });
                    }
                }
                if empty_zone_line_index > 0 {
                    if let GameBoardCell::Ball(first_ball_color) =
                        game_board[empty_zone_line_index - 1][empty_zone_row_index]
                    {
                        let length = if empty_zone_line_index > 1 {
                            (0..empty_zone_line_index - 2)
                                .rev()
                                .take_while(|&line_index| {
                                    if let GameBoardCell::Ball(ball_color) =
                                        game_board[line_index][empty_zone_row_index]
                                    {
                                        ball_color == first_ball_color
                                    } else {
                                        false
                                    }
                                })
                                .count()
                        } else {
                            0
                        } + 1;
                        best_lines.push(LineCandidate {
                            length,
                            zone_id: current_zone_id,
                            line_index: empty_zone_line_index,
                            row_index: empty_zone_row_index,
                            ball_color: first_ball_color,
                        });
                    }
                }
            }
        }
    }

    // Move a ball
    while best_lines.len() > 0 {
        let line_candidate = best_lines.pop().unwrap();
        for ball_line_index in 0..GAME_BOARD_SIZE {
            if ball_line_index == line_candidate.line_index {
                continue;
            }
            for ball_row_index in 0..GAME_BOARD_SIZE {
                if ball_row_index == line_candidate.row_index
                    || distance(ball_line_index, line_candidate.line_index)
                        == distance(ball_row_index, line_candidate.row_index)
                {
                    continue;
                }
                if let GameBoardCell::Ball(ball_color) = game_board[ball_line_index][ball_row_index]
                {
                    if ball_color == line_candidate.ball_color {
                        let has_common_zone = loop {
                            if ball_row_index > 0 {
                                if let GameBoardCell::Empty(zone_id) =
                                    game_board[ball_line_index][ball_row_index - 1]
                                {
                                    if zone_id == line_candidate.zone_id {
                                        break true;
                                    }
                                }
                            }
                            if ball_row_index < GAME_BOARD_SIZE - 1 {
                                if let GameBoardCell::Empty(zone_id) =
                                    game_board[ball_line_index][ball_row_index + 1]
                                {
                                    if zone_id == line_candidate.zone_id {
                                        break true;
                                    }
                                }
                            }
                            if ball_line_index > 0 {
                                if let GameBoardCell::Empty(zone_id) =
                                    game_board[ball_line_index - 1][ball_row_index]
                                {
                                    if zone_id == line_candidate.zone_id {
                                        break true;
                                    }
                                }
                            }
                            if ball_line_index < GAME_BOARD_SIZE - 1 {
                                if let GameBoardCell::Empty(zone_id) =
                                    game_board[ball_line_index + 1][ball_row_index]
                                {
                                    if zone_id == line_candidate.zone_id {
                                        break true;
                                    }
                                }
                            }
                            break false;
                        };
                        if has_common_zone {
                            println!(
                                "{} {} {} {}",
                                ball_row_index + 1,
                                ball_line_index + 1,
                                line_candidate.row_index + 1,
                                line_candidate.line_index + 1
                            );
                            return;
                        }
                    }
                }
            }
        }
    }
}
