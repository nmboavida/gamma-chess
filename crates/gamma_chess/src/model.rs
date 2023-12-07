use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

use crate::proto::{ChessDataSet, ChessGame};
use prost::Message;
use shakmaty::{uci::Uci, Chess, Color, Move, Piece, Position, Role, Square};
use tch::{Device, Kind, Tensor};

pub struct Dataset {
    pub positions: Tensor,
    pub moves: Tensor,
}

impl Dataset {
    pub fn new(file_path: &str) -> Self {
        // Load protobuf data
        let file = File::open(file_path).unwrap();
        let mut reader = BufReader::new(file);
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).unwrap();
        let in_dataset: ChessDataSet = ChessDataSet::decode(&*buf).unwrap();

        // Initialize ChessDataset with empty Tensors
        let mut out_dataset = Dataset {
            positions: Tensor::empty(&[0], (Kind::Float, Device::Cpu)), // Initialize with an empty Tensor
            moves: Tensor::empty(&[0], (Kind::Float, Device::Cpu)), // Initialize with an empty Tensor
        };

        // Process games
        out_dataset.process_games(in_dataset.games);

        out_dataset
    }

    fn process_games(&mut self, games: Vec<ChessGame>) {
        let mut positions = Vec::new();
        let mut moves = Vec::new();

        for game in games {
            let mut board = Chess::default();
            for uci_move_str in game.moves {
                let uci_move = match Uci::from_str(&uci_move_str) {
                    Ok(m) => m,
                    Err(_) => break, // Handle errors or illegal moves
                };

                let chess_move = match uci_move.to_move(&board) {
                    Ok(m) => m,
                    Err(_) => break, // Handle errors or illegal moves
                };

                if board.is_legal(&chess_move) {
                    // Add current board state to positions
                    positions.push(self.board_to_tensor(&board));

                    board.play_unchecked(&chess_move);

                    // Make move and add to moves
                    moves.push(Self::move_to_tensor(chess_move));
                }
            }
        }

        // Convert Vec<Tensor> to a stacked Tensor
        self.positions = Tensor::stack(&positions, 0);
        self.moves = Tensor::stack(&moves, 0);
    }

    fn board_to_tensor(&self, board: &Chess) -> Tensor {
        // Initialize the tensor with shape (12, 8, 8)
        let tensor = Tensor::zeros(&[12, 8, 8], (Kind::Float, Device::Cpu));

        // Mapping from piece type and color to index in the tensor
        let piece_idx = |piece: Piece| -> i64 {
            let base_index = match piece.role {
                Role::Pawn => 0,
                Role::Knight => 1,
                Role::Bishop => 2,
                Role::Rook => 3,
                Role::Queen => 4,
                Role::King => 5,
            };
            base_index + if piece.color == Color::White { 0 } else { 6 }
        };

        for square in Square::ALL.iter() {
            if let Some(piece) = board.board().piece_at(*square) {
                let index = piece_idx(piece);
                let (row, col) = (square.rank() as u8, square.file() as u8);

                // Set the corresponding tensor value to 1
                let _ = tensor.get(index).get(row as i64).get(col as i64).fill_(1.0);
            }
        }

        tensor
    }

    fn move_to_tensor(chess_move: Move) -> Tensor {
        // Initialize a one-hot encoded tensor of length 4096
        let one_hot_move = Tensor::zeros(&[4096], (Kind::Float, tch::Device::Cpu));

        // Calculate the index for the one-hot vector
        let source = chess_move.from().unwrap() as u8; // Convert from Square to 0-based index
        let destination = chess_move.to() as u8; // Convert from Square to 0-based index

        // Calculate the index in the one-hot vector
        let index = (source * 64 + destination) as i64; // Cast to i64 for indexing

        // Set the appropriate position to 1
        let _ = one_hot_move.get(index).fill_(1.0);

        one_hot_move
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_creation() {
        // Path to the test protobuf file
        let test_file_path = "../../dataset/chunks/0.pb";

        // Create the dataset
        let dataset = Dataset::new(test_file_path);

        // Check if the tensors are not empty
        assert!(
            dataset.positions.size().iter().product::<i64>() > 0,
            "Positions tensor is empty"
        );
        assert!(
            dataset.moves.size().iter().product::<i64>() > 0,
            "Moves tensor is empty"
        );

        // Optional: Check the shape of the tensors
        // e.g., assert_eq!(dataset.positions.size(), &[expected_size]);

        // Optional: Check for specific values if known
        // e.g., assert_eq!(dataset.positions.get(...), expected_value);
    }
}
