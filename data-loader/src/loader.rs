use std::mem;

use chess::{Board, ChessMove, Color, File, Piece, Rank, Square};
use pgn_reader::{BufferedReader, RawHeader, SanPlus, Skip, Visitor};

struct MyVisitor {
    games: Vec<Vec<String>>, // Vector of games, each game is a vector of moves
    current_game_moves: Vec<String>, // Vector to store moves of the current game
}

impl Visitor for MyVisitor {
    type Result = ();

    fn begin_game(&mut self) {
        self.current_game_moves.clear()
    }

    fn san(&mut self, san_plus: SanPlus) {
        // self.moves.push(san_plus.san.to_string()); // Store the move
        self.current_game_moves.push(san_plus.san.to_string());
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

fn main() {
    let pgn_data = std::fs::read_to_string("path/to/your/game.pgn").expect("Failed to read file");

    let mut reader = BufferedReader::new_cursor(&pgn_data);
    let mut visitor = MyVisitor {
        games: Vec::new(),
        current_game_moves: Vec::new(),
    };

    while let Ok(_) = reader.read_game(&mut visitor) {
        // Each game is processed in turn
    }

    // Now, visitor.games contains the moves of all games
    for (game_index, game) in visitor.games.iter().enumerate() {
        println!("Game {}: {:?}", game_index + 1, game);
    }
}
