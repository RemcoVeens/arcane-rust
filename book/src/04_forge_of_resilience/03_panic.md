# ⚔️ 4.3 — Panic!: When Magic Goes Critical

> *"A spell backfires. A crystal cracks. A summoning goes wrong in a way no apprentice was ever supposed to see. Some failures are not outcomes — they are proof that the world is broken. For those, Rust lets you detonate."*

## ⚡ The Concept

A **panic** stops the current thread: Rust prints a message, unwinds the stack (dropping everything properly), and — in `main` — exits the program with an error.

```rust,should_panic
fn main() {
    panic!("the crystal cracked at charge {}", 140);
}
// thread 'main' panicked at src/main.rs:2:5:
// the crystal cracked at charge 140
```

### Panics you already met

```rust,ignore
let v = vec![1, 2, 3];
v[99];                 // index out of bounds: the len is 3 but the index is 99
let lvl: u8 = level + 250; // level = 10 → (debug builds) attempt to add with overflow
10 / 0;                // attempt to divide by zero
None::<u32>.unwrap();  // called `Option::unwrap()` on a `None` value
```

### Panicking on purpose

```rust,ignore
panic!("message {with} {formatting}");
assert!(charge <= 100, "crystal overcharged: {charge}");   // panic if false
assert_eq!(a, b);                                           // panic if a != b
unreachable!("every element was handled above");
todo!();                                                    // you know this one!
let mana = mana.checked_sub(cost).expect("the spell backfires");
```

`expect("msg")` is `unwrap()` with a message that explains *why it should never happen*. Prefer it over bare `unwrap()`.

### When to panic — and when not

Panic when continuing would be **wrong**, because a promise in your code was broken:

- an **invariant** is violated (a crystal can never exceed 100 charge — if it does, there is a bug somewhere),
- the **caller misused** your function (splitting loot over a party of 0),
- in **tests, prototypes and examples**, where `unwrap` is fine.

Do **not** panic for things that can simply happen: bad user input, a missing file, an unknown spell name. Those are *outcomes*, and the caller deserves to handle them — that's `Result`, next quest.

<div class="compare">

🐍 Python uses exceptions for everything. · 🐹 Go splits it like Rust: `error` values for expected failures, `panic` for programmer bugs. · 🦀 Rust: `Result` for expected failures, `panic!` for bugs. There is no `try/except` for panics in everyday code.

</div>

### Testing that something panics

```rust
pub fn split_loot(gold: u32, party: u32) -> u32 {
    if party == 0 {
        panic!("cannot split loot over an empty party");
    }
    gold / party
}

#[test]
#[should_panic(expected = "empty party")]
fn nobody_to_share_with() {
    split_loot(100, 0);
}
```

`#[should_panic]` makes the test pass only if it panics. Always add `expected = "..."` (a substring of the message): otherwise *any* panic — including a `todo!()` — would make the test pass.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Libraries shouldn't panic on bad input.** Let the caller decide; return a `Result`.
- **`unwrap()` on user input** is a crash waiting for a typo.
- **Overflow panics only in debug builds.** In `--release`, `255u8 + 1` silently wraps to 0. Use `checked_*`, `saturating_*` or `wrapping_*` to say what you mean.
- **`should_panic` without `expected`** passes for the wrong reasons.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/04_forge_of_resilience/03_panic`

Implement four functions that panic exactly when their contract is broken (`panic!`, indexing, `expect`, `assert!`). Then fix the last test: it is *supposed* to panic, so tell the test harness.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `panic!("fmt {}", x)` · `assert!(cond, "msg")` · `assert_eq!` · `unreachable!` · `todo!`
- `opt.expect("why this can't fail")` · `opt.unwrap()`
- `a.checked_sub(b)` → `Option` · `saturating_sub` → clamps at 0
- `#[should_panic(expected = "substring")]` on a `#[test]`
- Panic = bug. `Result` = expected failure.

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — split_loot & read_page</summary>

`split_loot`: an `if party_size == 0 { panic!(...) }` guard, with a message that contains "empty party", then integer division. `read_page`: just `&book[page]` — the built-in index panic message already contains "index out of bounds".

</details>
<details class="hint"><summary>Hint 2 — channel & charge_crystal</summary>

`mana.checked_sub(cost)` gives `None` when it would underflow; `.expect("... backfire ...")` turns that `None` into your panic. For the crystal: compute `charge + extra`, then `assert!(new_charge <= 100, "crystal overcharged: {new_charge}")`.

</details>
<details class="hint"><summary>Hint 3 — the last test</summary>

Add `#[should_panic(expected = "overcharged")]` between `#[test]` and `fn overcharged_crystal_explodes()`.

</details>
