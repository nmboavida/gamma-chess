use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

pub fn main() {
    // if let Err(e) = print_first_1000_lines("./dataset/lichess_db_standard_rated_2016-05.pgn") {
    //     eprintln!("Error occurred: {}", e);
    // }

    create_index(
        "./dataset/lichess_db_standard_rated_2016-05.pgn",
        "./dataset/index_dummy.txt",
    )
    .expect("Index creation failed");
}

fn print_first_1000_lines(file_path: &str) -> Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        println!("{}", line);

        if i >= 999 {
            // Zero-based index, so 999 means 1000 lines
            break;
        }
    }

    Ok(())
}

pub fn create_index(pgn_file_path: &str, index_file_path: &str) -> Result<()> {
    let file = File::open(pgn_file_path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();

    let reader = BufReader::new(file);
    let mut index_file = File::create(index_file_path)?;

    let progress_bar = ProgressBar::new(file_size);
    progress_bar.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
    )?);

    let mut offset: u64 = 0;
    let mut in_game = false;

    let mut i = 0;

    for line in reader.lines() {
        i += 1;

        if i >= 1_000 {
            break;
        }
        let line = line?;
        println!("{}", line);
        if line.starts_with("[Event ") && !in_game {
            println!("WOHO!! Index: {}", i);
            // Start of a new game
            writeln!(index_file, "{}", offset)?;
            in_game = true;
        } else if line.trim().is_empty() {
            in_game = false;
        }
        offset += line.len() as u64 + 1; // +1 for the newline character
        progress_bar.set_position(offset);
    }

    progress_bar.finish_with_message("Indexing complete");

    Ok(())
}
