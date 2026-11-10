# ⚔️ 7.2 — Arc & Mutex: The Shared Mana Pool

> *"Ten mages, one crystal. If two reach for it at once, both see 30 mana, both cast, and the crystal shatters at −30. The Conclave's first law: one hand on the crystal at a time."*

## ⚡ The Concept

Channels move data between threads. Sometimes, though, many threads really need the **same** data: a counter, a pool, a ledger. That takes two ingredients.

### `Arc<T>` — shared ownership across threads

`Arc` = **A**tomically **R**eference **C**ounted pointer. `Arc::clone(&a)` doesn't copy the data; it bumps a thread-safe counter and gives you another pointer to the same value. The value is freed when the last `Arc` is dropped.

```rust
use std::sync::Arc;
use std::thread;

let spells = Arc::new(vec!["Fireball", "Frostbolt"]);
let for_thread = Arc::clone(&spells);          // cheap: one counter increment
thread::spawn(move || println!("{}", for_thread.len())).join().unwrap();
println!("{}", spells.len());                  // still usable here
```

But `Arc` only gives *shared* (read-only) access. `*arc += 1` doesn't compile: many owners, so nobody may mutate.

### `Mutex<T>` — one at a time

A `Mutex` wraps a value; the only way to reach it is `lock()`, which returns a **guard**. While you hold the guard, every other thread calling `lock()` waits. When the guard is dropped, the lock is released — automatically.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let crystal = Arc::new(Mutex::new(0u32));
let handles: Vec<_> = (0..4)
    .map(|_| {
        let crystal = Arc::clone(&crystal);
        thread::spawn(move || {
            let mut mana = crystal.lock().unwrap(); // MutexGuard<u32>
            *mana += 25;
        }) // guard dropped here → unlocked
    })
    .collect();
for h in handles { h.join().unwrap(); }
assert_eq!(*crystal.lock().unwrap(), 100);
```

`lock()` returns a `Result` because a thread that panics while holding the lock **poisons** the mutex. `.unwrap()` is the usual answer.

<div class="compare">

🐹 Go: `var mu sync.Mutex` sits *next to* the data — nothing stops you touching the data without locking. · 🦀 Rust: the data lives *inside* the `Mutex`; you literally cannot reach it without the guard. · 🐍 Python: `with lock:` — same idea as the guard, but again only by convention.

</div>

### Check-and-act under one lock

```rust,ignore
// ❌ race: two threads can both pass the check before either subtracts
if pool.remaining() >= 30 { pool.subtract(30); }

// ✅ one guard covers both the check and the change
let mut mana = self.mana.lock().unwrap();
if *mana >= 30 { *mana -= 30; }
```

### `RwLock<T>` — many readers *or* one writer

```rust
use std::sync::RwLock;
let ledger = RwLock::new(5);
{
    let a = ledger.read().unwrap();  // many read guards at once is fine
    let b = ledger.read().unwrap();
    assert_eq!(*a + *b, 10);
}
*ledger.write().unwrap() += 1;       // exclusive
```

Use it when reads vastly outnumber writes. For small, quick critical sections a plain `Mutex` is often just as fast. For a single number there are also atomics (`AtomicU32::fetch_add`) — no lock at all.

### Wrapping it in a type

The idiomatic move: hide `Arc<Mutex<T>>` inside a struct, `#[derive(Clone)]` it, and give it methods. Callers just clone the handle and call `pool.draw(30)` — no locking ceremony leaks out.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Deadlock:** calling `lock()` twice on the same mutex in one thread blocks forever. Rust prevents data races, not deadlocks.
- **Guards live until the end of their scope.** `let g = m.lock().unwrap();` at the top of a long function holds the lock for the whole function. Keep critical sections small, or use a `{ }` block.
- **`Arc::clone(&x)` vs `x.clone()`** — both do the same; the first makes it obvious that only the pointer is cloned.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/07_conclave_of_threads/02_arc_mutex`

Charge a shared crystal from many threads, build a cloneable `ManaPool` whose `draw` can't overdraw even when ten mages rush it, and a `GuildLedger` behind an `RwLock`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `Arc::new(v)` · `Arc::clone(&a)` → another owner, same value
- `Mutex::new(v)` · `*m.lock().unwrap() += 1`
- `RwLock`: `.read().unwrap()` (shared) · `.write().unwrap()` (exclusive)
- Guard dropped = lock released
- Share across threads: `Arc<Mutex<T>>` / `Arc<RwLock<T>>`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — channel_mana</summary>

Same shape as the example above: one `Arc<Mutex<u32>>`, a clone per thread, an inner `for _ in 0..per_mage { *crystal.lock().unwrap() += 1; }`. Join everything before reading the final value.

</details>
<details class="hint"><summary>Hint 2 — ManaPool::draw</summary>

`let mut mana = self.mana.lock().unwrap();` — now `*mana` is the number. Compare, subtract, return `true`/`false` while holding that one guard.

</details>
<details class="hint"><summary>Hint 3 — the ledger</summary>

`*self.gold.write().unwrap().entry(member.to_string()).or_insert(0) += gold;` for deposits; for balances, `self.gold.read().unwrap().get(member).copied().unwrap_or(0)`.

</details>
