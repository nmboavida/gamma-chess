import chess.pgn
from typing import List
from chess.pgn import Game

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
