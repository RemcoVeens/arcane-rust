# ⚔️ 7.4 — Async Basics: The Patient Incantation

> *"A novice stands at the cauldron, staring, while the potion brews. A master lights the fire, walks away to tend three other cauldrons, and returns exactly when the potion is ready."*

## ⚡ The Concept

Threads are great when there's work to do. But much of a program's life is **waiting**: for a socket, a file, a timer. An OS thread that waits still costs a stack and a scheduler slot. Async Rust lets one thread juggle thousands of waiting tasks.

### `async fn` returns a future

```rust
async fn brew(ingredient: &str) -> String {
    format!("Potion of {ingredient}")
}
```

Calling `brew("moonpetal")` does **not** run the body. It returns a *future*: a value of some type implementing `Future<Output = String>`. The body runs only when the future is `.await`ed (or otherwise polled by a runtime).

```rust,ignore
async fn ritual() -> String {
    let potion = brew("moonpetal").await; // run brew to completion, get its String
    potion.to_uppercase()
}
```

`.await` only works inside `async` code. It means "run this until it can't continue, and let other tasks run while I'm stuck".

<div class="compare">

🐍 Python's `asyncio` is the closest relative: `async def`, `await`, coroutines that do nothing until awaited, and an event loop (`asyncio.run`) to drive them. Rust's `.await` is postfix (`fut.await`, not `await fut`) so it chains: `fetch().await?.parse()`. · 🐹 Go has no async keyword: every goroutine *is* a cheap task, and the runtime hides the waiting from you.

</div>

### Futures are lazy

Python coroutines behave the same way (unlike JavaScript promises, which start running immediately). Go has no equivalent: `go f()` starts `f` right away. In Rust:

```rust,ignore
let fut = brew_logged("moonpetal", &log);  // nothing happens yet
log.push("cauldron lit");                  // this happens FIRST
let potion = fut.await;                    // NOW brew_logged runs
```

A future you never await never runs. The compiler warns: ``unused implementer of `Future` that must be used``.

### `async` blocks

Like closures, but producing a future:

```rust
use std::future::Future;

fn delayed(power: u32) -> impl Future<Output = u32> {
    async move { power + 1 } // `move` captures `power` by value
}
```

This is how you return a future from a non-`async` function. (Clippy will point out that `async fn delayed(power: u32) -> u32` is equivalent — and for a body this small, that's nicer. Both forms are worth recognising.)

### Who runs the futures? A runtime

Rust's standard library defines `Future` but ships **no runtime** to drive them. You pick one; the de-facto standard is **Tokio**:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust,ignore
#[tokio::main]           // builds a runtime and runs `main` on it
async fn main() {
    println!("{}", brew("moonpetal").await);
}

#[tokio::test]           // same, for a test
async fn brewing_works() {
    assert_eq!(brew("x").await, "Potion of x");
}
```

In this workshop, every async quest already has `tokio = { workspace = true }` in its `Cargo.toml`.

### Sequential by default

```rust,ignore
let a = brew("a").await; // finishes first…
let b = brew("b").await; // …then this starts
```

`.await` in a row is **sequential**, just like in Python. Running futures *concurrently* is Quest 7.5 and 7.6.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Forgetting `.await`**: `let p = brew("x");` gives you a future, not a `String`. The type error ("expected `String`, found future") is the clue.
- **`async fn` in a trait** works since Rust 1.75, with some limits; you won't need it here.
- **Never block inside async code** (`std::thread::sleep`, heavy loops): you freeze every task sharing that thread. Quest 7.5 shows the async-friendly alternatives.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/07_conclave_of_threads/04_async_basics`

Write `async fn`s that brew potions and charge a Fireball, return an `async move` block from a normal function, and prove — with a log — that futures are lazy.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `async fn f() -> T` ≈ `fn f() -> impl Future<Output = T>`
- `fut.await` — run to completion (only inside `async`)
- Futures are lazy: nothing runs until awaited
- `async move { ... }` — a future block that owns its captures
- `#[tokio::main]` / `#[tokio::test]` — run async code

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — brew_all</summary>

A plain `for` loop works in async code: create an empty `Vec`, and for each ingredient `push(brew_potion(ingredient).await)`.

</details>
<details class="hint"><summary>Hint 2 — charged_fireball</summary>

Awaits nest: `charge(charge(45).await).await` — then add the bonus.

</details>
<details class="hint"><summary>Hint 3 — lazy_cauldron</summary>

Three statements and an await: `let brew = brew_logged("moonpetal", log);`, push `"cauldron lit"`, then `let potion = brew.await;` and push it. Lock the mutex in each push separately (`log.lock().unwrap().push(...)`), never across the `.await`.

</details>
