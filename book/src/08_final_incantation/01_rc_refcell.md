# ⚔️ 8.1 — Rc & RefCell: The Shared Satchel

> *"Four adventurers, one satchel. Each carries a strap of it. Anyone may toss loot in or pay the innkeeper — but if two hands rummage at once, the satchel snaps shut on their fingers."*

## ⚡ The Concept

Ownership says: one owner. Real programs sometimes need **several owners of the same value**, and the right to **change it** through any of them. On one thread, that's `Rc<RefCell<T>>`.

### `Rc<T>` — reference counting (single-threaded)

You met `Rc` in Quest 7.3 as "the `Arc` that can't cross threads". `Rc::clone(&rc)` adds an owner; dropping one removes an owner; the value is freed when the count hits 0.

```rust
use std::rc::Rc;

let satchel = Rc::new(vec!["Rope"]);
let wren = Rc::clone(&satchel);            // count: 2
assert_eq!(Rc::strong_count(&satchel), 2);
drop(wren);                                // count: 1
assert_eq!(Rc::strong_count(&satchel), 1);
```

Like `Arc`, `Rc` gives only **shared** access: `&T`. No mutation.

### `RefCell<T>` — borrow rules, checked at runtime

`RefCell` moves the borrow checker's rules (*many `&` or one `&mut`*) from compile time to run time:

```rust
use std::cell::RefCell;

let gold = RefCell::new(100);
{
    let mut g = gold.borrow_mut(); // RefMut<u32> — like `&mut`
    *g -= 30;
}                                  // RefMut dropped → borrow released
assert_eq!(*gold.borrow(), 70);    // Ref<u32> — like `&`
```

Note: `borrow_mut` only needs `&gold`, not `&mut gold`. That's **interior mutability**: mutation through a shared reference, made safe by a runtime check. Break the rule and you get a panic instead of a compile error:

```rust,ignore
let a = gold.borrow_mut();
let b = gold.borrow_mut(); // 💥 panic: already mutably borrowed
```

`try_borrow()` / `try_borrow_mut()` return a `Result` instead of panicking.

### Together: `Rc<RefCell<T>>`

```rust
use std::cell::RefCell;
use std::rc::Rc;

let satchel = Rc::new(RefCell::new(Vec::<String>::new()));
let tiberius = Rc::clone(&satchel);
let morgana = Rc::clone(&satchel);

tiberius.borrow_mut().push("Goblin Ear".into());
morgana.borrow_mut().push("Wraith Essence".into());
assert_eq!(satchel.borrow().len(), 2);   // everyone sees everything
```

`Rc` answers *"who owns it?"* (all of them), `RefCell` answers *"who may change it right now?"* (one at a time, checked live).

<div class="compare">

🐍 This is how Python objects work *all the time*: refcounted, shared, mutable from anywhere — minus the safety check. · 🐹 Go: share a pointer, mutate freely. · 🦀 Rust makes you opt in, and still enforces "one writer at a time".

| | single thread | multi-thread |
|---|---|---|
| shared ownership | `Rc<T>` | `Arc<T>` |
| interior mutability | `RefCell<T>` (`Cell<T>` for `Copy` values) | `Mutex<T>` / `RwLock<T>` |

</div>

### `Cell<T>` in one breath

For small `Copy` values (counters, flags), `Cell` offers `get()`/`set()` with no borrow tracking at all: `let hits = Cell::new(0); hits.set(hits.get() + 1);`.

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Keep borrows short.** `let s = self.satchel.borrow_mut();` at the top of a method, then calling another method that borrows again → runtime panic. Borrow, do the work, let it drop.
- **A `Ref` kept in a variable lives to the end of its scope** — just like a `MutexGuard`.
- **Reach for `Rc<RefCell>` last.** Plain ownership plus `&mut` parameters is simpler and checked at compile time. Use it when ownership genuinely is shared (graphs, observer lists, game entities referenced from several places).
- **`Rc` cycles leak memory** — next quest.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/08_final_incantation/01_rc_refcell`

Build the party's shared satchel: adventurers join with an `Rc` clone, loot, pay and take items through `&self`, count the holders, and peek politely with `try_borrow`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `Rc::new(v)` · `Rc::clone(&rc)` · `Rc::strong_count(&rc)`
- `RefCell::new(v)` · `.borrow()` → `Ref<T>` · `.borrow_mut()` → `RefMut<T>`
- `.try_borrow()` / `.try_borrow_mut()` → `Result`, never panics
- Shared + mutable, one thread: `Rc<RefCell<T>>`
- Shared + mutable, many threads: `Arc<Mutex<T>>`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — creating and joining</summary>

`Rc::new(RefCell::new(Satchel { items: Vec::new(), gold }))`. In `join`, store `Rc::clone(satchel)` — cloning the `Rc`, never the `Satchel`.

</details>
<details class="hint"><summary>Hint 2 — pay and take</summary>

Hold one `let mut satchel = self.satchel.borrow_mut();` for the whole method. For `take`, `satchel.items.iter().position(|i| i == item)?` finds the index (the `?` returns `None` early), then `Some(satchel.items.remove(index))`.

</details>
<details class="hint"><summary>Hint 3 — holders and peek</summary>

`Rc::strong_count(satchel)`. For peeking: `satchel.try_borrow().ok().map(|s| s.items.clone())`.

</details>
