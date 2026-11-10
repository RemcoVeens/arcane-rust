# ⚔️ 7.0 — Threads: The Summoning of Familiars

> *"A familiar is a thread: a separate spirit that acts on your behalf. With a word you summon it, with a gesture it works, and when it is done it reports back — or dies trying."*

## ⚡ The Concept

### Spawning and joining

```rust
use std::thread;

let handle = thread::spawn(|| {
    // runs on a new OS thread
    6 * 7
});

let answer = handle.join().unwrap(); // wait for it, get its return value
assert_eq!(answer, 42);
```

- `thread::spawn(closure)` starts a real OS thread and returns a `JoinHandle<T>`, where `T` is whatever the closure returns.
- `handle.join()` blocks until the thread finishes and returns `Result<T, _>` — `Err` if the thread panicked.
- If `main` returns, all remaining threads are killed. Join what you care about.

<div class="compare">

🐍 `t = Thread(target=f); t.start(); t.join()` — but no return value, and the GIL means no CPU parallelism. · 🐹 `go f()` — no handle at all; you need a `WaitGroup` or channel to wait. · 🦀 `spawn` gives you a handle that *carries the result back*.

</div>

### `move` closures

A spawned thread may outlive the function that spawned it, so the compiler won't let it *borrow* local variables. Use `move` to hand ownership to the thread:

```rust,compile_fail
let spell = String::from("Fireball");
let h = thread::spawn(|| println!("{spell}")); // ❌ may outlive `spell`
```

```rust
let spell = String::from("Fireball");
let h = thread::spawn(move || println!("{spell}")); // ✅ the thread owns it now
h.join().unwrap();
```

### Many familiars, results in order

Collect the handles first, *then* join them. Joining in the order you spawned gives you results in that order, regardless of who finished first:

```rust
use std::thread;

let handles: Vec<_> = (1..=3)
    .map(|i| thread::spawn(move || i * 100))
    .collect();                       // all three are running now

let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();
assert_eq!(results, vec![100, 200, 300]);
```

⚠️ Writing `.map(|i| thread::spawn(..).join())` in one chain would spawn-and-wait one at a time — no parallelism at all. The `collect()` in between matters.

### Scoped threads: borrowing is back

`thread::scope` guarantees that every thread spawned inside it is joined before the scope ends. Because of that guarantee, those threads may **borrow** local data — no `move`, no cloning:

```rust
use std::thread;

let army = vec![10, 20, 30, 40];
let (left, right) = army.split_at(2);

let total = thread::scope(|s| {
    let a = s.spawn(|| left.iter().sum::<i32>());
    let b = s.spawn(|| right.iter().sum::<i32>());
    a.join().unwrap() + b.join().unwrap()
}); // every scoped thread is guaranteed finished here

assert_eq!(total, 100);
println!("{army:?}"); // still ours
```

### Panics stay inside the thread

A panicking thread doesn't crash the program; its `join()` returns `Err`:

```rust
let h = std::thread::spawn(|| panic!("the familiar explodes"));
assert!(h.join().is_err());
```

`Result::ok()` turns `Ok(v)` into `Some(v)` and `Err(_)` into `None`.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Spawn-and-join in one step is sequential.** Collect the handles first.
- **Threads are OS threads**, each with its own stack (2 MiB by default for spawned threads). Hundreds: fine. Hundreds of thousands: use async (Quest 7.4).
- **`println!` from threads interleaves randomly.** That's why the quests *return* values instead of printing.
- **A panic message still appears** in the test output when a familiar panics on purpose. That's expected.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/07_conclave_of_threads/00_threads`

Implement four rituals: count words in parallel (results in order), gather mana from `n` familiars, sum powers with **scoped** threads, and survive a familiar that panics.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `thread::spawn(move || expr)` → `JoinHandle<T>`
- `handle.join()` → `Result<T, Box<dyn Any + Send>>`; `Err` = the thread panicked
- Collect handles into a `Vec` first, then join → parallel + ordered
- `thread::scope(|s| { s.spawn(|| ...); })` → threads may borrow locals
- `result.ok()` → `Result` to `Option`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — counting words</summary>

`scrolls.into_iter().map(|scroll| thread::spawn(move || ...)).collect::<Vec<_>>()` gives you the handles. Then map each handle through `join().unwrap()`.

</details>
<details class="hint"><summary>Hint 2 — scoped threads</summary>

`let (left, right) = powers.split_at(powers.len() / 2);` then inside `thread::scope(|s| { ... })` spawn two threads with `s.spawn(|| left.iter().sum::<u32>())`. The scope's closure returns the sum of both joins.

</details>
<details class="hint"><summary>Hint 3 — the risky ritual</summary>

`divisor` is a `u32` (which is `Copy`), so a `move` closure copies it in. The whole function is one line: spawn a thread computing `100 / divisor`, `join()` it, and convert the `Result` into an `Option`.

</details>
