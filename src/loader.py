from torch.utils.data import Dataset
import chess
import chess.pgn
from typing import List
from chess.pgn import Game

class ChessDataset(Dataset):
    def __init__(self, data_file, chunk_size: int):
        # Store file paths or indices to your data
        self.data_file = data_file

        # load the first chunk
        load_pgn_chunk()
        self.indexes = self._build_indexes(data_file)

    #def _build_indexes(self, data_file):
        # Implement this method to read the file and build an index 
        # that allows you to quickly locate individual games or positions in the file
        # ...

    def __len__(self):
        # Return the number of items in the dataset
        return len(self.indexes)

    def __getitem__(self, idx):
        # Load and process data for a single item
        game_data = self._load_data(self.indexes[idx])
        # Process the game data as needed, e.g., convert to tensor
        # ...

        return game_tensor, label_tensor

    #def _load_data(self, index):
        # Implement this method to load the data for a single game/position
        # based on the provided index
        # ...


def load_pgn_chunk(file_path: str, chunk_size: int, chunk_number: int) -> List[chess.pgn.Game]:
    """
    Loads a specified chunk of games from a PGN file.

    :param file_path: Path to the PGN file.
    :param chunk_size: Number of games to load in each chunk.
    :param chunk_number: The specific chunk to load (1-indexed).
    :return: List of games in the specified chunk.
    """
    games: List[chess.pgn.Game] = []
    start_game = (chunk_number - 1) * chunk_size

    with open(file_path) as pgn_file:
        # Skip games until the start of the desired chunk
        for _ in range(start_game):
            try:
                game = chess.pgn.read_game(pgn_file)
                if game is None:  # Reached end of file before the desired chunk
                    return []
            except Exception as e:
                print(f"Error skipping game: {e}")

        # Load games for the desired chunk
        for _ in range(chunk_size):
            try:
                game = chess.pgn.read_game(pgn_file)
                if game is None:  # Reached end of file
                    break
                games.append(game)
            except Exception as e:
                print(f"Error reading game: {e}")

    return games
