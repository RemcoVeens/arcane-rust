# ⚔️ 7.1 — Channels: The Whispering Stones

> *"Scouts carry whispering stones. What one speaks into its stone, the mage at the tower hears — in order, one message at a time. When the last scout breaks their stone, the tower falls silent."*

## ⚡ The Concept

### Do not communicate by sharing memory…

…share memory by communicating. Go made the slogan famous; Rust's standard library ships the same idea as `std::sync::mpsc` — **m**ulti-**p**roducer, **s**ingle-**c**onsumer channels.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();      // tx: Sender<T>, rx: Receiver<T>

thread::spawn(move || {
    tx.send("goblins at the gate".to_string()).unwrap();
});

let msg = rx.recv().unwrap();        // blocks until a message arrives
assert_eq!(msg, "goblins at the gate");
```

`send` **moves** the value into the channel: after sending a `String`, the sending thread can't use it any more. Ownership travels with the message — no other thread can be mutating it.

<div class="compare">

🐹 Go: `ch := make(chan string)`; `ch <- v`; `v := <-ch`; `close(ch)`. · 🦀 Rust: `let (tx, rx) = mpsc::channel()`; `tx.send(v)`; `rx.recv()`; the channel closes when **every `Sender` is dropped** — there is no `close()`. · 🐍 Python: `queue.Queue` is the nearest cousin.

</div>

### Receiving everything

`Receiver` is iterable. The loop ends when the channel is closed *and* empty:

```rust
# use std::sync::mpsc; use std::thread;
let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    for n in 1..=3 {
        tx.send(n).unwrap();
    }
}); // the thread ends → `tx` is dropped → channel closes

let all: Vec<i32> = rx.iter().collect(); // or: for n in rx { ... }
assert_eq!(all, vec![1, 2, 3]);
```

With one producer, messages arrive in the order they were sent.

### Multiple producers

Clone the `Sender` — one clone per thread:

```rust
# use std::sync::mpsc; use std::thread;
let (tx, rx) = mpsc::channel();
for scout in ["Wren", "Bram"] {
    let tx = tx.clone();
    thread::spawn(move || tx.send(scout).unwrap());
}
drop(tx); // ⚠️ drop the original, or `rx.iter()` waits forever

let mut reports: Vec<_> = rx.iter().collect();
reports.sort(); // arrival order is unpredictable — sort to make it stable
```

### Not waiting

| Method | Behavior |
|---|---|
| `recv()` | block until a message, or `Err` when closed |
| `try_recv()` | return immediately: `Ok(msg)`, `Err(Empty)` or `Err(Disconnected)` |
| `recv_timeout(d)` | block for at most `d` |
| `try_iter()` | iterator over messages *already waiting*; never blocks |

There is also `mpsc::sync_channel(n)`: a bounded channel where `send` blocks when `n` messages are waiting (backpressure, like a buffered Go channel).

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **The forgotten Sender.** If any `Sender` is still alive — including the original `tx` you cloned from — `for msg in rx` never ends. The test simply hangs. Drop it with `drop(tx)` (or move it into the last thread).
- **Arrival order across producers is random.** Sort, sum, or count. Don't assert on order.
- **`send` fails** only when the receiver is gone; `.unwrap()` is fine in these quests.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/07_conclave_of_threads/01_channels`

A familiar chants words through a channel, four scouts report through one shared channel, you total the goblins, and you drain an inbox without blocking.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `let (tx, rx) = mpsc::channel::<T>();`
- `tx.clone()` per producer · `tx.send(v)` moves `v` in
- `for msg in rx` / `rx.iter()` — ends when all senders are dropped
- `rx.try_iter()` — only what's waiting, never blocks
- `drop(tx)` — close your copy of the channel

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — chant</summary>

Move `words` and `tx` into one spawned thread that sends `word.to_uppercase()` for each word. When that thread ends, `tx` is dropped, so `rx.into_iter().collect()` on the main thread terminates.

</details>
<details class="hint"><summary>Hint 2 — scout reports</summary>

Loop over the scouts, `let tx = tx.clone();` inside the loop, spawn a thread that sends the tuple. After the loop: `drop(tx)`. Then collect and `sort()` — tuples sort by their first element first.

</details>
<details class="hint"><summary>Hint 3 — drain_inbox</summary>

It's a single expression: `inbox.try_iter().collect()`.

</details>
