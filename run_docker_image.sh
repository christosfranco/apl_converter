#!/bin/bash

# Check if the number of arguments is correct
if [ $# -ne 1 ]; then
    echo "Usage: $0 <file_path>"
    exit 1
fi

# Get the file path from the command-line argument
file_path="$1"

# Check if the file exists
if [ ! -f "$file_path" ]; then
    echo "File not found: $file_path"
    exit 1
fi

# Get the file name from the file path
file_name=$(basename "$file_path")

# Run the Docker container and capture the container ID
container_id=$(sudo docker run -d -p 8080:8080 -v "$(dirname "$file_path")":/input apl_converter ./target/release/apl_converter -f "/input/$(basename "$file_path")")

# Check if the container ID is not empty
if [ -n "$container_id" ]; then
    # If the container ID is not empty, use it to get the logs
    sudo docker logs "$container_id"
else
    echo "Failed to start the container."
fi
