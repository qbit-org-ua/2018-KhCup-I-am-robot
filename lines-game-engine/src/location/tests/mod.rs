use super::super::common::GAME_BOARD_SIZE;
use super::{GameBoardLocation, GameBoardMove};

#[test]
fn game_board_walker_can_walk() {
    let mut walker = GameBoardLocation::zero().walk(1, 0);
    for x in 0..GAME_BOARD_SIZE {
        assert_eq!(
            walker.next().unwrap(),
            GameBoardLocation::from_offsets(x, 0).unwrap()
        );
    }
    assert!(walker.next().is_none());
}

quickcheck! {
    fn game_board_location_from_offsets_is_valid(x_offset: usize, y_offset: usize) -> bool {
        let location = GameBoardLocation::from_offsets(x_offset, y_offset);
        if x_offset < GAME_BOARD_SIZE && y_offset < GAME_BOARD_SIZE {
            location.is_ok()
        } else {
            location.is_err()
        }
    }

    fn game_board_location_from_coords_is_valid(x: usize, y: usize) -> bool {
        let location = GameBoardLocation::from_coords(x, y);
        if x > 0 && x <= GAME_BOARD_SIZE && y > 0 && y <= GAME_BOARD_SIZE {
            location.is_ok()
        } else {
            location.is_err()
        }
    }

    fn game_board_move_from_str_is_valid(x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
        let game_board_move = format!("{} {} {} {}", x1, y1, x2, y2).parse::<GameBoardMove>();
        if (x1 > 0 && x1 <= GAME_BOARD_SIZE as isize && y1 > 0 && y1 <= GAME_BOARD_SIZE as isize) &&
           (x2 > 0 && x2 <= GAME_BOARD_SIZE as isize && y2 > 0 && y2 <= GAME_BOARD_SIZE as isize) {
            game_board_move.is_ok()
        } else {
            game_board_move.is_err()
        }
    }
}
