# ⚔️ 4.6 — Custom Errors: Forging Your Own Failure Modes

> *"Not all failures are the same. A fizzled spell, an unknown incantation, a mumbled number — each deserves its own name. A mage who only ever says 'something went wrong' is no mage at all."*

## ⚡ The Concept

`String` errors can be *printed*, but not *inspected*: a caller that wants to react differently to "not enough mana" and "unknown spell" would have to compare strings. The Rust way is an **enum of everything that can go wrong**:

```rust
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum SpellError {
    UnknownSpell(String),
    NotEnoughMana { needed: u32, available: u32 },
    BadNumber(ParseIntError),
}
```

Now callers can `match` on the error, and the compiler tells them when a new variant appears.

<div class="compare">

🐹 Go: sentinel errors (`var ErrNotFound = errors.New(...)`) and custom error structs checked with `errors.Is`/`errors.As`. · 🐍 Python: a hierarchy of exception classes. · 🦀 One enum, matched exhaustively. No casting, no forgotten case.

</div>

To be a *proper* error, the type needs three things. They are all **trait implementations** — Quest 4.8 explains traits in depth; for now, the syntax `impl TraitName for Type { ... }` means "here is how `Type` does `TraitName`".

### 1. `Display` — the human-readable message

`Debug` (derived) is for developers; `Display` is what `{}` and `.to_string()` print:

```rust,ignore
use std::fmt;

impl fmt::Display for SpellError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpellError::UnknownSpell(name) => write!(f, "unknown spell '{name}'"),
            SpellError::NotEnoughMana { needed, available } => {
                write!(f, "not enough mana: need {needed}, have {available}")
            }
            SpellError::BadNumber(e) => write!(f, "bad number: {e}"),
        }
    }
}
```

`write!` is `format!` that writes into the formatter `f` instead of creating a `String`.

### 2. `std::error::Error` — the "I am an error" marker

```rust,ignore
impl std::error::Error for SpellError {}
```

That's it — the trait's methods all have defaults. It requires `Debug + Display`, which you now have. Once implemented, your error fits into a `Box<dyn Error>` and plays well with the rest of the ecosystem.

Optionally, override `source()` to expose an **underlying cause**, building an error chain:

```rust,ignore
impl std::error::Error for SpellError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SpellError::BadNumber(e) => Some(e),
            _ => None,
        }
    }
}
```

(The `'static` is a lifetime — Chapter 5. Copy the signature as-is.)

### 3. `From` — so `?` can convert for you

Remember that `?` *converts* errors? It calls `From::from`. Teach it how a `ParseIntError` becomes a `SpellError`:

```rust,ignore
impl From<ParseIntError> for SpellError {
    fn from(err: ParseIntError) -> Self {
        SpellError::BadNumber(err)
    }
}

fn parse_mana(input: &str) -> Result<u32, SpellError> {
    Ok(input.trim().parse()?)   // ParseIntError → SpellError::BadNumber, automatically
}
```

Errors you create yourself are just values: `return Err(SpellError::UnknownSpell(name.to_string()));`

<div class="compare">

🦀 In real projects the [`thiserror`](https://docs.rs/thiserror) crate derives `Display`, `Error` and `From` from attributes. Writing them by hand once shows you there's no magic involved.

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`Error` requires `Display` and `Debug`.** Forget either and `impl Error` fails to compile.
- **`?` needs a `From` impl** for every foreign error it meets. "the trait `From<ParseIntError>` is not implemented for `SpellError`" means: write it.
- **Don't over-engineer.** One error enum per module or crate is plenty; for a tiny script, `Box<dyn Error>` is fine.
- **`#[derive(PartialEq)]` only works** if every wrapped error is `PartialEq` too (`ParseIntError` is; `std::io::Error` isn't).

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/04_forge_of_resilience/06_custom_errors`

**This file does not compile yet.** Turn `SpellError` into a real error type (`Display`, `Error` with `source`, `From<ParseIntError>`), then write `parse_mana` and `cast` using `?`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Error enum: `#[derive(Debug, PartialEq)] enum MyError { A(String), B { x: u32 }, C(OtherError) }`
- `impl fmt::Display for MyError { fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, ...) } }`
- `impl std::error::Error for MyError {}` (+ optional `fn source(&self)`)
- `impl From<OtherError> for MyError { fn from(e: OtherError) -> Self { MyError::C(e) } }`
- `?` calls `From::from` on the error
- `matches!(value, Pattern)` → `bool`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — Display and From</summary>

`fmt` is a `match self { ... }` with one `write!(f, "...")` per variant, using the exact messages from the doc comment. `from` is just `SpellError::BadNumber(err)`.

</details>
<details class="hint"><summary>Hint 2 — the Error impl</summary>

Below the `Display` impl, write `impl std::error::Error for SpellError { ... }` containing the `source` method shown above. Without `source` the empty `{}` version compiles too — but one test checks the chain.

</details>
<details class="hint"><summary>Hint 3 — cast</summary>

`let mana = parse_mana(mana_input)?;` first. Then `let Some(cost) = spell_cost(spell) else { return Err(SpellError::UnknownSpell(spell.to_string())); };`. Then an `if mana < cost { return Err(SpellError::NotEnoughMana { ... }); }` and finally `Ok(mana - cost)`.

</details>
