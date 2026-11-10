# ⚔️ 2.3 — Clone & Copy: The Scribe's Duplicates

> *"A whisper can be repeated for free. A library must be copied by hand, page by page, by a scribe who charges by the hour. Rust asks you to know which one you hold."*

## ⚡ The Concept

### `Clone` — the explicit, deep copy

When you really need two independent values, ask for it:

```rust
let original = String::from("Fireball");
let mut copy = original.clone();   // allocates new heap memory, copies the bytes
copy.push_str(" +1");
assert_eq!(original, "Fireball");  // untouched
assert_eq!(copy, "Fireball +1");
```

`.clone()` is always *visible* in the code. That is a feature: you can grep for every place where your program duplicates heap data.

### `Copy` — the implicit, cheap copy

Some types are so small and simple that duplicating them is just copying bits. Those are **`Copy`**: assignment copies instead of moving, and the original stays valid.

```rust
let rune = 'ᚱ';
let pair = (rune, rune);       // char is Copy — no clone needed
let level: u32 = 7;
let other = level;             // still both usable
```

A type is `Copy` only if it owns **no heap memory** (and nothing else that needs cleanup):

| Type | `Copy`? | Why |
|---|---|---|
| `i32`, `u64`, `f64`, `bool`, `char` | ✅ | plain bits |
| `(u8, bool)`, `[u32; 3]` | ✅ | made only of Copy parts |
| `&str`, any `&T` | ✅ | copying a *reference* just copies a pointer |
| `String`, `Vec<T>` | ❌ | own heap memory |
| `(String, u32)` | ❌ | one non-Copy part is enough |

`Copy` types are also `Clone` (`5u32.clone()` works — it's just pointless).

<div class="compare">

🐍 **Python:** `b = a` never copies; `copy.copy` / `copy.deepcopy` when you need it. Same idea as `.clone()`, but nobody makes you think about it.

🐹 **Go:** assignment copies *the value* — which for slices and maps is a header pointing at shared data. Real deep copies are manual (`copy()`, `maps.Clone`).

🦀 **Rust:** small value types are copied implicitly (`Copy`), everything else moves, and deep copies are always an explicit `.clone()`.

</div>

### Your own types

In Chapter 3 you will write your own types (`struct`s). They are **not** `Copy` or `Clone` automatically — you opt in with a *derive* line, which asks the compiler to write the code for you:

```rust,ignore
#[derive(Clone, Copy)]     // only allowed if every field is Copy
struct Rune(char);
```

For now, tuples and `String` are all we need.

### Clones in loops

Remember the move-in-a-loop problem? Cloning is the honest fix when each iteration truly needs its own copy:

```rust
let spell = String::from("Blink");
let mut images = Vec::new();       // a growable list; owns its elements
for _ in 0..3 {
    images.push(spell.clone());    // each image owns its own String
}
```

`Vec::new()` creates an empty vector, `.push(x)` moves `x` into it, `.len()` counts the elements. Vectors get a full quest in Chapter 4 — this is all you need now.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Clone-to-shut-up-the-compiler.** It works, it hides design problems, and in a hot loop it allocates like crazy. If you only need to *read*, borrow (next quest).
- **`Copy` is all or nothing.** One `String` inside a tuple makes the whole tuple move-only.
- **Cloning a reference.** `(&s).clone()` clones the `String` behind it — usually what you want. But in generic code it's easy to end up cloning just the `&` (Chapter 4). Read the type!

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/02_ownership_chronicles/03_clone_copy`

Duplicate scrolls, enchant a copy without touching the original, conjure mirror images in a loop, pair up a rune — and pass the Oracle's `Copy` quiz.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `x.clone()` — explicit deep copy; may allocate
- `Copy` — implicit bitwise copy on assignment/passing; original stays valid
- Copy: integers, floats, `bool`, `char`, `&T`, tuples/arrays of Copy types
- Not Copy: `String`, `Vec<T>`, anything containing them
- `let mut v = Vec::new(); v.push(x); v.len()`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — duplicate & enchant_copy</summary>

Clone *first*, while you still own the original: `let copy = scroll.clone();` then return `(scroll, copy)`. For `enchant_copy`, make the clone `mut` and `push_str` onto it.

</details>
<details class="hint"><summary>Hint 2 — mirror_images</summary>

Push `spell.clone()` inside the `for _ in 0..n` loop. Pushing `spell` itself would move it on the first iteration. (For extra elegance: clone `n - 1` times and move the original in last — but mind `n == 0`.)

</details>
<details class="hint"><summary>Hint 3 — the quiz</summary>

Apply the rule "does it own heap memory, directly or through a part?" to each line: `i32` no, `String` yes, `(u8, bool)` no, `[u32; 3]` no, `(String, u32)` yes … finish the last two yourself.

</details>
