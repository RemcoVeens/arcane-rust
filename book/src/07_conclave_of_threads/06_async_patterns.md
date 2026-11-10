# ⚔️ 7.6 — Async Patterns: The Weaving of Many Spells

> *"An archmage never casts one spell and waits. She weaves three at once, abandons the one that is too slow, and drops everything the instant a scout cries 'Ambush!'"*

## ⚡ The Concept

Spawning tasks (Quest 7.5) is one way to get concurrency. Often you don't need a separate task at all: you just want to **await several futures at once** inside the current one.

### `join!` — wait for all

```rust,ignore
let (scale, feather, coins) = tokio::join!(
    fetch("dragon scale", 300),
    fetch("phoenix feather", 200),
    count_coins(100),
);
// ≈ 300 ms total, not 600
```

`join!` polls all futures concurrently **on the current task** and returns a tuple of their outputs once all are done. No `'static`, no `Send`, no `JoinHandle` — the futures may borrow local data.

`try_join!` is the `Result` version: it returns early with the first `Err`.

<div class="compare">

🐍 `await asyncio.gather(a(), b(), c())` · 🐹 a `sync.WaitGroup` plus result variables · 🦀 `tokio::join!(a(), b(), c())`

</div>

### `select!` — wait for the first

```rust,ignore
let winner = tokio::select! {
    _ = sleep(Duration::from_millis(300)) => "Fireball",
    _ = sleep(Duration::from_millis(500)) => "Frostbolt",
};
```

Each branch is `pattern = future => expression`. The first future to complete wins; its branch's expression is the value of the whole `select!`. **The other futures are dropped** — that's how cancellation works in async Rust: a future that is no longer polled simply stops.

If a branch's pattern doesn't match the output (e.g. `Some(msg) = rx.recv()` when `recv()` returns `None`), that branch is disabled and `select!` keeps waiting on the others.

<div class="compare">

🐹 Go's `select { case <-time.After(d): ... case msg := <-ch: ... }` is the direct ancestor. · 🐍 `asyncio.wait(..., return_when=FIRST_COMPLETED)` — but you must cancel the losers yourself.

</div>

### `timeout` — give up after a while

```rust,ignore
use tokio::time::{timeout, Duration};

match timeout(Duration::from_secs(1), slow_cast("Meteor", 5_000)).await {
    Ok(result) => println!("landed: {result}"),
    Err(_elapsed) => println!("fizzled"),
}
```

`timeout(d, fut)` wraps a future; it returns `Ok(output)` or `Err(Elapsed)`. Combine with `map_err` to turn it into your own error type.

### Async channels: `tokio::sync::mpsc`

Tokio has its own channels whose `send`/`recv` are `async` (they wait without blocking the thread). They are bounded:

```rust,ignore
let (tx, mut rx) = tokio::sync::mpsc::channel::<()>(8);
tx.send(()).await.unwrap();
let msg: Option<()> = rx.recv().await; // None once all senders are gone
```

A receiver inside `select!` is the classic "interruptible operation": do the work, unless a message arrives first.

### Structured concurrency, in one sentence

Prefer `join!`/`select!`/`timeout` inside one task over spawning detached tasks: everything a function starts is finished (or cancelled) by the time it returns, which makes the program much easier to reason about.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`join!` is not parallel across CPU cores** — it interleaves on one task. For CPU-heavy work use `tokio::spawn` / `spawn_blocking`.
- **Cancellation can strike at any `.await`.** A future dropped by `select!` stops mid-way; don't leave half-updated state across an await point.
- **A closed channel is not a message.** `rx.recv()` returning `None` is instant. Write the branch as `Some(x) = rx.recv() => …` so a closed channel doesn't fire it.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/07_conclave_of_threads/06_async_patterns`

Gather reagents concurrently with `join!`, cut a slow spell short with `timeout`, settle a duel with `select!`, and channel a spell that an ambush can interrupt — but a closed channel can't.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `tokio::join!(a, b, c)` → `(A, B, C)`, all concurrently
- `tokio::try_join!` → stops at the first `Err`
- `tokio::select! { v = fut => expr, ... }` → first one wins, others dropped
- `tokio::time::timeout(dur, fut).await` → `Result<T, Elapsed>`
- `tokio::sync::mpsc::channel(n)`; `tx.send(v).await`; `rx.recv().await` → `Option<T>`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — gather_reagents</summary>

`tokio::join!` already returns a tuple in exactly the order you list the futures. The whole body can be that one macro call.

</details>
<details class="hint"><summary>Hint 2 — cast_with_timeout</summary>

Build the cast as an `async { sleep(...).await; format!("{spell} lands!") }` block, wrap it in `tokio::time::timeout(...)`, `.await`, and `map_err(|_| CastError::Fizzled)`.

</details>
<details class="hint"><summary>Hint 3 — duel & channel_spell</summary>

Two branches each: `_ = sleep(Duration::from_millis(fire_ms)) => "Fireball",` and so on. For the channel: `_ = sleep(...) => Channeling::Completed,` and `Some(()) = interrupts.recv() => Channeling::Interrupted,`.

</details>
