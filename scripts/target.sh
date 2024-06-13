#!/bin/bash

# Function to find the repository root
find_repo_root() {
    local dir="$PWD"
    while [[ "$dir" != "/" ]]; do
        if [[ -f "$dir/.plzconfig" ]]; then
            echo "$dir"
            return 0
        fi
        dir=$(dirname "$dir")
    done
    return 1
}

# Call the function and print the repository root
REPO_ROOT=$(find_repo_root)
if [[ -n "$REPO_ROOT" ]]; then
    echo "$REPO_ROOT"
else
    echo "Error: Could not find the repository root."
    exit 1
fi
