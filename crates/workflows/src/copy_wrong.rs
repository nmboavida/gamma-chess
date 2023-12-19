use anyhow::anyhow;
use anyhow::Result;
use consts::{DATASET_FOLDER, INDEX_FILE_PATH, PGN_FILE_PATH};
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};

pub mod consts;

fn main() -> Result<()> {
    let start_index = 2_782_163;
    let end_index = 2_782_164;

    let output_path = format!("{DATASET_FOLDER}/extracted_lines.txt"); // The path where the extracted lines will be saved

    let games = read_games(PGN_FILE_PATH, INDEX_FILE_PATH, start_index, end_index)?;

    // Open a file in write mode
    let mut output_file = File::create(output_path)?;

    // Write the extracted games to the file
    write!(output_file, "{}", games)?;

    Ok(())
}

fn read_games(
    pgn_file_path: &str,
    index_file_path: &str,
    start_index: usize,
    end_index: usize,
) -> Result<String> {
    let index_file = File::open(index_file_path)?;
    let reader = BufReader::new(index_file);
    let lines: Vec<_> = reader.lines().collect();

    let start_offset_str = lines
        .get(start_index)
        .ok_or_else(|| anyhow!("Start index out of bounds"))?
        .as_ref()
        .unwrap();

    let start_offset = start_offset_str
        .parse::<u64>()
        .expect("Failed to parse start offset");

    let end_offset_str = lines
        .get(end_index)
        .ok_or_else(|| anyhow!("End index out of bounds"))?
        .as_ref()
        .unwrap();

    let end_offset = end_offset_str
        .parse::<u64>()
        .expect("Failed to parse end offset");

    let mut pgn_file = File::open(pgn_file_path)?;
    pgn_file.seek(SeekFrom::Start(start_offset))?;

    let mut content = String::new();
    let mut buffer = Vec::new();
    while pgn_file.stream_position()? < end_offset {
        let size = (end_offset - pgn_file.stream_position()?).min(1024);
        buffer.resize(size as usize, 0);
        pgn_file.read_exact(&mut buffer)?;
        content.push_str(std::str::from_utf8(&buffer)?);
    }

    Ok(content)
}
