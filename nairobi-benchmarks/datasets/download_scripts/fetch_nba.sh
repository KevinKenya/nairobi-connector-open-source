#!/bin/bash
# nairobi-benchmarks/datasets/download_scripts/fetch_nba.sh
set -e

# Base directory relative to this script
BASE_DIR=$(dirname $(dirname $(dirname $(readlink -f "$0"))))
TARGET_DIR="$BASE_DIR/datasets/nba_real"
mkdir -p "$TARGET_DIR"

echo "Downloading PlayerStatisticsExtended.csv from Kaggle..."
kaggle datasets download -d eoinamoore/historical-nba-data-and-player-box-scores -f PlayerStatisticsExtended.csv -p "$TARGET_DIR"

if [ -f "$TARGET_DIR/PlayerStatisticsExtended.csv.zip" ]; then
    echo "Unzipping dataset..."
    unzip -o "$TARGET_DIR/PlayerStatisticsExtended.csv.zip" -d "$TARGET_DIR"
    echo "Cleaning up zip file..."
    rm "$TARGET_DIR/PlayerStatisticsExtended.csv.zip"
else
    echo "File was downloaded directly or already exists unzipped."
fi

echo "Data acquisition complete. File: $TARGET_DIR/PlayerStatisticsExtended.csv"
ls -lh "$TARGET_DIR/PlayerStatisticsExtended.csv"
