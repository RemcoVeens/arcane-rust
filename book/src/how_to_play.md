# ⚗️ How to Play

## 1. The tools

```bash
# Rust itself (skip if `cargo --version` works)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# The workshop runner, this book, and a file watcher
cargo install --locked workshop-runner mdbook cargo-watch
```

Use an editor with **rust-analyzer** — it shows errors as you type.

## 2. The loop

From the root of the repository:

```bash
just book     # open this book in your browser (mdbook serve --open)
just quest    # = wr : check your current quest, open the next one when it passes
just watch    # re-run `wr` every time you save a file
just rank     # see your XP and mage rank
```

`wr` works through `exercises/` in order. For every quest it:

1. **builds** the crate (`cargo build`) — a compile error means the quest is not solved yet;
2. **tests** it (`cargo test`) — every Gatekeeper must pass;
3. when everything is green, asks whether to **open the next quest**.

Your progress lives in `exercises/progress.db`. Run `wr --recheck` to re-check everything you already solved.

## 3. A quest, step by step

1. Read the page in this book.
2. Open the exercise folder named in the ⚔️ **Quest** box.
3. Read `src/lib.rs` from top to bottom — the comments tell you what to do, the tests at the bottom are the exact spec.
4. Replace the `todo!()`s / fix the errors.
5. `just quest`. Red? Read the message — **errors are your teachers**. Green? Onward!

Stuck? Every page ends with progressive 🔮 **Hints**. Open them one at a time.

<div class="pitfall">

⚠️ **Don't edit the tests.** They are the Gatekeepers. Bribing them only cheats yourself.

</div>

## 4. Projects

The last quest of every chapter is a **Project**: a playable terminal game.

```bash
cargo run -p ch01_08_project_spellbook
```

Each project comes with a scaffold that already contains the previous chapter's features, so a stumble in one chapter never blocks the next.

## 5. Ranks

| XP | Rank |
|---|---|
| 0 | 🕯️ Apprentice |
| 150 | 📘 Initiate |
| 300 | 🔮 Adept |
| 500 | ⚡ Conjurer |
| 750 | 🔥 Sorcerer |
| 990 | 🌌 Archmage — every quest complete |

A quest is worth **10 XP**, a project **50 XP**.
