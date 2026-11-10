# ⚔️ 3.0 — Structs: Crafting Vessels of Magic

> *"A potion bottle has labelled compartments: base, catalyst, stabiliser. Pour the wrong thing in the wrong chamber and it explodes. A struct is the same — named fields that hold related data, each in its place."*

## ⚡ The Concept

### Defining and building a struct

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Mage {
    pub name: String,
    pub level: u32,
    pub mana: u32,
    pub max_mana: u32,
}

let tiberius = Mage {
    name: String::from("Tiberius"),
    level: 7,
    mana: 40,
    max_mana: 100,
};
println!("{} has {} mana", tiberius.name, tiberius.mana);   // dot access
```

Every field must be given a value — there are no default constructors and no `None`/`nil` fields unless you ask for them. `pub` makes a field visible outside the module (modules come in a later chapter); our game code and integration tests use the fields directly, so they are `pub`.

### `#[derive(...)]` — let the compiler write the boring code

| Derive | Gives you | Python equivalent |
|---|---|---|
| `Debug` | `println!("{:?}", mage)` and `{:#?}` pretty-print | `__repr__` |
| `PartialEq` | `mage_a == mage_b` (field by field) — needed by `assert_eq!` | `__eq__` |
| `Clone` | `mage.clone()` (clones every field) | `copy.deepcopy` |
| `Copy` | implicit copies — only if *every* field is Copy | — |

A derive is the compiler implementing a *trait* for you; writing traits by hand is Chapter 4. For now: derive what you need.

### Mutability is all-or-nothing

```rust
let mut tiberius = Mage { name: String::from("Tiberius"), level: 7, mana: 40, max_mana: 100 };
tiberius.mana += 25;          // needs `let mut` on the whole binding
```

You can't mark a single field as mutable. Mutability belongs to the *binding*, as in Chapter 1.

### Field init shorthand

When a variable has the same name as the field, write it once:

```rust
fn apprentice(name: String) -> Mage {
    Mage { name, level: 1, mana: 100, max_mana: 100 }   // `name` = `name: name`
}
```

### Struct update syntax

Build a new struct from an old one, changing just some fields:

```rust
let promoted = Mage { level: tiberius.level + 1, ..tiberius };
```

`..tiberius` fills in the remaining fields — and **moves** them. Since `name` is a `String`, `tiberius` is (partially) moved afterwards. Ownership rules still apply!

### Destructuring

```rust
let Mage { name, level, .. } = &tiberius;   // name: &String, level: &u32
println!("{name} is level {level}");
```

`..` ignores the rest. Destructuring through a reference gives you references to the fields.

<div class="compare">

🐍 **Python:** `@dataclass class Mage: name: str; level: int` — attributes can still be added, removed, or set to `None` at runtime.

🐹 **Go:** `type Mage struct { Name string; Level uint32 }` — very close! But Go zero-fills missing fields silently; Rust makes you set every one. Capitalisation means public in Go; `pub` does in Rust.

🦀 **Rust:** fixed shape, every field initialised, mutability per binding, and the borrow checker tracks fields individually.

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`assert_eq!` needs `PartialEq` *and* `Debug`.** Forgetting the derive gives a long error about a missing trait "`Mage: PartialEq`" — add the derive.
- **`..old` moves non-Copy fields.** After `Mage { level: 2, ..old }`, using `old.name` is a use-after-move.
- **No zero values.** Missing a field is a compile error, not a zero value. Good — but it means adding a field breaks every constructor. Constructor functions (and later `impl` blocks) keep that in one place.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/03_grimoire_of_forms/00_structs`

❌ The file **does not compile** until you define the `Potion` struct. Then build apprentices, promote them with struct update syntax, brew and drink potions, describe a mage and compute a spell's efficiency by destructuring.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `struct Name { field: Type, … }` · build with `Name { field: value, … }`
- `#[derive(Debug, Clone, PartialEq)]` — printing, cloning, comparing
- Field init shorthand: `Mage { name, … }`
- Update syntax: `Mage { level: 2, ..old }` (moves the rest out of `old`)
- Destructure: `let Spell { cost, power, .. } = spell;`
- `&mut Mage` lets you change fields: `mage.mana += 1`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — Potion & brew</summary>

Copy the shape of `Spell`: a `#[derive(...)]` line, `pub struct Potion { pub name: String, pub strength: u32 }`. `brew` then is `Potion { name: String::from(name), strength }`.

</details>
<details class="hint"><summary>Hint 2 — apprentice & promote</summary>

Shadow the `&str` into a `String` first (`let name = String::from(name);`) to use the shorthand. For `promote`, compute `let max_mana = mage.max_mana + 10;` and then `Mage { level: mage.level + 1, max_mana, mana: max_mana, ..mage }`.

</details>
<details class="hint"><summary>Hint 3 — efficiency</summary>

After `let Spell { cost, power, .. } = spell;` both are `&u32` (because `spell` is a reference) — dereference with `*`. Guard the zero case with `if`, and convert with `as f64` before dividing.

</details>
