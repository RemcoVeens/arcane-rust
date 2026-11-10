# ⚔️ 2.4 — References: Borrowing the Grimoire

> *"A wise mage does not give away their grimoire to every apprentice who wants a look. They lend it — and expect it back, unharmed."*

## ⚡ The Concept

### `&T` — a shared borrow

Returning tuples just to give a value back gets old fast. Instead, *lend* it:

```rust
fn rune_count(grimoire: &String) -> usize {   // takes a reference
    grimoire.chars().count()
}                                              // nothing is dropped: we never owned it

let grimoire = String::from("ᚠᚢᚦ runes");
let n = rune_count(&grimoire);                 // `&` creates the reference
println!("{grimoire} has {n} runes");          // ✅ still ours
```

```text
  grimoire ──────────▶ "ᚠᚢᚦ runes"   (heap)
      ▲
  &grimoire (a pointer to the owner, not a new owner)
```

A reference is a pointer that is **guaranteed valid** and **does not own**. When it goes away, nothing is freed. Calling methods works straight through it — `grimoire.chars()` — the dot auto-dereferences.

### `&mut T` — an exclusive, mutable borrow

To let a function change something it doesn't own, lend it mutably:

```rust
const MAX_MANA: u32 = 100;

fn recharge(mana: &mut u32, amount: u32) {
    *mana = (*mana + amount).min(MAX_MANA);   // `*` = "the value behind the reference"
}

let mut mana = 40;          // the owner must be `mut`...
recharge(&mut mana, 25);    // ...and the borrow must say `&mut` too
assert_eq!(mana, 65);
```

With `&mut String` you can call mutating methods directly — `scroll.push('ᚦ')` — because method calls auto-dereference. Plain operators like `+=` or `=` on a `&mut u32` need the explicit `*`.

<div class="compare">

🐍 **Python:** every argument is effectively a shared, mutable reference. A function can mutate your list behind your back — and you won't know from the call site.

🐹 **Go:** `func recharge(mana *uint32)` and `recharge(&mana)` — very similar! But Go pointers can be `nil`, can alias freely, and can outlive what they point to (the GC keeps it alive).

🦀 **Rust:** `&mut` is visible at the **call site**, is never null, and (next quest) never aliases. If a call doesn't say `&mut`, your data won't change.

</div>

### `&` and `*` side by side

| Expression | Meaning |
|---|---|
| `&x` | borrow `x` (read-only) |
| `&mut x` | borrow `x` exclusively (read-write) |
| `*r` | the value `r` points to |
| `*r = v` | write through a `&mut` |
| `r.method()` | auto-deref: works on `T`, `&T`, `&mut T` |

### Borrowing arrays

Passing `[u32; 5]` by value copies all 20 bytes (arrays of Copy types are Copy). For big arrays, borrow instead:

```rust
fn circle_cost(circle: &[u32; 5]) -> u32 {
    let mut total = 0;
    for cost in circle {        // iterating a &array yields &u32 items
        total += cost;          // `+=` accepts a &u32 on the right
    }
    total
}
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Forgetting `*`.** `mana += 5` on a `&mut u32` → "cannot use `+=` on type `&mut u32`". Write `*mana += 5`.
- **`&mut` needs a `mut` owner.** You can't lend out write access to something you yourself can't write.
- **References don't extend lifetimes.** Unlike Go, a reference can't keep its owner alive. The owner must outlive the borrow — the compiler checks (next quest).
- **`&String` parameters are a smell.** It works, but `&str` accepts more callers. We'll fix that in Quest 2.6 — clippy already grumbles about it.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/02_ownership_chronicles/04_references`

Read through `&`, write through `&mut`: count runes, shout a spell, recharge mana with a cap, write runes in place, swap two mana pools, and sum a borrowed spell circle.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `&x` lend read-only · `&mut x` lend read-write (owner must be `mut`)
- References never own: nothing is dropped when they go away
- `*r` reads/writes the value behind a reference
- Methods auto-deref: `s.len()`, `s.push('x')` work on `&String` / `&mut String`
- `.min(max)` caps a number · `.to_uppercase()` returns a new `String`
- `std::mem::swap(a, b)` swaps two `&mut` values

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — rune_count & shout</summary>

Both only read: `grimoire.chars().count()`; `format!("{}!", spell.to_uppercase())`.

</details>
<details class="hint"><summary>Hint 2 — recharge & add_rune</summary>

`*mana = (*mana + amount).min(MAX_MANA);`. For the scroll, `push` works directly on a `&mut String`.

</details>
<details class="hint"><summary>Hint 3 — swap_mana</summary>

Classic three-step: `let tmp = *a; *a = *b; *b = tmp;` (works because `u32` is Copy). The standard library has a one-liner for this too: `std::mem::swap`.

</details>
