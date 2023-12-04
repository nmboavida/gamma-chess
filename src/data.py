import torch
import chess
from torch.utils.data import Dataset

class ChessDataset(Dataset):
    def __init__(self, games):
        self.positions = torch.Tensor
        self.moves = torch.Tensor
        self.process_games(games)

    def process_games(self, games: list[list[str]]) -> None:
        pos = []
        movs = []
        
        for game in games:
            board = chess.Board()
            for move_san in game:
                try:
                    move = board.parse_san(move_san)
                    if board.is_legal(move):
                        # Add current board state to positions
                        pos.append(self.board_to_tensor(board))
                        # Make move and add to moves
                        movs.append(self.move_to_tensor(move))
                        board.push(move)
                except ValueError:
                    # Handle errors or illegal moves
                    break
        
        # Convert lists to PyTorch tensors
        self.positions = torch.stack(pos)
        self.moves = torch.stack(movs)
                    
    def board_to_tensor(self, board: chess.Board) -> torch.Tensor:
        # Initialize the tensor with shape (12, 8, 8)
        tensor = torch.zeros((12, 8, 8), dtype=torch.float)

        piece_idx = {'P': 0, 'N': 1, 'B': 2, 'R': 3, 'Q': 4, 'K': 5,
                     'p': 6, 'n': 7, 'b': 8, 'r': 9, 'q': 10, 'k': 11}

        for square in chess.SQUARES:
            piece = board.piece_at(square)
            if piece:
                # Get the piece type
                piece_type = piece.symbol()
                row, col = divmod(square, 8)
                # Populate the tensor
                tensor[piece_idx[piece_type], row, col] = 1

        return tensor


    def move_to_tensor(self, move: chess.Move) -> torch.Tensor:
        # Initialize a one-hot encoded tensor of length 4096
        one_hot_move = torch.zeros(64 * 64, dtype=torch.float)

        # Calculate the index for the one-hot vector
        source = move.from_square
        destination = move.to_square
        # We multiply the source by 64, given that for each source there are 64 possible moves
        # (the tensor dimensions assumes that a move can have the same source and destination - though
        # the network will learn that those moves are illegal)
        index = source * 64 + destination

        # Set the appropriate position to 1
        one_hot_move[index] = 1

        return one_hot_move

    def check_layout(self, index: int):
        ## Checking the positions input dataset
        game_layout = self.positions[index]

        for piece_type_coordinates in game_layout:
            print(piece_type_coordinates)

    def __len__(self):
        return len(self.positions)

    def __getitem__(self, idx):
        return self.positions[idx], self.moves[idx]