# ⚔️ 3.2 — Enums: The Choice of Forms

> *"A spell is Fire, Frost or Arcane. A potion is Health, Mana or plain Water. Some things are not 'and' — they are 'or'. That is an enum."*

## ⚡ The Concept

### Simple enums

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Fire,
    Frost,
    Arcane,
}

let e = Element::Fire;            // variants live in the enum's namespace
```

A value of type `Element` is **exactly one** of the variants — never something else, never null.

### Enums that carry data

This is where Rust enums leave Python's and Go's far behind. Each variant can hold its *own* data, of its own shape:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Potion {
    Health(u32),                  // tuple-like variant
    Mana(u32),
    Invisibility { turns: u32 },  // struct-like variant
    Water,                        // no data at all
}

let p = Potion::Invisibility { turns: 3 };
```

A `Potion` is a Health potion with an amount, *or* a Mana potion with an amount, *or* … You can't accidentally read the `turns` of a Health potion: that field doesn't exist in that variant. Type theorists call this a **sum type** or **tagged union**.

<div class="compare">

🐍 **Python:** `enum.Enum` gives named constants. For "variants with data" you'd write several dataclasses and a `Union[...]` type hint — and `isinstance` checks.

🐹 **Go:** `const ( Fire = iota; Frost; Arcane )` — just integers; `Element(42)` is perfectly legal. Data-carrying variants need an interface plus a type switch.

🦀 **Rust:** one `enum`, closed set of variants, each with its own payload, checked exhaustively by `match`.

</div>

### A first look at `match`

To *use* an enum you ask "which variant is it?" with `match`:

```rust
fn label(potion: &Potion) -> String {
    match potion {
        Potion::Health(hp) => format!("Health potion (+{hp} HP)"),
        Potion::Mana(m) => format!("Mana potion (+{m} mana)"),
        Potion::Invisibility { turns } => format!("Invisible for {turns} turns"),
        Potion::Water => String::from("Just water"),
    }
}
```

- Each **arm** is `pattern => expression`; the whole `match` is an expression.
- Patterns **bind** the variant's data to names (`hp`, `turns`).
- `match` must be **exhaustive**: forget a variant and it won't compile. Add a variant later and the compiler lists every `match` you need to update.
- `_ => …` is a catch-all arm for "everything else".

Quest 3.4 is all about `match`; this is enough for now.

### Explicit discriminants

Fieldless enums can be given numbers, and cast to integers with `as`:

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Apprentice = 1,
    Adept = 5,
    Archmage = 10,
}

assert_eq!(Rank::Adept as u32, 5);
```

The cast only goes *one way*: there's no `5 as Rank` — an arbitrary number might not be a valid rank.

### Size

An enum is as big as its largest variant plus a small tag. `Element` is 1 byte. `Potion` is 8 bytes (a `u32` + tag, aligned).

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Forgetting the enum name.** `Fire` alone is not in scope — write `Element::Fire`, or `use Element::*;` inside a function to import the variants.
- **Matching on a reference.** `match potion` where `potion: &Potion` works fine; the bindings (`hp`) become `&u32`. `format!` doesn't care, but arithmetic needs `*hp`.
- **Overusing `_`.** A catch-all arm silences the exhaustiveness check: when you add a new variant later, the compiler won't remind you. Prefer listing variants when there are few.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/03_grimoire_of_forms/02_enums`

❌ The file **does not compile** until you define the `Potion` enum. Then implement the wheel of elements, element runes, potion labels, mana restoration and rank levels.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `enum E { A, B(u32), C { x: u32 } }` — unit, tuple and struct variants
- Build: `E::A`, `E::B(5)`, `E::C { x: 1 }`
- `match value { E::A => …, E::B(n) => …, E::C { x } => … }`
- `_ => …` catches the rest · match must be exhaustive
- `enum Rank { A = 1, B = 5 }` + `rank as u32`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — Potion</summary>

```rust,ignore
#[derive(Debug, Clone, PartialEq)]
pub enum Potion {
    Health(u32),
    // … three more
}
```

</details>
<details class="hint"><summary>Hint 2 — strong_against & rune</summary>

Three arms each: `Element::Fire => Element::Frost,` and so on. Char literals use single quotes: `'🔥'`.

</details>
<details class="hint"><summary>Hint 3 — label & mana_restored</summary>

`label` needs all four arms, binding the data: `Potion::Invisibility { turns } => format!("Invisibility potion ({turns} turns)")`. For `mana_restored`, two arms are enough: `Potion::Mana(m) => *m,` and `_ => 0`.

</details>
