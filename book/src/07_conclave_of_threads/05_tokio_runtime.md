# ⚔️ 7.5 — The Tokio Runtime: The Engine of Patience

> *"Beneath the Conclave hums an engine of brass and starlight. Hand it a thousand patient incantations and it keeps them all turning, waking each one the moment its moon rises."*

## ⚡ The Concept

### Tasks: `tokio::spawn`

`tokio::spawn` is the async twin of `thread::spawn`. It hands a future to the runtime, which runs it **concurrently** with everything else, and returns a `JoinHandle<T>`:

```rust,ignore
let handle: tokio::task::JoinHandle<u32> = tokio::spawn(async {
    6 * 7
});
let answer = handle.await.unwrap(); // Result<u32, JoinError>
```

- The task starts running right away (spawning is not lazy — the *runtime* polls it).
- Like threads, spawned futures must be `Send + 'static`: use `async move` and owned data (`String`, not `&str`).
- `handle.await` returns `Err(JoinError)` if the task panicked (`err.is_panic()`) or was cancelled (`handle.abort()`).
- Tasks are cheap: spawning 100 000 is fine. Threads: not so much.

<div class="compare">

🐹 `go f()` ≈ `tokio::spawn(f())`, except you get a handle with the result back. · 🐍 `asyncio.create_task(coro)` ≈ `tokio::spawn(fut)`; awaiting the task gives its result.

</div>

### Time: `tokio::time::sleep`

```rust,ignore
use std::time::Duration;
tokio::time::sleep(Duration::from_millis(500)).await; // yields to other tasks
```

**Never** use `std::thread::sleep` in async code: it blocks the whole worker thread, freezing every task scheduled on it. `tokio::time::sleep` just tells the runtime "wake me in 500 ms" and steps aside.

### Concurrency: spawn first, await later

```rust,ignore
// Sequential: ~3 s total
let a = slow_cast("Fireball".into(), 1_000).await;
let b = slow_cast("Meteor".into(), 2_000).await;

// Concurrent: ~2 s total
let ha = tokio::spawn(slow_cast("Fireball".into(), 1_000));
let hb = tokio::spawn(slow_cast("Meteor".into(), 2_000));
let (a, b) = (ha.await.unwrap(), hb.await.unwrap());
```

Same rule as threads: start everything, *then* wait. Awaiting the handles in input order gives you results in input order.

### Testing time without waiting: paused clocks

Tests that really sleep are slow and flaky. Tokio can freeze its clock:

```rust,ignore
#[tokio::test(start_paused = true)]
async fn meteor_takes_three_seconds() {
    let start = tokio::time::Instant::now();
    slow_cast("Meteor".into(), 3_000).await;
    assert!(start.elapsed() >= Duration::from_secs(3)); // passes instantly!
}
```

With a paused clock, whenever every task is asleep, Tokio **jumps time forward** to the next timer. The test runs in microseconds but `Instant` sees exactly the simulated time. (This needs Tokio's `test-util` feature, enabled in the quest's `[dev-dependencies]`.)

### Blocking work: `spawn_blocking`

Got a heavy computation or a blocking library call inside async code? Ship it to Tokio's blocking thread pool:

```rust,ignore
let sum = tokio::task::spawn_blocking(|| (1..=1_000_000u64).sum::<u64>()).await.unwrap();
```

### `#[tokio::main]` under the hood

```rust,ignore
fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async { /* your async main */ });
}
```

The default runtime is multi-threaded (one worker per CPU core). `#[tokio::test]` uses a single-threaded one.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`&str` into `tokio::spawn`** → "borrowed value does not live long enough": spawned tasks need `'static` data. Pass a `String`.
- **Holding a `std::sync::MutexGuard` across `.await`** makes the future non-`Send`, so `tokio::spawn` rejects it. Drop the guard first (or use `tokio::sync::Mutex`).
- **A dropped `JoinHandle` does not cancel the task** — it keeps running, detached. Use `handle.abort()` to cancel.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/07_conclave_of_threads/05_tokio_runtime`

Summon imp tasks and sum their results, cast slow spells with `tokio::time::sleep`, cast several **concurrently**, start a ritual in the background and return its handle, and catch a task that panics.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `tokio::spawn(fut)` → `JoinHandle<T>`; `handle.await` → `Result<T, JoinError>`
- `tokio::time::sleep(Duration::from_millis(n)).await`
- Spawn all, then await all → concurrent + ordered
- `#[tokio::test(start_paused = true)]` → simulated time
- `tokio::task::spawn_blocking(|| ...)` for blocking work

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — summon_imps</summary>

`(0..n).map(|i| tokio::spawn(async move { i * 2 })).collect::<Vec<_>>()`, then a `for` loop that adds `h.await.unwrap()` to a total. (Iterator adapters like `.map()` can't contain `.await`, so the second step is a loop.)

</details>
<details class="hint"><summary>Hint 2 — cast_concurrently</summary>

`slow_cast(spell, ms)` already *is* a future, so `tokio::spawn(slow_cast(spell, ms))` works directly. Collect the handles, then await them in order into a `Vec`.

</details>
<details class="hint"><summary>Hint 3 — begin_ritual & guarded_ritual</summary>

`begin_ritual` is not `async`: it returns `tokio::spawn(async move { sleep(...).await; power })` straight away. For `guarded_ritual`, spawn `async move { ritual() }`, await the handle, and `map_err` the `JoinError` into the required message.

</details>
