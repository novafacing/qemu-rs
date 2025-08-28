#!/bin/bash

# Check arguments
if [ $# -ne 1 ]; then
    echo "Usage: $0 <path-to-qemu-repository>"
    exit 1
fi

REPO_PATH="$1"
FILE_PATH="include/qemu/qemu-plugin.h"
PATTERN="#define QEMU_PLUGIN_VERSION"

# Check if directory exists and is a git repository
if [ ! -d "$REPO_PATH" ]; then
    echo "Error: Directory $REPO_PATH does not exist"
    exit 1
fi

cd "$REPO_PATH" || exit 1

if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "Error: $REPO_PATH is not a git repository"
    exit 1
fi

# Get all tags sorted by version
TAGS=$(git tag --sort=version:refname 2>/dev/null)

if [ -z "$TAGS" ]; then
    echo "No git tags found in this repository"
    exit 0
fi

# Print header
printf "%-20s %s\n" "TAG" "VERSION"
printf "%-20s %s\n" "---" "-------"

# Process each tag
while IFS= read -r tag; do
    # Get the file content at this tag using git show
    FILE_CONTENT=$(git show "$tag:$FILE_PATH" 2>/dev/null)
    
    if [ $? -ne 0 ] || [ -z "$FILE_CONTENT" ]; then
        printf "%-30s %s\n" "$tag" "file not found"
        continue
    fi
    
    # Search for the pattern and extract version
    VERSION=$(echo "$FILE_CONTENT" | grep "$PATTERN" | head -1 | sed -n "s/.*${PATTERN}[[:space:]]\+\([0-9]\+\).*/\1/p")
    
    if [ -n "$VERSION" ]; then
        printf "%-30s %s\n" "$tag" "$VERSION"
    else
        printf "%-30s %s\n" "$tag" "pattern not found"
    fi
    
done <<< "$TAGS"