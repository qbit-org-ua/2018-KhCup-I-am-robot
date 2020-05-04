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

fn main() {
    let stdin = std::io::stdin();
    let mut game_board = [[GameBoardCell::Empty(0); GAME_BOARD_SIZE]; GAME_BOARD_SIZE];

    // Read Game Board
    for (line, game_board_line) in stdin.lock().lines().zip(game_board.iter_mut()) {
        for (character, game_board_cell) in line.unwrap().split_whitespace().zip(game_board_line.iter_mut()) {
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
    for line_index in 0 .. GAME_BOARD_SIZE {
        for row_index in 0 .. GAME_BOARD_SIZE {
            if let GameBoardCell::Empty(0) = game_board[line_index][row_index] {
                mark_zones(&mut game_board, line_index, row_index, zone_id);
                zone_id += 1;
            }
        }
    }

    // Move a "random" ball
    for i in 0 .. 1000 {
        let ball_line_index = ((i*i * 1113) + i) % GAME_BOARD_SIZE;
        let ball_row_index = ((i * 7331) + i % 11) % GAME_BOARD_SIZE;
        if let GameBoardCell::Ball(ball_color) = game_board[ball_line_index][ball_row_index] {
            if i % 2 == 0 {
                if ball_line_index > 0 {
                    if let GameBoardCell::Empty(neighbour_zone_id) = game_board[ball_line_index - 1][ball_row_index] {
                        for i in 0 .. 1000 {
                            let empty_zone_line_index = ((i*i * 7571) + i) % GAME_BOARD_SIZE;
                            let empty_zone_row_index = ((i * 5431) + i % 17) % GAME_BOARD_SIZE;
                            if let GameBoardCell::Empty(zone_id) = game_board[empty_zone_line_index][empty_zone_row_index] {
                                if zone_id == neighbour_zone_id {
                                    println!("{} {} {} {}", ball_row_index + 1, ball_line_index + 1, empty_zone_row_index + 1, empty_zone_line_index + 1);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{:?}", game_board);
}
