use crate::{
    ndarray_to_tensor,
    proto::{ChessDataSet, ChessGame},
};
use anyhow::{anyhow, Result};
use ndarray::{s, Array2, Array4, ArrayViewMut1, ArrayViewMut3};
use prost::Message;
use shakmaty::{uci::Uci, Chess, Color, Move, Piece, Position, Role, Square};
use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};
use tch::{Device, Kind, Tensor};

pub struct Dataset {
    pub positions: Tensor,
    pub moves: Tensor,
}
// pub struct Dataset {
//     pub positions: Array4<f32>,
//     pub moves: Array2<f32>,
// }

impl Dataset {
    pub fn new(file_path: &str) -> Result<Self> {
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
        // let mut out_dataset = Dataset {
        //     positions: Array4::<f32>::zeros((0, 12, 8, 8)), // Initialize with an empty 4D array
        //     moves: Array2::<f32>::zeros((0, 4096)),         // Initialize with an empty 2D array
        // };

        // Process games
        out_dataset.process_games(in_dataset.games)?;
        Ok(out_dataset)
    }

    fn process_games(&mut self, games: Vec<ChessGame>) -> Result<()> {
        // Determine the total number of positions and moves
        let total_positions = games.iter().map(|g| g.moves.len()).sum();
        let total_moves = total_positions; // Assuming one move per position

        // Preallocate arrays with the total size
        let mut all_positions = Array4::<f32>::zeros((total_positions, 12, 8, 8));
        let mut all_moves = Array2::<f32>::zeros((total_moves, 4096));

        // Variables to keep track of where to fill in the preallocated arrays
        let mut position_index = 0;
        let mut move_index = 0;

        for game in games {
            let mut board = Chess::default();
            for uci_move_str in game.moves {
                let uci_move =
                    Uci::from_str(&uci_move_str).map_err(|_| anyhow!("Invalid UCI move string"))?;
                let chess_move = uci_move
                    .to_move(&board)
                    .map_err(|_| anyhow!("Invalid chess move"))?;

                if board.is_legal(&chess_move) {
                    // Fill the current board state into the preallocated array
                    Self::board_to_tensor(
                        &board,
                        &mut all_positions.slice_mut(s![position_index, .., .., ..]),
                    );
                    position_index += 1;

                    board.play_unchecked(&chess_move);

                    // Directly fill the move into the preallocated array
                    Self::move_to_tensor(chess_move, &mut all_moves.slice_mut(s![move_index, ..]))?;
                    move_index += 1;
                }
            }
        }

        // Assign the filled arrays to the dataset
        // self.positions = unsafe { ndarray_to_tensor(all_positions.into_dyn()) };
        // self.moves = unsafe { ndarray_to_tensor(all_moves.into_dyn()) };

        self.positions = Tensor::try_from(all_positions)?;
        self.moves = Tensor::try_from(all_moves)?;

        Ok(())
    }

    fn board_to_tensor(board: &Chess, board_slice: &mut ArrayViewMut3<f32>) {
        // Clear the slice (set all values to 0.0)
        board_slice.fill(0.0);

        // Mapping from piece type and color to index in the array
        let piece_idx = |piece: Piece| -> usize {
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
                let (row, col) = (square.rank() as usize, square.file() as usize);

                // Set the corresponding array value to 1
                board_slice[(index, row, col)] = 1.0;
            }
        }
    }

    fn move_to_tensor(chess_move: Move, move_slice: &mut ArrayViewMut1<f32>) -> Result<()> {
        // Initialize the slice to zeros
        move_slice.fill(0.0);

        // Calculate the index for the one-hot vector
        let source = chess_move.from().unwrap() as u8; // Convert from Square to 0-based index
        let destination = chess_move.to() as u8; // Convert from Square to 0-based index

        // Calculate the index in the one-hot vector
        let index = (source as usize)
            .checked_mul(64)
            .and_then(|x| x.checked_add(destination as usize))
            .ok_or(anyhow!("Arithmetic overflow occurred"))?;

        // Set the appropriate position to 1
        move_slice[index] = 1.0;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dataset_creation_ndarray_3() -> Result<()> {
        // Path to the test protobuf file
        let test_file_path = "../../dataset/chunks/0.pb";

        // Create the dataset
        let dataset = Dataset::new(test_file_path)?;

        // Check if the tensors are not empty
        // assert!(
        //     dataset.positions.size().iter().product::<i64>() > 0,
        //     "Positions tensor is empty"
        // );
        // assert!(
        //     dataset.moves.size().iter().product::<i64>() > 0,
        //     "Moves tensor is empty"
        // );

        // Optional: Check the shape of the tensors
        // e.g., assert_eq!(dataset.positions.size(), &[expected_size]);

        // Optional: Check for specific values if known
        // e.g., assert_eq!(dataset.positions.get(...), expected_value);

        Ok(())
    }
}
