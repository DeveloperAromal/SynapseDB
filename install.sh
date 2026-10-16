#!/bin/bash


set -e

# Run from the repository root no matter which directory the script is called from.
cd "$(dirname "$0")"

echo "Starting Synapse..."

go run .
