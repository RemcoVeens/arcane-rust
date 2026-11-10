# ⚔️ 6.3 — Unit Tests: The Testing Chamber

> *"A mage who never tests their spells is a danger to themselves and others. The Inquisitor walks the halls at night, swapping a rune here, a sigil there. Only a mage with good tests notices by morning."*

## ⚡ The Concept

So far the tests were written *for* you. Now you write them. In Rust, **unit tests live next to the code**, in a module compiled only for `cargo test`:

```rust
pub fn spell_damage(power: u32, level: u32) -> u32 {
    power + level * 5
}

#[cfg(test)]            // only compiled by `cargo test`
mod tests {
    use super::*;       // the code above: pub AND private items

    #[test]
    fn damage_grows_with_level() {
        assert_eq!(spell_damage(45, 7), 80);
    }
}
```

Because `tests` is a child module, `use super::*` gives it access to **private** functions too. That is the big difference with integration tests (next quest), which only see the public API.

<div class="compare">

🐍 pytest: `test_*.py` files and `assert x == y`; testing a `_private` function is merely frowned upon. · 🐹 Go: `_test.go` files in the same package, so unexported functions are visible — Rust's `mod tests` is the same idea, in the same file. · 🦀 No test framework to install: `#[test]` and the `assert` macros are built in.

</div>

### The assertion toolkit

```rust,ignore
assert!(can_cast, "a message, with {} formatting", "optional");
assert_eq!(left, right);               // shows both values on failure
assert_ne!(spell_damage(45, 1), 45);
```

### Testing `Option`, `Result` and panics

```rust,ignore
#[test]
fn not_enough_mana() {
    assert_eq!(try_cast(20, 30), None);      // Option and Result implement PartialEq
}

#[test]
#[should_panic(expected = "level-0")]       // passes only if it panics
fn forbidden_summon() {                     // with a message containing "level-0"
    summon(0);
}

#[test]
fn returning_result() -> Result<(), String> {
    let mana = try_cast(100, 30).ok_or("no mana")?; // `?` works in tests that return Result
    assert_eq!(mana, 70);
    Ok(())
}
```

Always add `expected = "…"` to `should_panic`: otherwise a test passes for *any* panic, including an `index out of bounds` you didn't mean.

### Running tests

```bash
cargo test -p ch06_03_unit_tests              # all tests of one package
cargo test -p ch06_03_unit_tests crit         # only tests whose name contains "crit"
cargo test -p ch06_03_unit_tests -- --list    # list the tests without running them
cargo test -p ch06_03_unit_tests -- --nocapture  # show println! output
```

Everything after `--` goes to the test binary, not to Cargo.

### What makes a good test?

A test is only as good as the bugs it catches. Test the **edges** a spec mentions: "exactly enough mana", "18 is already a crit", "ties stay in order", "level 0 panics". Testing only the happy middle (`spell_damage(45, 7)`) lets plenty of mutants through.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Tests run in parallel**, in threads. Don't rely on order or shared mutable state between tests.
- **A test that never asserts never fails.** Calling a function without checking its result proves only that it doesn't panic.
- **`should_panic` without `expected`** can hide the wrong panic.
- **Forgetting `use super::*;`** gives "cannot find function in this scope" — the tests module is a separate module.
- **Floating point:** `assert_eq!(0.1 + 0.2, 0.3)` fails. Compare with a tolerance: `assert!((a - b).abs() < 1e-9)`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/06_grand_library/03_unit_tests`

The code is correct — the tests are missing. Write **at least 5 tests** in `mod tests` that:

- all pass on the real code, and
- catch **every saboteur**: `wr` rebuilds the crate with each `sabotage_*` feature on (see `Cargo.toml`) and expects at least one of your tests to fail each time. One saboteur hides in the **private** `crit_multiplier`.

</div>

Read each doc comment, then ask: *"what is the sneakiest wrong implementation that would still look right?"* — and write the test that kills it. You can run a saboteur yourself:

```bash
cargo test -p ch06_03_unit_tests --features sabotage_crit   # should FAIL
```

## 📜 The Grimoire

<div class="grimoire">

- `#[cfg(test)] mod tests { use super::*; #[test] fn …() { … } }`
- Unit tests can test private items
- `assert!`, `assert_eq!`, `assert_ne!` — all take an optional message
- `#[should_panic(expected = "…")]` · tests may return `Result<(), E>`
- `cargo test <filter>` · `cargo test -- --list` · `-- --nocapture`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — getting started</summary>

Start the module with `use super::*;`, then write one `#[test] fn` per function. Run `wr` often: its messages tell you which saboteur escaped.

</details>
<details class="hint"><summary>Hint 2 — the edges</summary>

`try_cast`: what should happen with 20 mana and a cost of 30? `crit_multiplier`: test the *boundary* roll, not just 20. `rank_by_power`: use at least three spells so a reversed order shows. `summon`: there's an attribute for "this must panic".

</details>
<details class="hint"><summary>Hint 3 — almost there</summary>

```rust,ignore
#[test]
fn crits_start_at_eighteen() {
    assert_eq!(crit_multiplier(17), 1);
    assert_eq!(crit_multiplier(18), 2);
}

#[test]
#[should_panic(expected = "level-0")]
fn level_zero_summons_panic() {
    summon(0);
}
```

Add similar tests for `spell_damage` (try level 0 *and* a higher level), `try_cast` returning `None`, and `rank_by_power` with a strongest-first `vec![…]`.

</details>
