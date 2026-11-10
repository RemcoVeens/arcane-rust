# ⚔️ 4.4 — Result: The Fork in the Road

> *"A spell may succeed or fizzle. A potion may brew or explode. These are not catastrophes — they are outcomes. A wise mage plans for both roads."*

## ⚡ The Concept

`Result` is just an enum from the standard library — you could have written it in Chapter 3:

```rust
enum Result<T, E> {
    Ok(T),    // success, carrying a value of type T
    Err(E),   // failure, carrying an error of type E
}
```

A function that can fail says so **in its signature**:

```rust
fn cast(mana: u32, cost: u32) -> Result<u32, String> {
    if mana >= cost {
        Ok(mana - cost)
    } else {
        Err(format!("Not enough mana: need {cost}, have {mana}"))
    }
}
```

<div class="compare">

🐹 Go: `func cast(...) (uint32, error)` and `if err != nil`. Rust's `Result` is the same idea, but you **cannot** use the value without dealing with the error — there is no "forgot to check `err`". · 🐍 Python: exceptions are invisible in the signature; any line might throw. In Rust, failure is part of the type.

</div>

### Handling a Result

It's an enum, so everything from Chapter 3 works:

```rust,ignore
match cast(100, 30) {
    Ok(left) => println!("Cast! {left} mana left"),
    Err(e) => println!("Fizzle: {e}"),
}

if let Err(e) = cast(10, 30) {
    println!("{e}");
}

let Ok(left) = cast(100, 30) else {
    return; // or handle the failure some other way
};
```

### Handy methods

```rust,ignore
r.is_ok();  r.is_err();
r.unwrap_or(0);          // the Ok value, or a default
r.unwrap();              // the Ok value, or PANIC (tests / prototypes only)
r.expect("reason");      // same, with a message
r.ok();                  // Result<T, E> → Option<T> (throws the error away)
```

### Results from the standard library

`str::parse` returns a `Result`, because text might not be a number:

```rust
let ok: Result<u32, std::num::ParseIntError> = "42".parse();      // Ok(42)
let bad: Result<u32, _> = "lots".parse::<u32>();                    // Err(ParseIntError)
let negative = "-5".parse::<u32>();                                 // Err too: u32 has no minus
```

The `::<u32>` ("turbofish") tells `parse` what to parse into when the compiler can't infer it.

To turn the library's error into **your** message, match on it:

```rust,ignore
match input.parse::<u32>() {
    Ok(n) => Ok(n),
    Err(_) => Err(format!("'{input}' is not a mana amount")),
}
```

### Early return on the first failure

Looping over steps that might fail, you want to stop at the first error:

```rust,ignore
for &cost in costs {
    match cast(mana, cost) {
        Ok(left) => mana = left,
        Err(e) => return Err(e),   // bail out, passing the error up
    }
}
Ok(mana)
```

That `match`-and-return dance is so common that Rust has a one-character shortcut for it. That's the next quest — write it out by hand once, so you know what the shortcut does.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Results are `#[must_use]`.** Calling `cast(10, 30);` and ignoring the value gives a warning. Don't silence it with `let _ =` unless you truly don't care.
- **`unwrap()` everywhere** throws away exactly the safety `Result` gives you. In this quest: no `unwrap`.
- **`String` errors are fine for now**, but callers can only *print* them, not match on them. Quest 4.6 fixes that.
- **`Ok` needs wrapping:** a function returning `Result<u32, String>` must end with `Ok(value)`, not `value`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/04_forge_of_resilience/04_result`

Cast spells, parse mana from player input, brew potions and cast whole spell chains — returning `Result<_, String>` everywhere, without a single `unwrap`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `Result<T, E>` = `Ok(T)` | `Err(E)`
- Handle: `match`, `if let Ok(x) = …`, `let Ok(x) = … else { … };`
- `is_ok`, `is_err`, `unwrap_or(default)`, `ok()`, `expect("why")`
- `"42".parse::<u32>()` → `Result<u32, ParseIntError>`
- Early exit: `Err(e) => return Err(e),`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — cast and potions</summary>

`cast` is an `if`/`else` returning `Ok(...)` or `Err(format!(...))`. `potion_strength` is a `match name { "healing" => Ok(30), ... _ => Err(...) }`.

</details>
<details class="hint"><summary>Hint 2 — parse_mana</summary>

Shadow the input first: `let input = input.trim();` — then the error message uses the trimmed version automatically. Then `match input.parse::<u32>() { Ok(n) => Ok(n), Err(_) => Err(format!(...)) }`.

</details>
<details class="hint"><summary>Hint 3 — cast_chain & drink_or_nothing</summary>

`let mut mana = mana;` then loop `for &cost in costs` with the match-and-return pattern shown above; end with `Ok(mana)`. `drink_or_nothing` is `mana + potion_strength(potion).unwrap_or(0)`.

</details>
