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
    skip_current_game: bool,
}

impl MyVisitor {
    fn new(max_games: usize) -> Self {
        MyVisitor {
            games: Vec::with_capacity(max_games),
            current_game_moves: Vec::new(),
            current_board: Chess::default(),
            max_games,
            skip_current_game: false,
        }
    }

    fn is_done(&self) -> bool {
        self.games.len() >= self.max_games
    }

    fn skip_game(&mut self) {
        self.current_game_moves.clear();
        self.skip_current_game = true;
    }
}

impl Visitor for MyVisitor {
    type Result = ();

    fn begin_game(&mut self) {
        self.current_game_moves.clear();
        self.current_board = Chess::default();
        self.skip_current_game = false;
    }

    fn san(&mut self, san_plus: SanPlus) {
        if self.skip_current_game {
            return;
        }

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
                        eprintln!(
                            "Skipping: Failed to apply move '{}' to the board: {} \n",
                            chess_move,
                            self.current_board.board()
                        );

                        self.skip_game();
                    }
                } else {
                    eprintln!(
                        "Skipping: Failed to convert SAN '{}' to the move. The board: {}",
                        san_str,
                        self.current_board.board(),
                    );

                    self.skip_game();
                }
            }
            Err(e) => {
                eprintln!("Skipping: Failed to parse SAN: {}", e);
                self.skip_game();
            }
        }
    }

    fn header(&mut self, _key: &[u8], _value: RawHeader) {
        // Process PGN headers like Event, Site, Date, Round, White, Black, Result, etc.
    }

    // Called at the end of a game.
    fn end_game(&mut self) -> Self::Result {
        if self.skip_current_game {
            return ();
        }

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
    use pgn_reader::BufferedReader;
    use std::io::Cursor;

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
    fn test_my_visitor() {
        let pgn_data = r#"
        [Event "Rated Classical game"]
        [Site "https://lichess.org/XDQeUk6j"]
        [White "davidtrolero395"]
        [Black "OleRedBeard"]
        [Result "0-1"]
        [UTCDate "2016.05.15"]
        [UTCTime "01:05:14"]
        [WhiteElo "1050"]
        [BlackElo "1297"]
        [WhiteRatingDiff "-10"]
        [BlackRatingDiff "+5"]
        [ECO "B01"]
        [Opening "Scandinavian Defense: Mieses-Kotroc Variation"]
        [TimeControl "600+0"]
        [Termination "Normal"]
        
        1. e4 d5 2. exd5 Qxd5 3. Bb5+ Qxb5 4. d4 b6 5. Qf3 Nc6 6. Ne2 Bb7 7. d5 Ne5 8. Qf4 f6 9. Qe4 Qxd5 10. Qxd5 Bxd5 11. Nd4 Bxg2 12. Rg1 Nf3+ 13. Nxf3 Bxf3 14. Rg3 Bd5 15. Rd3 Bc4 16. Rd1 e5 17. Be3 Bb4+ 18. Bd2 Ba5 19. Bxa5 bxa5 20. Na3 Bf7 21. Nb5 Rc8 22. Nxa7 Ra8 23. Nc6 Ne7 24. Nxe7 Kxe7 25. O-O-O Rad8 26. Rxd8 Kxd8 27. a4 e4 28. c4 Re8 29. c5 e3 30. fxe3 Rxe3 31. c6 f5 32. Rb1 f4 33. b4 f3 34. Rb3 Rxb3 35. bxa5 f2 36. a6 Rb1+ 37. Kxb1 f1=Q+ 38. Kb2 Qf2+ 39. Ka3 Qa2+ 40. Kb4 Qb2+ 41. Ka5 Qb6# 0-1        
        "#;

        // Create a cursor from the PGN data string
        let cursor = Cursor::new(pgn_data);

        // Create a BufferedReader from the cursor
        let mut buffered_reader = BufferedReader::new(cursor);

        // Create a MyVisitor instance for testing
        let mut visitor = MyVisitor::new(/* max_games */ 1);

        // Iterate through the PGN games using the visitor
        while let Ok(game_read) = buffered_reader.read_game(&mut visitor) {
            match game_read {
                Some(_) => {}
                None => {
                    break;
                }
            }
        }

        // Skipping: Failed to convert SAN 'O-O-O' to the move.
        // The board: r6r/2p1kbpp/5p2/p3p3/8/8/PPP2P1P/R2RK3
        // Therefore the length of the games vector should be zero
        assert_eq!(visitor.games.len(), 0);
    }
}
