#!/bin/bash

# Directory containing the zst file
DIR="dataset"

# File name of the zst file
FILE="lichess_db_standard_rated_2016-05.pgn.zst"

# Check if zstd is installed
if ! command -v zstd &> /dev/null
then
    echo "zstd could not be found, please install it to continue."
    exit 1
fi

# Check if the file exists
if [ ! -f "$DIR/$FILE" ]; then
    echo "File not found: $DIR/$FILE"
    exit 1
fi

# Decompress the file
zstd -d "$DIR/$FILE" -o "$DIR/${FILE%.zst}"

echo "Decompression complete."
