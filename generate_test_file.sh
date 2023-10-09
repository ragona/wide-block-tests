#!/bin/bash

# Check for the correct number of arguments
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <FILE_PATH> <FILE_SIZE>"
    exit 1
fi

# Get the file path and size from the arguments
FILE_PATH="$1"
FILE_SIZE="$2"  # in megabytes

# Create a file of the specified size filled with random data
dd if=/dev/urandom of="$FILE_PATH" bs=1M count="$FILE_SIZE"

# Print a message when done
echo "$FILE_SIZE MB file $FILE_PATH generated successfully."
