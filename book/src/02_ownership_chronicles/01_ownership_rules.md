# ⚔️ 2.1 — The Ownership Rules: Three Laws of Magic

> *"Three laws govern all magic. A spell belongs to one caster. There is only ever one caster at a time. When the caster leaves, the spell fades. Break them, and the compiler rejects your incantation."*

## ⚡ The Concept

### The three laws

1. Each value in Rust has an **owner** (a variable, a struct field, a Vec slot…).
2. There is only **one owner at a time**.
3. When the owner goes **out of scope**, the value is **dropped** (its memory freed).

```rust
fn tower() {
    let staff = String::from("Staff of Embers"); // `staff` owns the heap text
    // ... use staff ...
}   // ← `staff` goes out of scope: the String is dropped, heap memory freed
```

That closing brace is where Python's refcount would drop to zero, or where Go's GC would *eventually* notice. In Rust it's decided at compile time: the compiler inserts the cleanup right there. No collector, no pauses, no leaks.

<div class="compare">

🐍 **Python:** `a = [1, 2]; b = a` → two names, one list, refcount 2. Freed when the count hits zero.

🐹 **Go:** `b := a` on a slice → both share the backing array; the GC frees it someday.

🦀 **Rust:** `let b = a;` with a `String` → **`b` is now the only owner**, `a` is unusable. Freed exactly when `b`'s scope ends.

</div>

### Ownership travels through functions

Passing a value to a function hands ownership to the parameter. Returning a value hands it back out.

```rust
fn forge() -> String {
    let blade = String::from("Runeblade");
    blade                       // ownership moves out to the caller
}

fn melt(blade: String) {
    println!("{blade} melts...");
}                               // `blade` dropped here

fn main() {
    let sword = forge();        // main owns it now
    melt(sword);                // melt owns it now — and drops it
    // println!("{sword}");     // ❌ sword no longer owns anything
}
```

### Giving it back

Before you learn borrowing (Quest 2.4), the only way to let a function *look* at a value and still keep it is to have the function **return it**:

```rust
fn weigh(scroll: String) -> (String, usize) {
    let weight = scroll.len();
    (scroll, weight)            // hand the scroll back, plus the answer
}

let scroll = String::from("Scroll of Mending");
let (scroll, weight) = weigh(scroll);   // shadowing: rebind the returned owner
```

Clumsy? Yes — on purpose. Feel the friction now, and references will feel like a gift.

### Mutating something you own

Owning a value lets you change it — you just need a `mut` binding. A parameter can be `mut` too:

```rust
fn polish(mut blade: String) -> String {
    blade.push_str(" (polished)");
    blade
}
```

### Building `String`s

```rust
let a = String::from("Ignis");          // from a literal
let b = format!("{a} Maxima");           // format! always builds a new String
let mut c = String::new();               // empty
c.push('ᚠ');                             // add a char
c.push_str("ire");                       // add a &str
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **"Dropped" is about the owner's scope, not the last use.** The value lives until the closing `}` of the block that owns it (or until ownership moves elsewhere).
- **`mut` belongs to the binding, not the value.** `fn polish(mut blade: String)` is the function saying "I'll modify my own copy of the handle" — the caller doesn't need `mut` at all.
- **Ownership ≠ the stack frame.** A `String` moved into a function is freed at the end of *that* function, not the caller's.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/02_ownership_chronicles/01_ownership_rules`

Implement five small functions that create, transform, give back and drop owned `String`s. No references allowed — values go in, values come out.

</div>

## 📜 The Grimoire

<div class="grimoire">

- One value, one owner. Owner leaves scope → value dropped.
- Passing a `String` to a function moves ownership into the parameter.
- Returning a value moves ownership out to the caller.
- "Lend and get back" without references: return a tuple `(value, answer)`.
- `String::from`, `String::new`, `format!`, `.push(char)`, `.push_str(&str)`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — forge_staff & merge</summary>

`format!` returns an owned `String`, so both are one-liners. Placeholders can name variables directly: `format!("{first} {second}")`.

</details>
<details class="hint"><summary>Hint 2 — inscribe</summary>

You own `scroll`, but the binding isn't `mut`. Either write `mut scroll: String` in the parameter list, or shadow it: `let mut scroll = scroll;`. Then `scroll.push(rune)` and return it.

</details>
<details class="hint"><summary>Hint 3 — weigh & count_and_drop</summary>

Compute the length *first*, store it in a `usize`, then return `(scroll, weight)`. For the chest: create `String::from("gold")`, return its `.len()`; the String itself is dropped at the closing brace.

</details>
