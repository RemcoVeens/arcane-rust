# 🧙 Arcane Rust — The Terminal Grimoire

Learn Rust by building a terminal RPG, one tested quest at a time.
8 chapters · ~70 quests · every quest checked by [`wr`](https://github.com/mainmatter/rust-workshop-runner).

## Start

```bash
cargo install --locked workshop-runner mdbook cargo-watch   # once
just book      # read the quest in your browser
just quest     # check your work (= wr), unlock the next quest
just edit      # open the current quest in Zed
just watch     # re-check on every save
just rank      # XP & mage rank
```

No `just`? Use `wr`, `mdbook serve book --open`, `cargo watch -- wr` directly.

## How it works

- `book/` — the story and the concepts (mdBook). Each page ends with a ⚔️ **Quest**.
- `exercises/NN_chapter/NN_quest/` — a small crate with `todo!()`s and tests. Make the tests pass.
- `wr` builds + tests your current quest, and only then opens the next one. Progress: `exercises/progress.db`.
- Stuck? Each book page has progressive 🔮 hints. Don't edit the tests.
- Chapter projects are playable: `just play 1`.

## Legacy

Your original hand-written Chapter 1 spellbook lives in `legacy/spellbook/`
(`cargo run --manifest-path legacy/spellbook/Cargo.toml`), and at git tag `pre-workshop`.
