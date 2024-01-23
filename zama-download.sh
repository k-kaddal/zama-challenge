#!/bin/bash

if [ -z "$1" ] || [ -z "$2" ]; then
  echo "Error: Please provide both folder_path and file_name."
  exit 1
fi

FOLDER_PATH="$1"
FILE_NAME="$2"

docker-compose run --rm -v "$FOLDER_PATH":/app/data/uploads zama-client download --file_name "$FILE_NAME"
