#!/bin/bash

if [ -z "$1" ]; then
    echo "Usage: $0 <day_name>"
    echo "Example: $0 day01"
    exit 1
fi

NEW_DAY=$1

if [ -d "$NEW_DAY" ]; then
    echo "Error: Directory '$NEW_DAY' already exists."
    exit 1
fi

# Just simply copy the template day00 directory
cp -r day00 "$NEW_DAY"

# Update the package name in Cargo.toml
sed "s/name = \"day00\"/name = \"$NEW_DAY\"/" "$NEW_DAY/Cargo.toml" > "$NEW_DAY/Cargo.toml.tmp" && mv "$NEW_DAY/Cargo.toml.tmp" "$NEW_DAY/Cargo.toml"

# Makes no sense to keep the 'target' directory...
rm -rf "$NEW_DAY/target"

echo "Successfully created '$NEW_DAY' based on 'day00'."
