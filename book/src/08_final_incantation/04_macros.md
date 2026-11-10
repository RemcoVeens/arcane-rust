# ⚔️ 8.4 — Macros: The Art of Enchantment

> *"A spell is words that change the world. A macro is words that write spells. The Enchanters of old grew tired of inscribing `Spell { name: …, cost: …, power: … }` a thousand times — so they taught the scroll to write itself."*

## ⚡ The Concept

You've used macros since Quest 1.0: `println!`, `vec!`, `assert_eq!`, `format!`, `todo!`. The `!` marks a macro: it runs **at compile time**, takes *tokens*, and expands into code that is then compiled normally.

### `macro_rules!` — pattern → code

```rust
macro_rules! shout {
    ($spell:expr) => {
        format!("{}!!!", $spell)
    };
}

assert_eq!(shout!("Fireball"), "Fireball!!!");
```

Each rule is `(matcher) => { expansion };`. Rules are tried top to bottom; the first match wins. `$spell:expr` captures an expression into a **metavariable**.

### Fragment specifiers

| Specifier | Matches | Example |
|---|---|---|
| `expr` | an expression | `30`, `base * 2`, `f(x)` |
| `ident` | an identifier | `Fireball`, `x` |
| `literal` | a literal | `"Arcane Blast"`, `42` |
| `ty` | a type | `u32`, `Vec<String>` |
| `tt` | a single token tree | anything |

Literal tokens in the matcher must appear as-is: in `($name:ident => $cost:expr)`, the `=>` is matched literally. That's how you build a mini-language.

`stringify!($name)` turns tokens into a `&'static str`: `stringify!(Fireball)` → `"Fireball"`.

### Repetition

`$( … ),*` repeats a pattern zero or more times, separated by `,` (`+` = one or more, `?` = zero or one). In the expansion, `$( … ),*` repeats once per match:

```rust
macro_rules! total_cost {
    ($($cost:expr),* $(,)?) => {   // `$(,)?` allows a trailing comma
        0 $(+ $cost)*
    };
}

assert_eq!(total_cost!(30, 25, 40), 95);  // expands to 0 + 30 + 25 + 40
assert_eq!(total_cost!(), 0);
```

### Multiple rules and recursion

```rust
macro_rules! count {
    () => { 0usize };
    ($head:tt $($tail:tt)*) => { 1usize + count!($($tail)*) };
}
assert_eq!(count!(a b c), 3);
```

A macro may call itself with fewer tokens — compile-time recursion.

### Hygiene and `$crate`

Variables created inside a macro don't clash with the caller's (hygiene). But *paths* are resolved where the macro is **used**. In an exported macro, write `$crate::Spell` instead of `Spell`, so it works from other crates that never imported `Spell`. `#[macro_export]` makes the macro available at the crate root: `my_crate::spell!`.

<div class="compare">

🐍 Python has no compile-time macros; decorators and metaclasses transform code at *runtime*. · 🐹 Go deliberately has none — `go generate` writes code to files instead. · 🦀 Rust's macros are hygienic, typed by fragment, and expand before type checking. (Procedural macros — `#[derive(Debug)]` and friends — are a separate, more powerful kind that lives in its own crate.)

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Macros match tokens, not types.** `$x:expr` happily accepts `"hello"` where you meant a number; the type error then appears in the *expansion*.
- **Some fragments are restricted in what may follow them**: after `$e:expr` only `=>`, `,` or `;` are allowed. After `$i:ident` anything goes — which is why `Fireball x 3` works.
- **Order matters.** Put the more specific rules first.
- **Use a function if a function will do.** Macros are for variadic arguments, new syntax, or code generation.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/08_final_incantation/04_macros`

Each macro has a placeholder rule that matches anything and panics. Replace them with real rules: `spell!` (one spell), `spellbook!` (repetition, optional trailing `;`), `cast!` (two forms, one with an `x N` multiplier), and a recursive `max_power!`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `macro_rules! m { (pattern) => { expansion }; … }`
- `$x:expr` · `$i:ident` · `$l:literal` · `$t:tt`
- Repeat: `$( … ),*` / `;*` / `+` / `?` — same shape in the expansion
- `stringify!(tokens)` → `&'static str`
- `#[macro_export]` + `$crate::Path` for macros used outside the crate

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — spell!</summary>

Matcher: `($name:ident => $cost:expr, $power:expr)`. Expansion: `$crate::Spell { name: stringify!($name), cost: $cost, power: $power }`.

</details>
<details class="hint"><summary>Hint 2 — spellbook!</summary>

Wrap the `spell!` matcher in a repetition separated by `;` and allow one optional trailing `;`: `($($name:ident => $cost:expr, $power:expr);* $(;)?)`. Expand to `vec![$( … ),*]`, calling `$crate::spell!` inside.

</details>
<details class="hint"><summary>Hint 3 — cast! and max_power!</summary>

`cast!`: rule one `($book:expr, $name:ident)` → `$book.iter().find(|s| s.name == stringify!($name)).map(|s| s.power)`; rule two `($book:expr, $name:ident x $times:expr)` can reuse rule one and `map` the multiplication. `max_power!`: a base rule `($x:expr) => { $x }` and a recursive rule `($x:expr, $($rest:expr),+)` using `std::cmp::max`.

</details>
