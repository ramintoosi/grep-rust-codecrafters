#!/bin/bash
# Test script to simulate Stage #YX6 (File Search - Recursive search)

set -e

# Clean up any existing test directory
rm -rf dir

# Create directory structure
mkdir -p dir/subdir

# Create test files with content
echo "raspberry" > "dir/fruits-2092.txt"
echo "pear" >> "dir/fruits-2092.txt"
echo "spinach" > "dir/subdir/vegetables-7995.txt"
echo "celery" >> "dir/subdir/vegetables-7995.txt"
echo "cauliflower" >> "dir/subdir/vegetables-7995.txt"
echo "pumpkin" > "dir/vegetables-9474.txt"
echo "tomato" >> "dir/vegetables-9474.txt"
echo "cucumber" >> "dir/vegetables-9474.txt"

echo "Running: ./your_program.sh -r -E .+er dir/"
echo "---"
./your_program.sh -r -E .+er dir/
echo "---"
echo "Exit code: $?"

# Optional: Clean up
# Uncomment the line below to remove test directory after running
# rm -rf dir
