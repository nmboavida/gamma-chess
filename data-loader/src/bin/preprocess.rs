use anyhow::Result;
use data_loader::loader::read_games_in_chunk;
use data_loader::proto::{ChessDataSet, ChessGame};
use indicatif::{ProgressBar, ProgressStyle};
use prost::Message;
use rayon::prelude::*;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, RwLock};

fn main() -> Result<()> {
    // Create chunks folder if it does not exist yet
    let chunks_dir = Path::new("./dataset/chunks");
    if !chunks_dir.exists() {
        fs::create_dir_all(chunks_dir).expect("Failed to create directory");
    }

    let index_file_path = "./dataset/index.txt";
    let pgn_file_path = "./dataset/lichess_db_standard_rated_2016-05.pgn";
    let chunk_size = 10_000; // Define your chunk size
    let total_games = 6_225_957; // Total number of games to process
                                 // let total_games = 10_000; // Total number of games to process

    let num_chunks = (total_games as f64 / chunk_size as f64).ceil() as usize;
    let last_chunk_id = num_chunks - 1;

    let progress_bar = ProgressBar::new(num_chunks as u64);
    progress_bar.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
    )?);

    let i = Arc::new(RwLock::new(1));

    // Process each chunk in parallel
    (0..num_chunks).into_par_iter().for_each(|chunk_id| {
        let actual_chunk_size = if chunk_id == last_chunk_id {
            total_games - (last_chunk_id * chunk_size)
        } else {
            chunk_size
        };

        let start_index = chunk_id * actual_chunk_size;
        process_and_write_chunk(
            index_file_path,
            pgn_file_path,
            chunk_id,
            start_index,
            actual_chunk_size,
        );

        let i_value = *i.read().unwrap();
        progress_bar.set_position(i_value);

        let mut i = i.write().unwrap();
        *i += 1;
    });

    progress_bar.finish_with_message("Preprocessing complete");

    Ok(())
}

fn process_and_write_chunk(
    index_file_path: &str,
    pgn_file_path: &str,
    chunk_id: usize,
    start_index: usize,
    chunk_size: usize,
) {
    let dataset = process_chunk(index_file_path, pgn_file_path, start_index, chunk_size);

    // Serialize to protobuf
    let protobuf_data = serialize_to_protobuf(&dataset).expect("Serialization failed");

    // Write to a temporary file
    let mut temp_file =
        File::create(format!("./dataset/chunks/{}.pb", chunk_id)).expect("File creation failed");
    temp_file.write_all(&protobuf_data).expect("Write failed");
}

// Function to process a chunk of the PGN file
fn process_chunk(
    index_file_path: &str,
    pgn_file_path: &str,
    start_index: usize,
    chunk_size: usize,
) -> ChessDataSet {
    // Use your existing function to read a chunk of games
    let games_san = read_games_in_chunk(index_file_path, pgn_file_path, start_index, chunk_size)
        .expect("Failed to read games");

    // Convert each game's moves into ChessGame messages
    let games: Vec<ChessGame> = games_san
        .into_iter()
        .map(|game_moves| ChessGame { moves: game_moves })
        .collect();

    // Create and return a ChessDataSet containing these games
    ChessDataSet { games }
}

fn serialize_to_protobuf(dataset: &ChessDataSet) -> Result<Vec<u8>, prost::EncodeError> {
    let mut buf = Vec::new();
    dataset.encode(&mut buf)?;
    Ok(buf)
}
