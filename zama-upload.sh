#!/bin/bash

if [ -z "$1" ]; then
  echo "Error: Please provide a file_name"
  exit 1
fi

FOLDER_PATH="$1"

docker-compose run --rm -v "$FOLDER_PATH":/app/data/uploads zama-client upload --folder_path /app/data/uploads
