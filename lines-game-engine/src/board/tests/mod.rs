use super::super::common::{BallColor, GameScore, GAME_BOARD_SIZE};
use super::super::location::GameBoardLocation;
use super::{ConnectedZoneId, GameBoard};

#[test]
fn game_board_is_empty_on_creation() {
    let game_board = GameBoard::default();
    assert!(game_board.is_empty());
}

fn add_ball_expect_score(
    game_board: &mut GameBoard,
    x: usize,
    y: usize,
    ball_color: BallColor,
    expected_score: GameScore,
) {
    assert_eq!(
        game_board
            .add_ball(&GameBoardLocation::from_coords(x, y).unwrap(), ball_color)
            .unwrap(),
        expected_score
    );
}

fn add_ball_zero_score(game_board: &mut GameBoard, x: usize, y: usize, ball_color: BallColor) {
    add_ball_expect_score(game_board, x, y, ball_color, GameScore::new(0))
}

#[test]
fn game_board_can_be_completely_filled_with_balls() {
    let mut game_board = GameBoard::default();
    let colors = vec![BallColor::Red, BallColor::Green, BallColor::Blue];
    for y in 1..GAME_BOARD_SIZE + 1 {
        for x in 1..GAME_BOARD_SIZE + 1 {
            add_ball_zero_score(&mut game_board, x, y, colors[((x * 7 + y * 11) % 17) % 3]);
        }
    }
    assert!(game_board.is_full());
}

#[test]
fn game_board_should_drop_horizontal_lines() {
    let mut game_board = GameBoard::default();
    assert!(game_board.is_empty());
    add_ball_zero_score(&mut game_board, 1, 1, BallColor::Red);
    add_ball_zero_score(&mut game_board, 2, 1, BallColor::Red);
    add_ball_zero_score(&mut game_board, 3, 1, BallColor::Red);
    add_ball_zero_score(&mut game_board, 4, 1, BallColor::Red);
    add_ball_expect_score(&mut game_board, 5, 1, BallColor::Red, GameScore::new(10));
    assert!(game_board.is_empty());
}

#[test]
fn game_board_should_drop_vertical_lines() {
    let mut game_board = GameBoard::default();
    assert!(game_board.is_empty());
    add_ball_zero_score(&mut game_board, 2, 2, BallColor::Red);
    add_ball_zero_score(&mut game_board, 2, 3, BallColor::Red);
    add_ball_zero_score(&mut game_board, 2, 4, BallColor::Red);
    add_ball_zero_score(&mut game_board, 2, 5, BallColor::Red);
    add_ball_expect_score(&mut game_board, 2, 6, BallColor::Red, GameScore::new(10));
    assert!(game_board.is_empty());
}

#[test]
fn game_board_should_drop_main_diagonal_lines() {
    let mut game_board = GameBoard::default();
    assert!(game_board.is_empty());
    add_ball_zero_score(&mut game_board, 2, 2, BallColor::Red);
    add_ball_zero_score(&mut game_board, 3, 3, BallColor::Red);
    add_ball_zero_score(&mut game_board, 4, 4, BallColor::Red);
    add_ball_zero_score(&mut game_board, 5, 5, BallColor::Red);
    add_ball_expect_score(&mut game_board, 6, 6, BallColor::Red, GameScore::new(10));
    assert!(game_board.is_empty());
}

#[test]
fn game_board_should_drop_minor_diagonal_lines() {
    let mut game_board = GameBoard::default();
    assert!(game_board.is_empty());
    add_ball_zero_score(&mut game_board, 1, 5, BallColor::Red);
    add_ball_zero_score(&mut game_board, 2, 4, BallColor::Red);
    add_ball_zero_score(&mut game_board, 3, 3, BallColor::Red);
    add_ball_zero_score(&mut game_board, 4, 2, BallColor::Red);
    add_ball_expect_score(&mut game_board, 5, 1, BallColor::Red, GameScore::new(10));
    assert!(game_board.is_empty());
}

#[test]
fn game_board_should_drop_intersecting_lines() {
    let mut game_board = GameBoard::default();
    assert!(game_board.is_empty());
    add_ball_zero_score(&mut game_board, 7, 8, BallColor::Red);
    add_ball_zero_score(&mut game_board, 6, 8, BallColor::Red);
    add_ball_zero_score(&mut game_board, 5, 8, BallColor::Red);
    add_ball_zero_score(&mut game_board, 4, 8, BallColor::Red);
    add_ball_zero_score(&mut game_board, 8, 7, BallColor::Red);
    add_ball_zero_score(&mut game_board, 8, 6, BallColor::Red);
    add_ball_zero_score(&mut game_board, 8, 5, BallColor::Red);
    add_ball_zero_score(&mut game_board, 8, 4, BallColor::Red);
    add_ball_expect_score(&mut game_board, 8, 8, BallColor::Red, GameScore::new(72));
    assert!(game_board.is_empty());
}

#[test]
fn game_board_should_drop_only_same_color_lines() {
    let mut game_board = GameBoard::default();
    assert!(game_board.is_empty());
    add_ball_zero_score(&mut game_board, 2, 2, BallColor::Red);
    add_ball_zero_score(&mut game_board, 3, 2, BallColor::Red);
    add_ball_zero_score(&mut game_board, 4, 2, BallColor::Red);
    add_ball_zero_score(&mut game_board, 5, 2, BallColor::Red);
    add_ball_zero_score(&mut game_board, 6, 2, BallColor::Yellow);
    assert!(!game_board.is_empty());

    add_ball_expect_score(&mut game_board, 1, 2, BallColor::Red, GameScore::new(10));
    assert!(!game_board.is_empty());

    add_ball_zero_score(&mut game_board, 2, 2, BallColor::Yellow);
    add_ball_zero_score(&mut game_board, 3, 2, BallColor::Yellow);
    add_ball_zero_score(&mut game_board, 4, 2, BallColor::Yellow);
    add_ball_expect_score(&mut game_board, 5, 2, BallColor::Yellow, GameScore::new(10));
    assert!(game_board.is_empty());
}

#[test]
fn game_board_has_single_zone_on_empty_board() {
    let game_board_connected_zones = GameBoard::default().connected_zones();
    let first_zone_id = game_board_connected_zones.at(&GameBoardLocation::zero());
    assert!(first_zone_id.is_some());
    assert!(
        game_board_connected_zones
            .0
            .iter()
            .all(|line| line.iter().all(|&zone_id| zone_id == first_zone_id))
    );
}

#[test]
fn game_board_has_no_zones_when_completely_filled() {
    let mut game_board = GameBoard::default();
    let colors = vec![BallColor::Red, BallColor::Green, BallColor::Blue];
    for y in 1..GAME_BOARD_SIZE + 1 {
        for x in 1..GAME_BOARD_SIZE + 1 {
            add_ball_zero_score(&mut game_board, x, y, colors[((x * 7 + y * 11) % 17) % 3]);
        }
    }
    assert!(game_board.is_full());
    assert!(
        game_board
            .connected_zones()
            .0
            .iter()
            .all(|line| line.iter().all(|&zone_id| zone_id.is_none()))
    );
}

#[test]
fn game_board_has_several_zones_when_not_connected() {
    let mut game_board = GameBoard::default();
    add_ball_zero_score(&mut game_board, 2, 1, BallColor::Red);
    add_ball_zero_score(&mut game_board, 1, 2, BallColor::Red);
    let connected_zones = game_board.connected_zones();
    assert_eq!(connected_zones.0[0][0], Some(ConnectedZoneId(1)));
    assert_eq!(connected_zones.0[1][0], None);
    assert_eq!(connected_zones.0[2][0], Some(ConnectedZoneId(3)));
}

#[test]
fn game_board_can_be_loaded_from_empty_map_string() {
    use std::str::FromStr;
    let game_board = GameBoard::from_str(
        "\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         ",
    ).unwrap();
    assert!(game_board.is_empty());
}

#[test]
fn game_board_can_be_loaded_from_string_with_some_balls() {
    use std::str::FromStr;
    let game_board = GameBoard::from_str(
        "\
         R G B P M C Y _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         ",
    ).unwrap();
    assert!(!game_board.is_empty());
}

#[test]
fn game_board_from_str_should_fail_on_invalid_string_map_format() {
    use std::str::FromStr;
    let game_board = GameBoard::from_str("");
    assert!(game_board.is_err());

    let game_board = GameBoard::from_str("qwe");
    assert!(game_board.is_err());
}

#[test]
fn game_board_from_str_should_fail_on_incomplete_string_map_format() {
    use std::str::FromStr;
    let game_board = GameBoard::from_str(
        "\
         R G B P M C Y _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         ",
    );
    assert!(game_board.is_err());

    let game_board = GameBoard::from_str(
        "\
         R G B P M C Y _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         _ _ _ _ _ _ _ _ _\n\
         ",
    );
    assert!(game_board.is_err());
}
