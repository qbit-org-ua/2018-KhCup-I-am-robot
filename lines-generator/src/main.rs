use std::env;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;

extern crate env_logger;
#[macro_use]
extern crate log;
extern crate rand;
use rand::SeedableRng;

extern crate lines_game_engine;
use lines_game_engine::{BallColor, GameBoard, GameBoardLocation, GameBoardMove, GameScore,
                        GAME_BOARD_SIZE};

type GameIteration = u32;

struct RandomGameBoardLocationIterator<T: rand::Rng + Clone> {
    rng: T,
}

impl<T: rand::Rng + Clone> RandomGameBoardLocationIterator<T> {
    fn new(rng: &T) -> Self {
        Self { rng: rng.clone() }
    }
}

impl<T: rand::Rng + Clone> std::iter::Iterator for RandomGameBoardLocationIterator<T> {
    type Item = GameBoardLocation;

    fn next(&mut self) -> Option<Self::Item> {
        Self::Item::from_offsets(
            self.rng.gen_range(0, GAME_BOARD_SIZE),
            self.rng.gen_range(0, GAME_BOARD_SIZE),
        ).ok()
    }
}

fn random_ball_color<T: rand::Rng>(rng: &mut T) -> BallColor {
    use BallColor::*;
    match rng.gen_range(0, 7) {
        0 => Green,
        1 => Pink,
        2 => Red,
        3 => Maroon,
        4 => Cyan,
        5 => Blue,
        6 => Yellow,
        x => panic!("Unexpected value ({}) for a ball color", x),
    }
}

fn load_shared_generator_context(
    shared_generator_context_filepath: &PathBuf,
) -> (GameIteration, GameScore, GameBoard) {
    if !shared_generator_context_filepath.exists() {
        (0, GameScore::new(0), GameBoard::default())
    } else {
        let mut shared_generator_context_str = String::new();
        fs::File::open(&shared_generator_context_filepath)
            .expect("Shared generator context file should be readable.")
            .read_to_string(&mut shared_generator_context_str)
            .expect("Shared generator context file should be readable.");
        let mut shared_generator_context_lines = shared_generator_context_str.lines();
        (
            shared_generator_context_lines
                .next()
                .expect("Unexpected EOF on Game Iteration value.")
                .parse::<GameIteration>()
                .expect("Game Iteration is expected to be an integer."),
            shared_generator_context_lines
                .next()
                .expect("Unexpected EOF on Game Score value.")
                .parse::<GameScore>()
                .expect("Game Score is expected to be an integer."),
            shared_generator_context_lines
                .take(GAME_BOARD_SIZE)
                .collect::<Vec<&str>>()
                .join("\n")
                .parse::<GameBoard>()
                .expect("Game Board is expected to parse fine."),
        )
    }
}

fn save_shared_generator_context(
    shared_generator_context_filepath: &PathBuf,
    game_iteration: GameIteration,
    game_score: GameScore,
    game_board: &GameBoard,
) {
    fs::File::create(shared_generator_context_filepath)
        .expect("Shared generator context file should be writeable.")
        .write_all(format!("{}\n{}\n{}", game_iteration, game_score, game_board).as_bytes())
        .expect("Shared generator context file should be writeable.");
}

fn main() {
    env_logger::init();

    let mut args = std::env::args().skip(1);
    let shared_generator_context_filepath: PathBuf = args.next()
        .expect(
            "The first argument to the generator should be a path to the \
             shared generator context file.",
        )
        .into();
    let previous_solution_output_filepath: PathBuf = args.next()
        .expect(
            "The second argument to the generator should be a path to the \
             previous solution output file.",
        )
        .into();
    let moves_per_game = env::var("MOVES_PER_GAME")
        .ok()
        .and_then(|str_value| str_value.parse::<GameIteration>().ok())
        .unwrap_or(25);

    let (game_iteration, mut game_score, mut game_board) =
        load_shared_generator_context(&shared_generator_context_filepath);
    let mut remaining_balls = if game_iteration % moves_per_game == 0 {
        5
    } else {
        3
    };

    if game_iteration % moves_per_game == 0 {
        game_board = GameBoard::default();
    } else {
        let mut previous_solution_output_is_valid = false;
        let mut previous_solution_output = String::new();
        if let Err(why) = fs::File::open(&previous_solution_output_filepath)
            .map(|mut file| file.read_to_string(&mut previous_solution_output))
        {
            info!(
                "Generator could not read the previous solution output file: {:?}",
                why
            );
        } else {
            match previous_solution_output.parse::<GameBoardMove>() {
                Ok(player_move) => {
                    if let Ok(score) = game_board.move_ball(&player_move) {
                        previous_solution_output_is_valid = true;
                        if score > GameScore::new(0) {
                            game_score += score;
                            remaining_balls = 0;
                        }
                    }
                }
                Err(why) => {
                    if game_board.is_full() {
                        previous_solution_output_is_valid = true;
                    } else {
                        info!(
                            "The previous player move was invalid ({:?}), so we \
                             fill the board with balls to indicate the game over.",
                            why
                        );
                    }
                }
            }
        }
        if !previous_solution_output_is_valid {
            let mut rng = rand::IsaacRng::new_unseeded();
            while !game_board.is_full() {
                for location in RandomGameBoardLocationIterator::new(&rng).take(100) {
                    game_board
                        .add_ball(&location, random_ball_color(&mut rng))
                        .ok();
                }
            }
            remaining_balls = 0;
        }
    }

    if remaining_balls > 0 {
        let mut rng = rand::IsaacRng::from_seed(&[game_iteration; 32]);
        for location in RandomGameBoardLocationIterator::new(&rng) {
            if let Ok(score) = game_board.add_ball(&location, random_ball_color(&mut rng)) {
                game_score += score;
                remaining_balls -= 1;
                if remaining_balls == 0 {
                    break;
                }
            } else if game_board.is_full() {
                break;
            }
        }
    }

    save_shared_generator_context(
        &shared_generator_context_filepath,
        game_iteration + 1,
        game_score,
        &game_board,
    );

    println!("{}\n{}", game_board, game_score);
}
