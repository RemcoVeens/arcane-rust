# ⚔️ 4.9 — Trait Objects: The Shapeshifter's Art

> *"A goblin, a wraith and a dragon walk into your battlefield. Different shapes, different strengths — yet each one can be hit, and each one hits back. To face them all at once, you must see past their shapes to what they share."*

## ⚡ The Concept

### The limit of generics

A `Vec<T>` holds values of **one** type. With generics, `T` is decided at compile time:

```rust,compile_fail
let horde = vec![Goblin { hp: 30 }, Dragon { hp: 200 }]; // ❌ expected `Goblin`, found `Dragon`
```

### Trait objects: `dyn Trait`

`dyn Creature` means "some type that implements `Creature` — decided at **runtime**". Different types have different sizes, so a `dyn Creature` always lives behind a pointer: `&dyn Creature`, `&mut dyn Creature` or `Box<dyn Creature>`.

```rust,ignore
let horde: Vec<Box<dyn Creature>> = vec![
    Box::new(Goblin { hp: 30 }),
    Box::new(Dragon { hp: 200 }),
];

for creature in &horde {
    println!("{} attacks for {}", creature.name(), creature.attack());
}
```

`Box<T>` is the simplest smart pointer: it puts a value on the heap and owns it. `Box::new(Goblin {..})` is a `Box<Goblin>`; the compiler converts it to a `Box<dyn Creature>` because the vector's type says so.

<div class="compare">

🐹 Go: every interface value *is* a trait object — `[]Creature{goblin, dragon}` just works. · 🐍 Python: every list is a list of trait objects; duck typing does the dispatch. · 🦀 Rust makes you choose: generics (static, fastest) or `dyn` (dynamic, flexible). Writing `dyn` makes the runtime cost visible.

</div>

### How it works: the vtable

A `&dyn Creature` is a **fat pointer**: one pointer to the data, one to a *vtable* — a table of function pointers for that concrete type. Calling `creature.attack()` looks up `attack` in the table. That's one extra indirection per call, the same as a Go interface call or a C++ virtual method.

### Working with trait objects

```rust,ignore
// Accept ANY creature by reference — works for a plain Goblin AND for a boxed one
fn describe(c: &dyn Creature) -> String { format!("{} ({} hp)", c.name(), c.hp()) }

describe(&Goblin { hp: 30 });
describe(horde[0].as_ref());      // Box<dyn Creature> → &dyn Creature

// Mutate every creature in place
fn fireball_all(horde: &mut [Box<dyn Creature>], damage: u32) {
    for creature in horde.iter_mut() {   // creature: &mut Box<dyn Creature>
        creature.take_damage(damage);    // method calls see through the Box
    }
}
```

### Generics or trait objects?

| | Generics `<T: Creature>` | Trait objects `dyn Creature` |
|---|---|---|
| Decided | compile time | runtime |
| Mixed types in one `Vec` | ❌ | ✅ |
| Speed | inlined, fastest | one indirection |
| Code size | one copy per type | one copy |

Rule of thumb: use generics by default; use `dyn` when you need a **heterogeneous collection** or want to choose the type at runtime.

### Dyn compatibility ("object safety")

Not every trait can become a `dyn Trait`. Roughly: methods must take `self` by reference (`&self` / `&mut self`), must not be generic, and must not return `Self`. A trait with `fn clone(&self) -> Self` can't be a trait object — the caller wouldn't know the size of the result. The compiler tells you exactly which method is the problem.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`dyn Trait` needs a pointer.** `Vec<dyn Creature>` doesn't compile ("size cannot be known at compilation time"). Use `Vec<Box<dyn Creature>>`.
- **The concrete type is forgotten.** Once it's a `Box<dyn Creature>`, you can only call `Creature` methods — no access to `Dragon`-specific fields.
- **Annotate the Vec.** `vec![Box::new(Goblin{..}), Box::new(Dragon{..})]` without a type annotation fails: the compiler infers `Box<Goblin>` from the first element. Tell it `Vec<Box<dyn Creature>>` (or put it in the function's return type).

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/04_forge_of_resilience/09_trait_objects`

`Goblin` is done. Implement `Creature` for the shadowy `Wraith` (half damage) and the `Dragon` (enraged below 100 hp), spawn a mixed horde in a `Vec<Box<dyn Creature>>`, and write functions that describe, damage and count it.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `Box::new(value)` → heap-allocated, owned
- `Vec<Box<dyn Trait>>` — mixed types, one collection
- `&dyn Trait` / `&mut dyn Trait` as parameters
- `boxed.as_ref()` → `&dyn Trait` · `boxed.as_mut()` → `&mut dyn Trait`
- `for x in &v` (read) · `for x in v.iter_mut()` (modify)
- Generics: static dispatch · `dyn`: dynamic dispatch via a vtable

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — Wraith and Dragon</summary>

Copy the shape of `impl Creature for Goblin`. Wraith: `amount / 2` is integer division (rounds down), combined with `saturating_sub`. Dragon's `attack`: `if self.hp < 100 { 40 } else { 20 }`.

</details>
<details class="hint"><summary>Hint 2 — spawn_horde and describe</summary>

The return type already says `Vec<Box<dyn Creature>>`, so `vec![Box::new(Goblin { hp: 30 }), Box::new(Wraith { hp: 40 }), Box::new(Dragon { hp: 200 })]` is enough. `describe` branches on `creature.is_alive()` and `format!`s the name and hp.

</details>
<details class="hint"><summary>Hint 3 — the horde functions</summary>

`total_threat` and `survivors`: `for creature in horde`, skip the dead ones with `if creature.is_alive()`, add `creature.attack()` / push `creature.name().to_string()`. `fireball_all`: `for creature in horde.iter_mut() { creature.take_damage(damage); }`.

</details>
