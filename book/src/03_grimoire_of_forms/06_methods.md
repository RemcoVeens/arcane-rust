# ⚔️ 3.6 — Methods: Breathing Life into Forms

> *"A spell does not merely sit in a book. It can be cast, empowered, described. Structs hold the data; methods give them behaviour."*

## ⚡ The Concept

### `impl` blocks

Functions that belong to a type go in an `impl` block:

```rust
pub struct Mage {
    pub name: String,
    pub mana: u32,
}

impl Mage {
    pub fn can_cast(&self, cost: u32) -> bool {
        self.mana >= cost
    }
}

let tiberius = Mage { name: String::from("Tiberius"), mana: 40 };
tiberius.can_cast(30);          // method-call syntax
Mage::can_cast(&tiberius, 30);  // ...which is sugar for this
```

A type can have several `impl` blocks. As in Go, you can only add methods in the crate (Go: package) that defines the type.

### The three receivers

The first parameter decides what the method may do with the value — it's ownership again, written in one word:

| Receiver | Short for | The method… | Caller needs |
|---|---|---|---|
| `&self` | `self: &Self` | reads | any binding |
| `&mut self` | `self: &mut Self` | changes fields | a `let mut` binding |
| `self` | `self: Self` | **consumes** the value | ownership (it's moved) |

```rust,ignore
impl Mage {
    pub fn rest(&mut self) {               // changes the mage
        self.mana = self.max_mana;
    }

    pub fn retire(self) -> String {        // the mage is gone afterwards
        format!("{} retires.", self.name)
    }
}

let mut m = Mage::new("Tiberius");
m.rest();                 // auto-borrows &mut m
let farewell = m.retire();
// m.rest();              // ❌ use of moved value: `m`
```

You never write `(&mut m).rest()` — the dot **auto-borrows** for you. That's why `&mut` doesn't appear at call sites of methods, unlike function calls in Chapter 2.

`Self` (capital S) is an alias for the type the `impl` is for.

### Associated functions — no `self` at all

Functions in an `impl` without a `self` parameter are called with `::`. The most common one is a constructor, conventionally named `new`:

```rust,ignore
impl Mage {
    pub fn new(name: &str) -> Self {
        Self { name: String::from(name), level: 1, mana: 100, max_mana: 100 }
    }
}

let m = Mage::new("Tiberius");
```

`new` is not a keyword — just a convention. Nothing stops you from having `Mage::apprentice(...)` and `Mage::archmage(...)` too. You've used associated functions all along: `String::from`, `String::new`, `Vec::new`.

### Methods on enums

`impl` works for enums too. For small `Copy` enums, taking `self` by value is idiomatic — it's just a byte:

```rust,ignore
impl Element {
    pub fn beats(self, other: Element) -> bool {
        matches!((self, other), (Element::Fire, Element::Frost) | /* … */)
    }
}

Element::Fire.beats(Element::Frost);   // true
```

<div class="compare">

🐍 **Python:** `def rest(self):` — `self` is always a mutable reference; `@staticmethod` / `@classmethod` for constructors.

🐹 **Go:** `func (m *Mage) Rest()` vs `func (m Mage) Describe()` — pointer vs value receivers, very close to `&mut self` vs `self`! But Go has no read-only pointer receiver, and value receivers *copy* rather than consume.

🦀 **Rust:** `&self` / `&mut self` / `self` spell out read, write, consume — and the borrow checker enforces it.

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Calling `&mut self` methods on an immutable binding.** `let m = Mage::new(..); m.rest();` → "cannot borrow `m` as mutable". Make the binding `let mut`.
- **Calling one method from another.** Inside an `impl`, it's `self.can_cast(cost)`, not `can_cast(cost)`.
- **`self` receivers on non-Copy types** move the value. Great for "finish" operations (`retire`, `into_…`), surprising if you meant `&self`.
- **No overloading, no default arguments.** Two methods need two names: `new` and `with_level`, for example.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/03_grimoire_of_forms/06_methods`

Give the `Mage` a constructor and behaviour — `can_cast`, `cast`, `rest`, `level_up`, `title`, and a consuming `retire` — and give `Element` an `all()` associated function and a `beats` method.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `impl Type { … }` — methods and associated functions
- `&self` read · `&mut self` write · `self` consume
- `Type::func(…)` for associated fns (`Mage::new`) · `value.method(…)` for methods
- `Self` = the type being implemented
- Call other methods via `self.other()`
- The dot auto-borrows: `m.rest()` ≙ `Mage::rest(&mut m)`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — new, can_cast, rest</summary>

`new` returns `Mage { name: String::from(name), level: 1, mana: STARTING_MANA, max_mana: STARTING_MANA }` (`Self { … }` works too). `can_cast` is a one-line comparison; `rest` assigns `self.mana = self.max_mana;`.

</details>
<details class="hint"><summary>Hint 2 — cast & level_up</summary>

`if self.can_cast(cost) { self.mana -= cost; true } else { false }`. `level_up` is three assignments — refill *after* raising `max_mana`.

</details>
<details class="hint"><summary>Hint 3 — title, retire, Element</summary>

Pick the rank with a `match self.level { 0..=4 => "Apprentice", 5..=9 => "Adept", _ => "Archmage" }`, then `format!("{} the {rank}", self.name)`. `Element::all()` returns an array literal; `beats` is a `matches!` on `(self, other)` with the three winning pairs.

</details>
