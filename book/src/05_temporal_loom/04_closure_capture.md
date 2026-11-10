# ⚔️ 5.4 — Capturing: Borrow, Mutate, or Consume

> *"A scroll can be read by many mages. It can be amended by one. Or it can be burned to fuel a single great spell. Closures take hold of the world in exactly these three ways."*

## ⚡ The Concept

### Three ways to capture

The compiler looks at what a closure *does* with each captured variable and picks the gentlest capture that works:

```rust
let scroll = String::from("Ignis");

let read = || println!("{scroll}");      // only reads   → borrows  &scroll
read();

let mut chants = 0;
let mut chant = || chants += 1;          // modifies     → borrows  &mut chants
chant();
chant();

let burn = || drop(scroll);              // gives it away → moves   scroll
burn();
// burn();  ❌ the scroll is already gone
```

### …which decides the closure's trait

| Trait | The closure… | Can be called | Example |
|---|---|---|---|
| `Fn` | only reads captures | many times, even shared | `\|x\| x + bonus` |
| `FnMut` | mutates captures | many times, needs `mut` | `\|\| count += 1` |
| `FnOnce` | moves captures out | **once** | `\|\| scroll` (returns it) |

They form a hierarchy: every `Fn` is also `FnMut`, and every `FnMut` is also `FnOnce`. So when **accepting** a closure, ask for the *least* you need:

```rust
fn twice<F: FnMut()>(mut f: F) {   // FnMut: I call it repeatedly, it may mutate
    f();
    f();
}

fn finally<F: FnOnce() -> String>(f: F) -> String {  // FnOnce: I call it once
    f()
}
```

`FnOnce` is the most permissive bound for callers; `Fn` is the most restrictive. Note the `mut f` — calling an `FnMut` mutates the closure's own state.

### `move` — take ownership of captures

`move` forces the closure to **own** everything it captures, even if it only reads:

```rust
fn echo(name: String) -> impl Fn() -> String {
    move || format!("{name}!")   // without `move`: borrows `name`, which dies here ❌
}
```

`move` decides **how values get in** (by move). `Fn`/`FnMut`/`FnOnce` describe **what the body does** with them. A `move` closure that only reads its captures is still `Fn`.

### Closures with state

A `move` closure that mutates its captured values carries private state between calls — a tiny object without a struct:

```rust
fn counter() -> impl FnMut() -> u32 {
    let mut count = 0;
    move || {
        count += 1;
        count
    }
}

let mut next = counter();
assert_eq!((next(), next(), next()), (1, 2, 3));
```

<div class="compare">

🐍 Python closures capture variables by reference and need `nonlocal` to rebind them. · 🐹 Go closures capture by reference, always. · 🦀 Rust picks borrow, mutable borrow, or move per variable — and encodes the result in the type, so misuse is a compile error, not a race.

</div>

### Storing different closures together: `Box<dyn Fn>`

Every closure has its own type, so `vec![|x| x * 2, |x| x + 1]` doesn't work. Box them as trait objects (Chapter 4!):

```rust
let runes: Vec<Box<dyn Fn(u32) -> u32>> = vec![
    Box::new(|x| x * 2),
    Box::new(|x| x + 1),
];
let mut power = 10;
for rune in &runes {
    power = rune(power);
}
assert_eq!(power, 21);
```

Use generics (`impl Fn`) when there's one closure; `Box<dyn Fn>` when you need a collection or a struct field.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`F: Fn()` rejects mutating closures.** Error: "expected a closure that implements `Fn`, but this closure only implements `FnMut`". Loosen the bound.
- **Forgetting `mut`** on an `FnMut` parameter or variable: `let mut f = counter();`.
- **"closure may outlive the current function"** → add `move`.
- **`move` copies `Copy` types.** `move || x + 1` with `x: u32` copies `x`; the original is still usable.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/05_temporal_loom/04_closure_capture`

The file **does not compile**: `channel`, `unseal` and `spell_echo` promise the wrong thing. The tests are correct — fix the signatures/closure. Then implement `mana_well`, `cooldown`, `carve_runes` and `apply_runes`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Reads → `Fn` · mutates → `FnMut` · moves out → `FnOnce`
- Accept the loosest bound you can: `FnOnce` > `FnMut` > `Fn`
- `fn f<F: FnMut()>(mut f: F)` — note the `mut`
- `move || ...` — the closure owns its captures
- Stateful closure: `let mut n = 0; move || { n += 1; n }`
- Collections of closures: `Vec<Box<dyn Fn(u32) -> u32>>`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1</summary>

`channel`: the test's closure does `chants += 1` → it's `FnMut`. `unseal`: the closure returns the captured `String` by value → `FnOnce`. `spell_echo`: the closure borrows `name`, which dies when the function returns → `move`.

</details>
<details class="hint"><summary>Hint 2</summary>

`mana_well`: create `let mut left = mana;` and return `move |amount| { ... }`. `amount.min(left)` is how much you can draw.

</details>
<details class="hint"><summary>Hint 3</summary>

`cooldown`: keep a `waited` counter; each call increments it, and when `waited >= turns` reset it to 0 and return `true`. `carve_runes`: `vec![Box::new(|p| p * 2), Box::new(move |p| p + bonus), Box::new(|p| p / 2)]`. `apply_runes`: loop and reassign, or `runes.iter().fold(power, |p, rune| rune(p))`.

</details>
