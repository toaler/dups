#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <directory> <svg_name>"
    exit 1
fi

directory="$1"
svg_name="$2"

# Run flamegraph command
flamegraph --skip-after "dups::file_system_traversal::FileSystemTraversal::traverse" -o "$svg_name" -- target/debug/dups "$directory"
