use std::env;
use std::iter;
use std::path::PathBuf;

extern crate env_logger;
#[macro_use]
extern crate log;
#[macro_use]
extern crate failure;
use failure::ResultExt;

extern crate lines_game_engine;
use lines_game_engine::{GameBoard, GameBoardMove, GameScore, GAME_BOARD_SIZE};

mod extensions;
use self::extensions::fs::read_utf8_with_limit;

#[derive(Debug)]
enum CheckerVerdict {
    OK,
    WrongAnswer(String),
    PresentationError(String),
}

#[derive(Debug)]
struct Config {
    test_input_filepath: PathBuf,
    test_answer_filepath: PathBuf,
    solution_output_filepath: PathBuf,
}

impl Config {
    fn from_args<I: iter::Iterator<Item = String>>(args: &mut I) -> Result<Self, failure::Error> {
        let test_input_filepath = args.next()
            .ok_or_else(|| {
                format_err!("The first argument should be a path to the test input file.")
            })?
            .into();
        let solution_output_filepath = args.next()
            .ok_or_else(|| {
                format_err!("The second argument should be a path to the solution output file.")
            })?
            .into();
        let test_answer_filepath = args.next()
            .ok_or_else(|| {
                format_err!("The third argument should be a path to the test answer file.")
            })?
            .into();
        Ok(Self {
            test_input_filepath,
            test_answer_filepath,
            solution_output_filepath,
        })
    }

    fn player_move(&self) -> Result<Option<GameBoardMove>, failure::Error> {
        let solution_output = read_utf8_with_limit(&self.solution_output_filepath, 1000)
            .context("The solution output file could not be read")?;
        Ok(if solution_output.is_empty() {
            None
        } else {
            Some(solution_output.parse::<GameBoardMove>().context("The game board move is invalid")?)
        })
    }

    fn game_state(&self) -> (GameBoard, GameScore) {
        let test_input = read_utf8_with_limit(&self.test_input_filepath, 1000)
            .expect("The test input file could not be read.");
        let mut test_input_lines = test_input.lines();
        (
            (&mut test_input_lines)
                .take(GAME_BOARD_SIZE)
                .collect::<Vec<&str>>()
                .join("\n")
                .parse::<GameBoard>()
                .expect("Game Board could not be parsed."),
            test_input_lines
                .skip(1)
                .next()
                .expect("Game Score is missing.")
                .parse::<GameScore>()
                .expect("Game Score could not be parsed"),
        )
    }

    fn expected_game_score(&self) -> GameScore {
        read_utf8_with_limit(&self.test_answer_filepath, 1000)
            .expect("The test answer file could not be read.")
            .trim_end()
            .parse::<GameScore>()
            .expect("The test answer file should be empty or contain an integer indicating the expected score.")
    }

    fn check_solution_output(&self) -> CheckerVerdict {
        let (mut game_board, mut game_score) = self.game_state();

        if !game_board.is_full() {
            let player_move = match self.player_move() {
                Ok(player_move) => player_move,
                Err(why) => {
                    return CheckerVerdict::PresentationError(why.to_string())
                }
            };
            if let Some(player_move) = player_move {
                let player_move_score = match game_board.move_ball(&player_move) {
                    Ok(score) => score,
                    Err(why) => {
                        return CheckerVerdict::WrongAnswer(format!(
                            "The move ({:?}) could not be made: {:?}",
                            player_move, why
                        ));
                    }
                };
                game_score += player_move_score;
            } else {
                return CheckerVerdict::WrongAnswer(
                    "The player did not make any move, but the board is not full yet.".into(),
                );
            }
        }

        let expected_game_score = self.expected_game_score();
            
        if game_score < expected_game_score {
            return CheckerVerdict::WrongAnswer(format!(
                "The expected game score ({}) is not reached ({})",
                expected_game_score, game_score
            ));
        }

        CheckerVerdict::OK
    }
}

fn main() {
    env_logger::Builder::from_default_env().parse("info").init();

    let checker = Config::from_args(&mut env::args().skip(1))
        .expect("Checker input arguments are not valid");

    std::process::exit(match checker.check_solution_output() {
        CheckerVerdict::OK => {
            info!("OK");
            0
        }
        CheckerVerdict::WrongAnswer(msg) => {
            info!("Wrong Answer due to {}", msg);
            1
        }
        CheckerVerdict::PresentationError(msg) => {
            info!("Presentation Error due to {}", msg);
            2
        }
    });
}
