# ⚔️ 2.0 — Stack & Heap: The Two Realms of Memory

> *"On the material plane, objects sit neatly on shelves, each in its place. In the ethereal plane, things float freely, tied to the material world by invisible threads. Your program lives in both."*

## ⚡ The Concept

Ownership only makes sense once you know *where* data lives. Every Rust program uses two memory regions.

### The stack — the material plane

A stack of plates: push on top, pop from the top. Every function call pushes a **frame** with its local variables; returning pops it.

- Every value must have a **size known at compile time**.
- Allocation is basically free (move a pointer).
- Cleanup is automatic: when the function returns, the frame is gone.

```rust
fn duel() {
    let mana: u32 = 100;          // 4 bytes, on the stack
    let ready = true;             // 1 byte
    let circle = [30u32; 5];      // 20 bytes — arrays are fixed-size, so stack
}   // frame popped: mana, ready and circle are gone
```

### The heap — the ethereal plane

For data whose size is only known at runtime (text the player types, a growing list of spells), you ask the allocator for space on the **heap** and keep a **pointer** to it on the stack.

```rust
let incantation = String::from("Ignis");
```

```text
   Stack (incantation)            Heap
  ┌──────────┬──────┐          ┌───┬───┬───┬───┬───┐
  │ ptr      │ ─────┼────────▶ │ I │ g │ n │ i │ s │
  │ len  = 5 │      │          └───┴───┴───┴───┴───┘
  │ cap  = 5 │      │
  └──────────┴──────┘
```

A `String` is a **three-word handle** (pointer, length, capacity) on the stack — 24 bytes on a 64-bit machine, *no matter how long the text is*. `Vec<T>` has exactly the same shape.

A string literal `"Ignis"` is different: it is baked into the binary. A `&str` is just a view — pointer + length, 16 bytes — onto text that lives somewhere else.

### Measuring with `size_of`

```rust
assert_eq!(size_of::<u64>(), 8);              // in the prelude since Rust 1.80
let s = String::from("a very long incantation indeed");
assert_eq!(size_of_val(&s), 24);              // the handle, not the text
assert_eq!(s.len(), 30);                      // the heap part, in bytes
```

<div class="compare">

🐍 **Python:** *everything* is a heap object — even `42` is a boxed `int` with a refcount. You never choose.

🐹 **Go:** the compiler's *escape analysis* decides: locals stay on the stack unless they escape, then the GC tracks them.

🦀 **Rust:** you can see it in the type. `u32`, arrays and tuples of them live inline; `String`, `Vec`, `Box` own heap memory. And no GC: the owner frees it (next quest).

</div>

### Why should a Pythonista care?

Because the heap is where the **danger** is. Stack values die with their frame — nothing to manage. Heap memory has to be freed *exactly once*, *after* the last use. Free too early: use-after-free. Twice: double free. Never: leak. Python and Go hire a garbage collector for this. Rust's answer is ownership, and it all starts from this picture.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`.len()` counts bytes, not characters.** Strings are UTF-8: `"🔥".len()` is 4. Use `.chars().count()` for runes.
- **`char` is 4 bytes**, not 1. A Rust `char` holds any Unicode scalar value — closer to Go's `rune` than C's `char`.
- **Big arrays can blow the stack.** `let huge = [0u8; 50_000_000];` overflows the (typically 8 MB) main thread stack. Large data belongs on the heap: `vec![0u8; 50_000_000]`.
- **Tuples can be bigger than their parts.** `(u8, u32)` is 8 bytes, not 5: fields get *aligned*. Don't predict struct sizes by adding up fields.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/02_ownership_chronicles/00_stack_heap`

The Oracle's Quiz: each function returns your *prediction* of a size in bytes, as a plain number literal. Guess, run `wr`, and let the failing assertion teach you the truth. Then implement `heap_bytes`.

</div>

Yes, you could cheat with `size_of::<String>()` — but then you'd learn nothing. The goal is that `24` for a `String` feels obvious by the end.

## 📜 The Grimoire

<div class="grimoire">

- **Stack:** fixed-size, LIFO, freed when the function returns. Scalars, arrays, tuples.
- **Heap:** dynamic size, reached through a pointer. `String`, `Vec<T>`, `Box<T>`.
- `String` / `Vec<T>` handle = ptr + len + cap = 24 bytes (64-bit)
- `&str` / `&[T]` = ptr + len = 16 bytes ("fat pointer")
- `size_of::<T>()` · `size_of_val(&x)` — stack size; `.len()` — bytes of content
- `char` = 4 bytes; UTF-8 text uses 1–4 bytes per char

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — the scalars</summary>

The number in the type name is the number of *bits*: `u8` → 8 bits → 1 byte, `u32` → 4 bytes. A `char` is always 4 bytes.

</details>
<details class="hint"><summary>Hint 2 — the handles</summary>

Count the machine words (8 bytes each): `&str` = pointer + length. `String` and `Vec` = pointer + length + capacity. An array is just its elements back to back: 5 × 4.

</details>
<details class="hint"><summary>Hint 3 — heap_bytes</summary>

The answers are `1, 4, 4, 20, 16, 24, 24`. For `heap_bytes`, `String` has a method that returns its length in bytes — the one you already used in Chapter 1.

</details>
