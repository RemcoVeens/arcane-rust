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
    wr open --chapter {{ chapter }} --exercise {{ exercise }}

# Re-check every quest you already solved
recheck:
    wr --recheck

# Play a chapter project, e.g. `just play 1`
play chapter:
    #!/usr/bin/env bash
    set -euo pipefail
    dir=$(ls -d exercises/0{{ chapter }}_*/*_project_* | head -1)
    cargo run -q -p "ch0{{ chapter }}_$(basename "$dir")"

# Open the current quest in Zed (lib.rs + book page; projects also open their tests)
edit:
    #!/usr/bin/env bash
    set -euo pipefail
    db=exercises/progress.db
    current=""
    [ -f "$db" ] && current=$(sqlite3 "$db" "select chapter || '/' || exercise from open_exercises where solved=0 order by chapter, exercise limit 1;")
    if [ -z "$current" ]; then
        echo "No open quest. Run 'just quest' to unlock the next one."
        exit 1
    fi
    dir="exercises/$current"
    files=("$dir/src/lib.rs")
    [ -d "$dir/tests" ] && files+=("$dir"/tests/*.rs)
    [ -f "book/src/$current.md" ]
    # [ -f "book/src/$current.md" ] && files+=("book/src/$current.md")
    echo "⚔️  Opening $current"
    zed . "${files[@]}"

# Play the project of the chapter you're currently in (based on wr progress)
play-current:
    #!/usr/bin/env bash
    set -euo pipefail
    db=exercises/progress.db
    chapter=""
    [ -f "$db" ] && chapter=$(sqlite3 "$db" "select chapter from open_exercises order by solved, chapter desc, exercise desc limit 1;")
    if [ -z "$chapter" ]; then
        echo "No quest opened yet. Run 'just quest' first."
        exit 1
    fi
    dir=$(ls -d exercises/"$chapter"/*_project_* | head -1)
    pkg="ch${chapter:0:2}_$(basename "$dir")"
    echo "🎮 Playing $pkg"
    cargo run -q -p "$pkg"
