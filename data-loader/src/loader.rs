use anyhow::{anyhow, Result};
use pgn_reader::{BufferedReader, RawHeader, SanPlus, Skip, Visitor};
use shakmaty::uci::Uci;
use shakmaty::{san::San, Chess, Position};
use std::fs::File;
use std::io::BufRead;
use std::str::FromStr;
use std::{
    io::{BufReader, Seek, SeekFrom},
    mem,
};

struct MyVisitor {
    games: Vec<Vec<String>>, // Vector of games, each game is a vector of moves
    current_game_moves: Vec<String>, // Vector to store moves of the current game
    current_board: Chess,    // A board to track the current state
    max_games: usize,
}

impl MyVisitor {
    fn new(max_games: usize) -> Self {
        MyVisitor {
            games: Vec::with_capacity(max_games),
            current_game_moves: Vec::new(),
            current_board: Chess::default(),
            max_games,
        }
    }

    fn is_done(&self) -> bool {
        self.games.len() >= self.max_games
    }
}

impl Visitor for MyVisitor {
    type Result = ();

    fn begin_game(&mut self) {
        self.current_game_moves.clear();
        self.current_board = Chess::default();
    }

    fn san(&mut self, san_plus: SanPlus) {
        let san_str = san_plus.san.to_string();

        match San::from_str(&san_str) {
            Ok(san_move) => {
                // Convert SAN to a Move
                if let Ok(chess_move) = san_move.to_move(&self.current_board) {
                    let uci = Uci::from_standard(&chess_move);

                    // Store the move in UCI notation
                    self.current_game_moves.push(uci.to_string());

                    // Apply the move and update the board state
                    let new_board_res = self.current_board.clone().play(&chess_move);

                    if let Ok(new_board) = new_board_res {
                        self.current_board = new_board;
                    } else {
                        eprintln!("Failed to apply move to the board");
                    }
                } else {
                    eprintln!("Failed to convert SAN to move");
                }
            }
            Err(e) => {
                println!("the move: {}", san_str);
                eprintln!("Failed to parse SAN: {}", e);
            }
        }
    }

    fn header(&mut self, _key: &[u8], _value: RawHeader) {
        // Process PGN headers like Event, Site, Date, Round, White, Black, Result, etc.
    }

    // Called at the end of a game.
    fn end_game(&mut self) -> Self::Result {
        // Use `std::mem::take` to efficiently transfer moves without cloning
        // The std::mem::take function replaces self.current_game_moves with a new,
        // empty vector and returns the old vector. This old vector, which contains all the
        // moves of the completed game, is then pushed into the games vector.
        self.games.push(mem::take(&mut self.current_game_moves));
    }

    fn end_headers(&mut self) -> Skip {
        Skip(false) // Return false to continue processing the game, true to skip it.
    }
}

pub fn read_games_in_chunk(
    index_file_path: &str,
    pgn_file_path: &str,
    start_index: usize,
    chunk_size: usize,
) -> Result<Vec<Vec<String>>> {
    let index_file = File::open(index_file_path)?;
    let reader = BufReader::new(index_file);

    if let Some(offset_str_res) = reader.lines().nth(start_index) {
        match offset_str_res {
            Ok(offset_str) => {
                let offset = offset_str.parse::<u64>().expect("Failed to parse offset");

                let mut file = File::open(pgn_file_path)?;
                file.seek(SeekFrom::Start(offset))?;

                let mut buffered_reader = BufferedReader::new(file);
                let mut visitor = MyVisitor::new(chunk_size);

                while let Ok(game_read) = buffered_reader.read_game(&mut visitor) {
                    match game_read {
                        Some(_) => {
                            if visitor.is_done() {
                                break;
                            }
                        }
                        None => {
                            break;
                        }
                    }
                }

                Ok(visitor.games)
            }
            Err(e) => return Err(e.into()),
        }
    } else {
        return Err(anyhow!("Invalid start index.."));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Assuming the PGN and index files are located in a test_resources directory
    const PGN_FILE_PATH: &str = "../dataset/lichess_db_standard_rated_2016-05.pgn";
    const INDEX_FILE_PATH: &str = "../dataset/index.txt";

    #[test]
    fn test_read_games_in_chunk() {
        let start_index = 0;
        let chunk_size = 10; // Adjust this based on your dataset
        let games =
            read_games_in_chunk(INDEX_FILE_PATH, PGN_FILE_PATH, start_index, chunk_size).unwrap();

        assert_eq!(
            games.len(),
            chunk_size,
            "The number of games returned should match the chunk size."
        );
    }

    #[test]
    fn test_read_games_in_chunk_100() {
        let start_index = 0;
        let chunk_size = 100_000; // Adjust this based on your dataset

        let games =
            read_games_in_chunk(INDEX_FILE_PATH, PGN_FILE_PATH, start_index, chunk_size).unwrap();

        assert_eq!(
            games.len(),
            chunk_size,
            "The number of games returned should match the chunk size."
        );
    }
}
