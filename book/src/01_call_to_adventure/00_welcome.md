# ⚔️ 1.0 — The Call

> *"The Academy gates are shut. A stone Gatekeeper stares down at you. 'Speak the Oath,' it rumbles, 'exactly as it is written — or go home.'"*

## ⚡ The Concept

### Cargo, crates and tests

Every quest is a **crate** — Rust's unit of compilation — managed by **Cargo**, Rust's build tool and package manager:

| | 🐍 Python | 🐹 Go | 🦀 Rust |
|---|---|---|---|
| Package manager | `pip` | `go mod` | `cargo` |
| Run tests | `pytest` | `go test` | `cargo test` |
| Manifest | `pyproject.toml` | `go.mod` | `Cargo.toml` |

A quest crate looks like this:

```text
00_welcome/
├── Cargo.toml     ← name, edition, dependencies
└── src/
    └── lib.rs     ← your code + the tests
```

### Reading a test

At the bottom of `lib.rs` lives a test module:

```rust,ignore
#[cfg(test)]          // only compiled when running `cargo test`
mod tests {
    use super::*;     // bring the code above into scope

    #[test]           // this function is a test
    fn the_oath_is_spoken() {
        assert_eq!(oath(), "I seek the Arcane");
    }
}
```

`assert_eq!(left, right)` panics — fails the test — when the two values differ. The `!` means it's a **macro**, not a function (more on that in Chapter 8). `println!` is a macro too.

### Returning a value

```rust
fn oath() -> &'static str {
    "I seek the Arcane"
}
```

`-> &'static str` says "returns a piece of text that lives forever" (a string literal baked into the binary). The last expression in a function — **without a semicolon** — is its return value. No `return` needed.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Exact means exact.** `"I seek the arcane"` (lowercase a) is a different string. The Gatekeeper is pedantic, and so is `assert_eq!`.
- **Run `wr` from the repository root**, not from inside the exercise folder.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/01_call_to_adventure/00_welcome`

Make `oath()` return the correct oath, then run `wr`.

</div>

First, see it fail:

```bash
wr
```

Read the red text. Find the line that says `🚪 The Gatekeeper frowns`. Now fix `oath()` and run `wr` again. When it asks to open the next quest — say **y**.

## 📜 The Grimoire

<div class="grimoire">

- A crate = `Cargo.toml` + `src/`. `cargo test` runs its tests.
- `#[test]` marks a test; `assert_eq!(a, b)` fails it when `a != b`.
- The last expression of a function (no `;`) is its return value.
- `wr` = build + test the current quest, then unlock the next one.

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

Look at the string inside `assert_eq!` in the test. That's the exact text you need.

</details>
<details class="hint"><summary>Hint 2</summary>

Replace the line `"I seek... uh... snacks?"` with `"I seek the Arcane"`.

</details>
