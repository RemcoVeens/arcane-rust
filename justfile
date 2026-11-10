# 🧙 Arcane Rust — run `just` to see all spells

default:
    @just --list

# Check your current quest; open the next one when it passes
quest:
    wr

# Re-run `wr` on every file save
watch:
    cargo watch -q -c -- wr

# Open the book in your browser
book:
    mdbook serve book --open

# Show your XP and mage rank
rank:
    @scripts/rank.sh

# Jump to a specific quest, e.g. `just jump 2 3`
jump chapter exercise:
    wr open --chapter {{chapter}} --exercise {{exercise}}

# Re-check every quest you already solved
recheck:
    wr --recheck

# Play a chapter project, e.g. `just play 1`
play chapter:
    #!/usr/bin/env bash
    set -euo pipefail
    dir=$(ls -d exercises/0{{chapter}}_*/*_project_* | head -1)
    cargo run -q -p "ch0{{chapter}}_$(basename "$dir")"
