#!/bin/bash
# Copyright 2026 Kevin Chege
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

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
