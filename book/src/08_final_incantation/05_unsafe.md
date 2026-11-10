# ⚔️ 8.5 — Unsafe & Publishing: The Forbidden Vault

> *"Beneath the Academy lies a vault with a single rule carved over the door: 'Here, the Wards do not protect you. Here, YOU are the Ward.' Few enter. Those who do write down every promise they make — and seal the vault behind a door that anyone may safely open."*

## ⚡ The Concept

### What `unsafe` does — and doesn't — do

`unsafe` doesn't switch off the borrow checker or type system. It unlocks exactly **five extra abilities**:

1. Dereference a raw pointer (`*const T`, `*mut T`)
2. Call an `unsafe fn` (including C functions via FFI)
3. Access or modify a mutable `static`
4. Implement an `unsafe trait` (like `Send`/`Sync`)
5. Access fields of a `union`

In exchange, **you** promise to uphold the rules the compiler can no longer check: no dangling pointers, no data races, no aliasing `&mut`, no out-of-bounds reads. Break the promise and you get *undefined behavior* — anything can happen, including "works on my machine".

<div class="compare">

🐹 Go's `unsafe.Pointer` and 🐍 Python's `ctypes` are the same idea: an escape hatch from the safe language. Rust's difference: `unsafe` blocks are small, greppable, and everything around them stays checked.

</div>

### Raw pointers

Creating one is safe; *dereferencing* one is not:

```rust
let mana: u32 = 100;
let p = &mana as *const u32;           // safe: just an address
// SAFETY: `p` comes from a live reference to `mana`.
let value = unsafe { *p };             // unsafe: compiler can't prove `p` is valid
assert_eq!(value, 100);
```

Raw pointers may be null, dangling, unaligned or aliased — that's why reading through them needs your promise.

### `unsafe fn` and the `# Safety` contract

An `unsafe fn` is a function whose caller must uphold conditions. Document them in a `# Safety` section; callers acknowledge them with an `unsafe { }` block and a `// SAFETY:` comment:

```rust
/// # Safety
/// `p` must be non-null, aligned and point to an initialised `u32`.
pub unsafe fn read(p: *const u32) -> u32 {
    // SAFETY: guaranteed by this function's contract.
    unsafe { *p }
}
```

**Edition 2024 note:** the body of an `unsafe fn` is no longer one big unsafe block — each unsafe operation inside still needs its own `unsafe { }` (the `unsafe_op_in_unsafe_fn` lint). That keeps the dangerous lines visible.

### The safe wrapper pattern

The real skill: build a **safe** function whose inside uses `unsafe`, after checking everything needed for the contract. The classic example is `split_at_mut` — two `&mut` halves of one slice. The borrow checker can't see that the halves don't overlap:

```rust,compile_fail
fn split(v: &mut [u32], mid: usize) -> (&mut [u32], &mut [u32]) {
    (&mut v[..mid], &mut v[mid..]) // ❌ cannot borrow `*v` as mutable more than once
}
```

But *we* can prove it: check `mid <= len`, then build both halves from raw parts with `v.as_mut_ptr()`, `ptr.add(mid)` and `std::slice::from_raw_parts_mut(ptr, len)`. The ranges `[0, mid)` and `[mid, len)` lie inside the slice and are disjoint — write exactly that in your `// SAFETY:` comment. Callers get a perfectly safe API.

"Check, then trust" works for indexing too: after `if !runes.is_empty()`, `runes.get_unchecked(0)` is sound.

### Things to avoid

- `static mut`: in edition 2024, taking a reference to one is an error. Use an atomic or a `Mutex` in a normal `static`.
- `std::mem::transmute`: reinterprets bits as another type. There's almost always a safe alternative.

## 📦 Publishing to crates.io — a checklist

You won't publish workshop exercises (`publish = false` in the workspace), but when you publish your own crate:

1. **Metadata** in `Cargo.toml`: `name` (unique on crates.io), `version`, `edition`, `description`, `license` (e.g. `"MIT OR Apache-2.0"`), `repository`, `readme`, `keywords` (≤ 5), `categories`.
2. **Docs**: crate-level `//!` docs in `lib.rs`, `///` on every public item, `# Examples`, `# Errors`, `# Panics` and `# Safety` sections. Add `#![warn(missing_docs)]`. Preview with `cargo doc --open`.
3. **Quality gates**: `cargo fmt --check`, `cargo clippy -- -D warnings`, `cargo test` (doctests included).
4. **Semver**: `0.x.y` while experimenting; any breaking public API change bumps MAJOR (or MINOR while < 1.0). Features must be **additive**.
5. **Dry run**: `cargo package --list` (what gets uploaded?) and `cargo publish --dry-run`.
6. **Publish**: `cargo login` once with an API token, then `cargo publish`. Published versions are permanent; a broken one can only be **yanked** (`cargo yank --version 1.0.1`), not deleted.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **"It passed the tests" proves nothing about UB.** Run unsafe code under Miri (`cargo +nightly miri test`) when you write real unsafe code.
- **`unsafe` is contagious only if you let it be.** Never expose a safe function that callers can make misbehave — if they can, the function must be `unsafe fn`.
- **Two `&mut` to the same data is UB** even if you never use them "at the same time".

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/08_final_incantation/05_unsafe`

Read and write runes through raw pointers in `unsafe fn`s with documented contracts, then build three **safe** wrappers: `swap_mana` (via `ptr::swap`), `first_rune` (check, then `get_unchecked`) and `split_ranks` — your own `split_at_mut`. Every `unsafe` block gets a `// SAFETY:` comment.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `&x as *const T` / `&mut x as *mut T` — safe to create
- `unsafe { *p }` — deref needs `unsafe`
- `/// # Safety` on `unsafe fn` · `// SAFETY:` on every `unsafe { }`
- `slice.as_mut_ptr()` · `ptr.add(n)` · `std::slice::from_raw_parts_mut(ptr, len)`
- `std::ptr::swap(a, b)` · `slice.get_unchecked(i)`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — read_rune & engrave_rune</summary>

One line each, inside an `unsafe { }` block: `*rune` and `*slot = value`. The contract is the caller's job; your `// SAFETY:` comment can say so.

</details>
<details class="hint"><summary>Hint 2 — swap_mana & first_rune</summary>

`&mut u32` coerces to `*mut u32` automatically, so `unsafe { std::ptr::swap(a, b) }` compiles. For `first_rune`: return `None` early when empty, then `Some(unsafe { *runes.get_unchecked(0) })`.

</details>
<details class="hint"><summary>Hint 3 — split_ranks</summary>

`let len = ranks.len(); assert!(mid <= len, "mid out of bounds: …");` then `let ptr = ranks.as_mut_ptr();` and, in one `unsafe` block, return the tuple `(from_raw_parts_mut(ptr, mid), from_raw_parts_mut(ptr.add(mid), len - mid))`.

</details>
