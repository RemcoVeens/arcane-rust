# ⚔️ 6.4 — Integration & Doc Tests: Trials from the Outside

> *"Unit tests check each rune in isolation. But a spell is judged in the arena — by strangers who know only what is written on the scroll. Integration tests are those strangers."*

## ⚡ The Concept

### Integration tests: the `tests/` folder

Every `.rs` file in `tests/` (next to `src/`) is compiled as its **own separate crate** that depends on your library. It sees exactly what any user of your crate sees: the public API, nothing more.

```text
04_integration_doc_tests/
├── src/lib.rs
└── tests/
    └── arena.rs      ← a separate crate: `cargo test --test arena`
```

```rust,ignore
// tests/arena.rs
use ch06_04_integration_doc_tests::{Duelist, duel};   // the crate name, like an outsider

#[test]
fn the_first_striker_can_win() {
    let mut tiberius = Duelist::new("Tiberius", 100);
    let mut goblin = Duelist::new("Goblin", 30);
    assert_eq!(duel(&mut tiberius, &mut goblin, 45, 10), "Tiberius");
}
```

No `#[cfg(test)]` and no `mod tests` needed: the whole file only exists for testing. Need shared helpers for several test files? Put them in `tests/common/mod.rs` and `mod common;` from each test file (a plain `tests/common.rs` would be treated as a test crate itself).

<div class="compare">

🐍 Python: a `tests/` folder that imports your package as an installed user would. · 🐹 Go: an external test package `package arena_test` that can only use exported names. · 🦀 Rust: `tests/*.rs`, one crate per file, using your crate by name.

</div>

| | Unit tests | Integration tests |
|---|---|---|
| Lives in | `src/…` in `#[cfg(test)] mod tests` | `tests/*.rs` |
| Sees | everything (private too) | public API only |
| Tests | one function's logic | how the pieces work together |
| Only for | lib and bin crates | **library** crates (binaries can't be imported) |

### Doc tests: examples that can't rot

Code blocks in `///` comments are compiled and run by `cargo test`. Your examples are tests — they can never silently go out of date:

````rust,ignore
/// A new duelist with `hp` hit points.
///
/// ```
/// use ch06_04_integration_doc_tests::Duelist;
///
/// let tiberius = Duelist::new("Tiberius", 100);
/// assert_eq!(tiberius.hp, 100);
/// ```
pub fn new(name: &str, hp: u32) -> Duelist { … }
````

Each block is wrapped in its own `fn main` and compiled as **yet another external crate**, so it needs the `use your_crate::…` line too. Code block flavours:

| Fence | Meaning |
|---|---|
| ` ``` ` | compile and run (default: Rust) |
| ` ```should_panic ` | must panic |
| ` ```no_run ` | compile, don't run (e.g. touches the network) |
| ` ```compile_fail ` | must *fail* to compile — great for showing what's forbidden |
| ` ```ignore ` | skip entirely |
| ` ```text ` | not code at all |

Lines starting with `# ` are compiled but hidden from the rendered docs — handy for boilerplate.

### Running one kind at a time

```bash
cargo test -p ch06_04_integration_doc_tests --lib          # unit tests only
cargo test -p ch06_04_integration_doc_tests --test arena   # one integration file
cargo test -p ch06_04_integration_doc_tests --doc          # doc tests only
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`crate::` in `tests/arena.rs` means the test crate itself**, not your library. Use the library's name.
- **Private helpers are invisible** from `tests/`. Don't make them `pub` just to test them — test them through the public behaviour (or with a unit test).
- **Doc tests forget `use`.** Each example is an outsider; it must import what it uses.
- **Doc tests only run for library crates.** Examples in `main.rs` docs are not tested.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/06_grand_library/04_integration_doc_tests`

1. `tests/arena.rs` **does not compile** — fix its `use` line, and rewrite the armor test so it only uses the public API.
2. Grow `tests/arena.rs` to **at least 5** integration tests.
3. Add **at least 2** doc tests to `src/lib.rs` (one can be a `should_panic` example for `Duelist::new`).

</div>

`wr` counts both kinds, so it really wants them.

## 📜 The Grimoire

<div class="grimoire">

- `tests/name.rs` = separate crate, public API only, `use my_crate::…`
- Shared test helpers: `tests/common/mod.rs` + `mod common;`
- `/// ``` … ``` ` = doc test; flavours `should_panic`, `no_run`, `compile_fail`, `ignore`
- Hide setup lines in docs with `# `
- `cargo test --lib` · `--test <name>` · `--doc`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — the compile error</summary>

Replace `crate::` with the library's crate name: the package name from `Cargo.toml`, `ch06_04_integration_doc_tests`.

</details>
<details class="hint"><summary>Hint 2 — the armor test</summary>

`damage_after_armor` is private, but its effect is visible: build a duelist with `Duelist::new("Knight", 100).with_armor(10)`, call `take_hit(30)` and check what it returns and what's left of `hp`.

</details>
<details class="hint"><summary>Hint 3 — doc tests</summary>

Put this right under the first paragraph of `Duelist::new`'s doc comment (and a similar one, with `duel`, on `duel`):

````rust,ignore
/// ```should_panic
/// use ch06_04_integration_doc_tests::Duelist;
///
/// Duelist::new("Ghost", 0);
/// ```
````

</details>
