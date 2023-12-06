#!/bin/bash

# Directory where the file will be saved
DIR="dataset"

# URL of the file to download
URL="https://database.lichess.org/standard/lichess_db_standard_rated_2016-05.pgn.zst"

# Create the directory if it does not exist
if [ ! -d "$DIR" ]; then
    mkdir -p "$DIR"
fi

# Download the file into the dataset directory
wget -O "$DIR/lichess_db_standard_rated_2016-05.pgn.zst" "$URL"
