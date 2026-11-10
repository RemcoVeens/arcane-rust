# ⚔️ 7.3 — Send & Sync: The Pacts of Passage

> *"Not every artifact may pass through the portal between towers. Some would crumble on the other side. The portal knows — and it refuses them before they ever step through."*

## ⚡ The Concept

How does the compiler *know* that `Arc<Mutex<u32>>` may go to another thread but something else may not? Two marker traits — traits with no methods, just a promise:

| Trait | Meaning | Required by |
|---|---|---|
| `Send` | a value of this type may be **moved** to another thread | `thread::spawn`'s closure (and everything it captures) |
| `Sync` | a `&T` may be **shared** between threads (i.e. `&T` is `Send`) | sharing via `Arc<T>` across threads |

You almost never implement them. They are **auto traits**: a struct is `Send` if all its fields are `Send`, `Sync` if all its fields are `Sync`. Almost everything is both: `u32`, `String`, `Vec<T>`, `HashMap`, `Arc<T>` (when `T: Send + Sync`), `Mutex<T>` (when `T: Send`).

`thread::spawn` is declared roughly as:

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

`Send` — the closure and its captures must be movable to the new thread. `'static` — it must not borrow anything that could die first (hence `move`).

### Meet `Rc` — the single-threaded `Arc`

`std::rc::Rc<T>` is a reference-counted pointer like `Arc`, but its counter is a plain integer, not an atomic one. Cheaper, and **not thread-safe**: two threads cloning it at once could corrupt the count. So `Rc` is neither `Send` nor `Sync`, and the compiler stops it at the portal:

```rust,compile_fail
use std::rc::Rc;
let spells = Rc::new(vec!["Fireball"]);
std::thread::spawn(move || println!("{spells:?}"));
// error[E0277]: `Rc<Vec<&str>>` cannot be sent between threads safely
```

The fix is usually a one-word swap: `Rc` → `Arc`. (Chapter 8 shows where `Rc` shines: single-threaded shared ownership.)

<div class="compare">

🐍 Python & 🐹 Go: nothing stops you from handing a non-thread-safe object to another thread. You find out in production — or with Go's `-race` flag, if the test happens to hit it. · 🦀 Rust: the type system knows, and the program doesn't compile.

</div>

### Reading the error

```text
error[E0277]: `Rc<str>` cannot be sent between threads safely
   |
   = help: within `Familiar`, the trait `Send` is not implemented for `Rc<str>`
note: required because it appears within the type `Familiar`
```

Read it bottom-up: `Familiar` isn't `Send` *because* one of its fields (`Rc<str>`) isn't. Fix the field, and the struct becomes `Send` automatically.

### The other portal errors

- **E0373 "closure may outlive the current function, but it borrows `x`"** → add `move`.
- **E0594 "cannot assign to data in an `Arc`"** → `Arc` alone is read-only; put a `Mutex` inside: `Arc<Mutex<T>>`.

### Checking a type at compile time

```rust
fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

assert_send::<String>();
assert_sync::<std::sync::Mutex<u32>>();
// assert_send::<std::rc::Rc<u32>>(); // ❌ would not compile
```

The function body is empty; the *bound* is the test. Your quest's tests use exactly this trick.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`unsafe impl Send for MyType {}`** exists — and is almost never what you want. It silences the compiler by *lying* to it. Fix the field instead.
- **`Cell`/`RefCell`** (Chapter 8) are `Send` but **not** `Sync`: they can move to another thread, but can't be shared between threads.
- **`MutexGuard` is not `Send`**: you can't lock on one thread and unlock on another.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/07_conclave_of_threads/03_send_sync`

This file **does not compile**. Four rituals break the pacts: an `Rc<str>` field, an `Rc<Vec<…>>` shared list, a mutation through a bare `Arc`, and a thread borrowing a local. Make the smallest fix for each.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `Send`: may move to another thread · `Sync`: `&T` may be shared
- Auto traits: a struct is `Send`/`Sync` iff all fields are
- `Rc` ❌ threads → use `Arc`
- `Arc<T>` = shared, read-only · `Arc<Mutex<T>>` = shared + mutable
- "may outlive" → `move ||`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — Familiar & affordable_counts</summary>

Both errors say `Rc<…> cannot be sent between threads safely`. `Arc<str>` works just like `Rc<str>` (and `name.into()` still builds one from a `&str`). Don't forget to adjust the `use` lines.

</details>
<details class="hint"><summary>Hint 2 — pooled_offering</summary>

`Arc::new(Mutex::new(0u32))`, then `*total.lock().unwrap() += amount;` in the thread and `*total.lock().unwrap()` at the end.

</details>
<details class="hint"><summary>Hint 3 — herald</summary>

The closure borrows `title`, but the thread could outlive the function. One keyword fixes it.

</details>
