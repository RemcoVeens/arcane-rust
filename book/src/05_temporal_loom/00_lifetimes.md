# ⚔️ 5.0 — Lifetimes: The Thread of Existence

> *"Every reference has a lifespan. It is born when the reference is created and dies when it is last used. The Weavers of the Temporal Loom have one law: no thread may outlast the cloth it was spun from."*

## ⚡ The Concept

### The problem lifetimes solve

You met the borrow checker in Chapter 2. Its most important job is preventing **dangling references**:

```rust,compile_fail
fn main() {
    let r;
    {
        let x = 5;
        r = &x;          // ❌ `x` does not live long enough
    }
    println!("{r}");     // x is gone, r would point at garbage
}
```

Inside one function the compiler sees everything. But what about a function that *returns* a reference?

```rust,compile_fail
fn longest(x: &str, y: &str) -> &str {   // ❌ missing lifetime specifier
    if x.len() > y.len() { x } else { y }
}
```

The caller needs to know: *how long may I keep the result?* Does it borrow from `x`, from `y`, or from something else? The signature doesn't say — and the compiler refuses to guess by reading the body. Signatures are contracts.

### Lifetime annotations

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

Read it as: *"there is some region of code `'a` in which both `x` and `y` are valid; the result is valid for that region too."* In practice `'a` becomes the **shorter** of the two input lifetimes.

- `'a` is a **lifetime parameter**, declared in `<...>` like a generic type.
- Annotations **don't change how long anything lives**. They describe relationships so the compiler can check the caller.

```rust
# fn longest<'a>(x: &'a str, y: &'a str) -> &'a str { if x.len() > y.len() { x } else { y } }
let outer = String::from("long incantation");
let result;
{
    let inner = String::from("short");
    result = longest(&outer, &inner);
    println!("{result}");   // ✅ both alive here
}
// println!("{result}");    // ❌ `inner` is dead, so `result` might be too
```

### Only annotate what's related

If the result only ever comes from *one* parameter, only that one shares the lifetime:

```rust
fn first_target<'a>(foe: &'a str, battle_cry: &str) -> &'a str {
    println!("{battle_cry}");
    foe
}
```

Now the caller may drop `battle_cry` immediately and keep the result. Tying both parameters to `'a` would *compile* — but it over-constrains every caller.

### `'static` — lives forever

String literals are baked into the binary, so they have the special lifetime `'static`:

```rust
fn element(rune: u8) -> &'static str {
    if rune == 1 { "fire" } else { "void" }
}
```

### You cannot annotate your way out of a dangling reference

```rust,compile_fail
fn summon() -> &'static str {
    let name = String::from("Tiberius");
    &name   // ❌ returns a reference to data owned by this function
}
```

`name` is dropped when `summon` returns. No annotation changes that. The fix is to **return the owned value**: `-> String`.

<div class="compare">

🐍 Python / 🐹 Go: the garbage collector keeps anything alive as long as something points to it, so returning a pointer to a local is fine. · 🦀 Rust: no GC — the *compiler* proves every reference is valid, and lifetime annotations are how you explain relationships it can't see from a signature alone.

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`'static` is not a magic fix.** When the compiler suggests `&'static str` for a function that builds a `String`, the real fix is usually to return `String`.
- **Don't tie everything to `'a` by reflex.** Only parameters the result can actually borrow from need the same lifetime.
- **Lifetimes are about borrows, not values.** `'a` doesn't keep `x` alive; it only forbids using the result after `x` dies.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/05_temporal_loom/00_lifetimes`

This file **does not compile**. Fix the four signatures:

1. `longest_incantation` — the result may come from either input.
2. `mark_target` — the result comes only from `foe`. One test drops the battle cry early!
3. `summon_champion` — returns a dangling reference. Change the return type.
4. `rune_element` — only ever returns literals.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `fn f<'a>(x: &'a T, y: &'a T) -> &'a T` — result valid while *both* inputs are
- `fn f<'a>(x: &'a T, y: &U) -> &'a T` — result only borrows from `x`
- `&'static str` — string literals; valid for the whole program
- Returning a reference to a local is always an error → return the owned value
- Annotations describe; they never extend a lifetime

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

Read the `help:` lines. For `longest_incantation` the compiler literally prints the signature you need.

</details>
<details class="hint"><summary>Hint 2</summary>

For `mark_target`, declare `<'a>`, put it on `foe` and on the return type — and leave `battle_cry` as plain `&str`. The literal `""` is `'static`, which fits any `'a`.

</details>
<details class="hint"><summary>Hint 3</summary>

`summon_champion() -> String` and return `String::from("Tiberius")` directly. `rune_element(rune: u8) -> &'static str`.

</details>
