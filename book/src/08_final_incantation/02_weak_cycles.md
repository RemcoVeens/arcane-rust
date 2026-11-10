# ⚔️ 8.2 — Weak: The Ouroboros of Memory

> *"The Grand Order held the Mages' guild charter. The Mages held the Grand Order's. When the Order was disbanded, neither charter could be burned — each was still held by the other. The two guilds haunt the archives to this day."*

## ⚡ The Concept

### The cycle problem

`Rc` frees a value when its strong count reaches zero. If two values hold `Rc`s to *each other*, neither count can ever reach zero:

```rust,ignore
// parent ──Rc──▶ child
//   ▲              │
//   └─────Rc───────┘     both counts stay ≥ 1 forever → memory leak
```

Rust's safety guarantees cover use-after-free and data races — **not leaks**. A leak is "safe" (it never touches freed memory); it's just wasteful. Python's garbage collector would find and collect this cycle. Rust has no GC, so you design cycles out.

### `Weak<T>` — a pointer that doesn't own

`Rc::downgrade(&rc)` creates a `Weak<T>`: it points at the value but **doesn't keep it alive**. It counts towards `weak_count`, not `strong_count`.

```rust
use std::rc::{Rc, Weak};

let order = Rc::new("Grand Order");
let memory: Weak<&str> = Rc::downgrade(&order);

assert_eq!(memory.upgrade().as_deref(), Some(&"Grand Order")); // still alive
drop(order);
assert!(memory.upgrade().is_none());                        // gone — and we know it
```

`upgrade()` returns `Option<Rc<T>>`: `Some` (a temporary new owner) if the value is still alive, `None` if it has been freed. No dangling pointers, ever.

### The rule of thumb for trees

**Owners point down, observers point up.**

```rust,ignore
struct Guild {
    name: String,
    parent: RefCell<Weak<Guild>>,       // up: Weak — children don't own parents
    children: RefCell<Vec<Rc<Guild>>>,  // down: Rc — parents own children
}
```

Why the `RefCell`s? Guilds are shared via `Rc<Guild>`, which only gives `&Guild`. To set a parent or push a child *after* creation, we need interior mutability (Quest 8.1).

Linking a child:

```rust,ignore
*child.parent.borrow_mut() = Rc::downgrade(parent); // child → parent: Weak
parent.children.borrow_mut().push(child);           // parent → child: Rc
```

`Weak::new()` creates an empty `Weak` (upgrade always `None`) — the "no parent yet" value.

### Walking up the tree

```rust,ignore
let mut current = self.parent();              // Option<Rc<Guild>>
while let Some(guild) = current {
    // … use guild …
    current = guild.parent();
}
```

<div class="compare">

🐍 Python has the same tool: `weakref.ref(obj)` — and you call it to get the object or `None`. Most Python code never needs it thanks to the cycle collector. · 🐹 Go's GC handles cycles; `weak.Pointer` arrived only in Go 1.24. · 🦀 In Rust, `Weak` is the standard way to model back-references.

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **`upgrade()` can fail.** Always handle the `None`; it's the whole point.
- **Don't hold the upgraded `Rc` longer than needed** — while you hold it, you're an owner and keep the value alive.
- **Debug-printing a tree with `Rc` both ways** recurses forever. With `Weak` upwards, `{:?}` prints `(Weak)` for the parent and stops.
- **Doubly-linked lists and graphs** are famously awkward in safe Rust. Often an arena (`Vec<Node>` + indices) is simpler than `Rc`/`Weak`.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/08_final_incantation/02_weak_cycles`

Build the guild tree: `new`, `add_child` (child remembers its parent with `Weak`), `parent()` with `upgrade`, `child_names`, `lineage` up to the root and the recursive `size`. The tests check the reference counts to prove there is no cycle.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `Rc::downgrade(&rc)` → `Weak<T>` · `Weak::new()` → empty
- `weak.upgrade()` → `Option<Rc<T>>`
- `Rc::strong_count` (owners) · `Rc::weak_count` (observers)
- Trees: `Rc` down to children, `Weak` up to the parent
- Fields mutated after sharing → `RefCell<…>`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — new & add_child</summary>

`Rc::new(Guild { name: …, parent: RefCell::new(Weak::new()), children: RefCell::new(Vec::new()) })`. `add_child` is the two-liner shown above.

</details>
<details class="hint"><summary>Hint 2 — parent & child_names</summary>

`self.parent.borrow().upgrade()`. For names: `self.children.borrow().iter().map(|c| c.name.clone()).collect()`.

</details>
<details class="hint"><summary>Hint 3 — lineage & size</summary>

`lineage`: start a `Vec` with your own name, then the `while let` loop above pushing each ancestor's name. `size`: `1 + ` the sum of `c.size()` over the children (`sum::<usize>()`).

</details>
