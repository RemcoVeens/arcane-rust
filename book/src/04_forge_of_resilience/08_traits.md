# ⚔️ 4.8 — Traits: The Schools of Magic

> *"Fire magic, ice magic, rune magic — different schools, one principle: every enchantment has a name and a power. A trait captures what the schools share, and lets each school do it its own way."*

## ⚡ The Concept

### Defining a trait

A trait is a set of methods a type promises to have:

```rust
pub trait Enchantment {
    fn name(&self) -> String;     // required: no body
    fn power(&self) -> u32;

    fn describe(&self) -> String { // provided: a DEFAULT body
        format!("{} ({} power)", self.name(), self.power())
    }
}
```

A default method can call the required ones — it doesn't know which type it runs on, only that the type implements `Enchantment`.

### Implementing it

```rust,ignore
pub struct Fireball;                      // a "unit struct": no fields at all

impl Enchantment for Fireball {
    fn name(&self) -> String { "Fireball".to_string() }
    fn power(&self) -> u32 { 45 }
    // describe(): the default is used
}

pub struct Rune { pub glyph: char, pub strength: u32 }

impl Enchantment for Rune {
    fn name(&self) -> String { format!("Rune {}", self.glyph) }
    fn power(&self) -> u32 { self.strength * 10 }
    fn describe(&self) -> String {        // override the default
        format!("{} glows with {} power", self.glyph, self.power())
    }
}
```

<div class="compare">

🐹 Go interfaces are **implicit**: having the methods is enough. Rust traits are **explicit**: you write `impl Trait for Type`. · 🐍 Python protocols / ABCs are the closest relative. · 🦀 Bonus over Go: traits can have default methods, and you can implement *your* trait for *someone else's* type (e.g. `impl Enchantment for u32`).

</div>

### Traits as parameters

Two spellings, same meaning (static dispatch — a specialised copy per type, like generics):

```rust,ignore
fn announce(e: &impl Enchantment) -> String { format!("✨ {}", e.describe()) }

fn announce<T: Enchantment>(e: &T) -> String { format!("✨ {}", e.describe()) }
```

Use the generic form when you need to name the type more than once:

```rust,ignore
fn total_power<T: Enchantment>(items: &[T]) -> u32 { /* all items are the SAME type T */ }

fn mightier<A: Enchantment, B: Enchantment>(a: &A, b: &B) -> String { /* two different types */ }
```

### `derive`: free implementations

For the standard traits with an obvious implementation, the compiler writes it for you:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rune { pub glyph: char, pub strength: u32 }
```

| Derive | Gives you |
|---|---|
| `Debug` | `{:?}` printing |
| `Clone` | `.clone()` |
| `Copy` | implicit copies instead of moves (needs `Clone`; all fields must be `Copy`) |
| `PartialEq` / `Eq` | `==` and `assert_eq!` |
| `PartialOrd` / `Ord` | `<`, `>`, `.sort()` |
| `Hash` | use as a `HashMap` key (with `Eq`) |
| `Default` | `Type::default()` |

### Implementing a standard trait by hand: `Display`

`Display` can't be derived — only you know how your type should look to humans:

```rust,ignore
use std::fmt;

impl fmt::Display for Rune {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}:{}]", self.glyph, self.strength)
    }
}

println!("{}", rune);       // [ᚱ:3]
let s = rune.to_string();   // to_string() comes for free with Display
```

You did exactly this for your error type in Quest 4.6 — now you know what it was.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **The orphan rule:** you may implement a trait for a type only if the trait *or* the type is defined in your crate. `impl Display for Rune` ✅ · `impl Enchantment for Vec<i32>` ✅ · `impl Display for Vec<Rune>` ❌ (both `Display` and `Vec` are foreign — wrapping the `Vec` in your own struct fixes it).
- **Traits must be in scope** to call their methods. A method "not found" on a type that clearly implements the trait usually means a missing `use`.
- **`Copy` can't be derived** for types with a `String`/`Vec` field — they own heap memory.
- **Required vs default:** forgetting a required method is a compile error that lists exactly which ones are missing.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/04_forge_of_resilience/08_traits`

**This file does not compile yet.** Write the default `describe`, implement `Enchantment` for three schools (one overrides the default), derive the standard traits `Rune` needs, implement `Display`, and write three functions that work with *any* enchantment.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `trait T { fn required(&self) -> X; fn provided(&self) -> Y { ... } }`
- `impl T for MyType { fn required(&self) -> X { ... } }`
- Parameters: `x: &impl T` · `fn f<A: T>(x: &A)` · `fn f<A: T, B: T>(a: &A, b: &B)`
- `#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]`
- `impl fmt::Display for X { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, ...) } }`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — getting it to compile</summary>

Start with `#[derive(Debug, Clone, Copy, PartialEq)]` above `pub struct Rune`, then write the three `impl Enchantment for ...` blocks with the two required methods each. For `Rune`, add a third method `describe` to override the default.

</details>
<details class="hint"><summary>Hint 2 — Display and describe</summary>

`Display` is a `write!(f, "[{}:{}]", self.glyph, self.strength)`. The default `describe` uses `self.name()` and `self.power()` inside a `format!`.

</details>
<details class="hint"><summary>Hint 3 — the generic functions</summary>

`announce`: `format!("✨ {}", e.describe())`. `total_power`: a `for item in items` loop adding `item.power()`. `mightier`: `if b.power() > a.power() { b.name() } else { a.name() }` — strictly greater, so ties go to `a`.

</details>
