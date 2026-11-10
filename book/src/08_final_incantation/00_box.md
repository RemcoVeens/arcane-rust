# ⚔️ 8.0 — Box: The Enchanted Chest

> *"The apprentice tried to write a combo scroll that contained another combo scroll. The scroll grew, and grew, and swallowed the library. The master sighed and handed her a small chest: 'Put the inner scroll in here. The chest is always the same size — no matter what's inside.'"*

## ⚡ The Concept

### `Box<T>`: a value on the heap, with one owner

```rust
let power = Box::new(45);   // the 45 lives on the heap; `power` (a pointer) on the stack
assert_eq!(*power + 1, 46); // deref with `*` (often automatic)
// `power` goes out of scope → the heap memory is freed. No GC, no `free()`.
```

A `Box` behaves like the value it holds: ownership, moves and borrowing all work exactly as before. It's just stored somewhere else. You use it for three reasons:

1. **Recursive types** — a type that contains itself (this quest).
2. **Trait objects** — `Box<dyn Trait>` when the concrete type is only known at runtime (Chapter 6 introduced them; this quest returns them from a factory).
3. Moving a very large value cheaply (only the pointer moves).

<div class="compare">

🐍 / 🐹 Every Python object is already "boxed" on the heap, and Go's escape analysis puts things on the heap for you. · 🦀 Rust keeps values on the stack by default and makes heap allocation explicit: `Box::new`.

</div>

### Recursive types

The compiler must know every type's size. This enum contains itself:

```rust,compile_fail
enum Combo {
    Spell(u32),
    Chain(Combo, Combo), // ❌ error[E0072]: recursive type `Combo` has infinite size
}
```

How big is a `Combo`? Big enough to hold two `Combo`s, each big enough to hold two `Combo`s… The compiler's `help:` says it: *insert some indirection (e.g., a `Box`)*. A `Box` is always pointer-sized:

```rust
enum Combo {
    Spell(u32),
    Chain(Box<Combo>, Box<Combo>), // ✅ two pointers
}

use Combo::*;
let combo = Chain(Box::new(Spell(45)), Box::new(Spell(35)));
```

### Recursion over recursive types

`match` + recursion. Inside a `match` on `&self`, a `Box<Combo>` field binds as `&Box<Combo>`, and method calls auto-deref through the box:

```rust,ignore
fn power(&self) -> u32 {
    match self {
        Combo::Spell(p) => *p,
        Combo::Chain(a, b) => a.power() + b.power(), // a: &Box<Combo>, .power() just works
    }
}
```

Small constructor helpers (`Combo::chain(a, b)` doing the `Box::new` for you) keep call sites readable.

### `Box<dyn Trait>` from a factory

A function can return *different* concrete types behind one trait:

```rust,ignore
fn forge(kind: &str) -> Option<Box<dyn Ward>> {
    match kind {
        "ice" => Some(Box::new(IceWard)),     // Box<IceWard> coerces to Box<dyn Ward>
        _ => None,
    }
}
```

`impl Ward` wouldn't work here: `impl Trait` in return position means *one* concrete type.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`Option<Combo>` doesn't help** with recursion — an `Option` stores its contents inline. Only a pointer (`Box`, `Rc`, `&`) breaks the cycle. (`Vec<Combo>` also works: a `Vec` keeps its elements on the heap.)
- **Don't box everything.** `Box<u32>` for a lone number is just slower. Box when you need one of the three reasons above.
- **Very deep recursion** can still overflow the *call* stack at runtime (millions of levels). Fine for game-sized trees.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/08_final_incantation/00_box`

The `Combo` enum **does not compile**: fix it with `Box`. Then implement the constructors, `power`, `spell_count` and `describe` recursively, plus a `forge_ward` factory returning `Box<dyn Ward>` and `damage_through` a stack of wards.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `Box::new(v)` → heap allocation, single owner, freed on drop
- Recursive enum: `Chain(Box<Combo>, Box<Combo>)`
- `match self { Chain(a, b) => a.power() + b.power(), … }` — auto-deref
- `Box<dyn Trait>`: different concrete types, one interface
- `iter().fold(start, |acc, x| …)` — thread a value through a sequence

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — the compile error</summary>

Change `Chain(Combo, Combo)` to `Chain(Box<Combo>, Box<Combo>)` and `Amplify(Combo, u32)` to `Amplify(Box<Combo>, u32)`. Then `chain` becomes `Combo::Chain(Box::new(first), Box::new(second))`.

</details>
<details class="hint"><summary>Hint 2 — describe</summary>

Three arms: `Spell { name, .. } => name.clone()`, `Chain(a, b) => format!("({} + {})", a.describe(), b.describe())`, and an `Amplify` arm in the same style using `x{factor}`.

</details>
<details class="hint"><summary>Hint 3 — wards</summary>

`forge_ward` is a `match` on `kind` returning `Some(Box::new(IceWard))` and so on. `damage_through` is `wards.iter().fold(damage, |dmg, ward| ward.absorb(dmg))`.

</details>
