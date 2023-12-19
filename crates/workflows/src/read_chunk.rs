use anyhow::Result;
use gamma_chess::proto::ChessDataSet;
use prost::Message;
use std::fs::File;
use std::io::Read;

fn main() -> Result<()> {
    let chunk_id = 0; // Replace with the desired chunk ID
    let chunk_file_path = format!("./dataset/chunks/{}.pb", chunk_id);

    // Open the chunked file for reading
    let mut chunk_file = File::open(&chunk_file_path)?;

    // Create a buffer to read the file contents into
    let mut buffer = Vec::new();
    chunk_file.read_to_end(&mut buffer)?;

    // Deserialize the chunk into ChessDataSet
    let dataset = deserialize_from_protobuf(&buffer)?;

    // Iterate over ChessGames within the dataset
    for (game_id, game) in dataset.games.iter().enumerate() {
        println!("Game {} Moves: {:?}", game_id, game.moves);
    }

    Ok(())
}

fn deserialize_from_protobuf(data: &[u8]) -> Result<ChessDataSet> {
    let dataset = ChessDataSet::decode(data)?;
    Ok(dataset)
}
