# ⚔️ 2.5 — Borrowing Rules: The Compact of the Arcane

> *"A spellbook can be read by many apprentices at once. But when the archmage inscribes a new spell, all reading must cease. To write is to demand solitude. To read is to share."*

## ⚡ The Concept

### The Compact

At any moment, a value can have **either**

- any number of **shared** borrows `&T`, **or**
- exactly **one** **exclusive** borrow `&mut T`,

and every reference must always point at a **live** value.

That's it. These two rules are what lets Rust promise "no data races, no dangling pointers, no iterator invalidation" with zero runtime cost.

### Why readers and writers can't mix

```rust,compile_fail
let mut book = vec![String::from("Fireball")];
let first = &book[0];               // shared borrow of `book`
book.push(String::from("Meteor"));  // ❌ E0502: needs &mut while `first` is alive
println!("{first}");
```

This isn't pedantry. `push` may need more room, **reallocate** the vector's heap buffer, and free the old one — leaving `first` pointing at freed memory. In C++ that is undefined behaviour; in Go it silently reads stale data; in Rust it doesn't compile.

### Borrows end at their last use

A borrow doesn't last until the end of the block — only until the **last time it is used** (the compiler calls this *non-lexical lifetimes*):

```rust
let mut mana = 100;
let glance = &mana;
println!("{glance}");   // last use of `glance` — the borrow ends here
mana -= 30;             // ✅ no conflict anymore
```

So most fixes are about **order**: finish reading before you write, or take what you need (a copy, a length) out of the borrow before it ends:

```rust
let mut grimoire = String::from("Fireball");
let before = grimoire.len();        // a usize — no borrow kept alive
grimoire.push_str(", Frostbolt");
```

### One writer at a time

```rust,compile_fail
let mut mana = 100;
let a = &mut mana;
let b = &mut mana;   // ❌ E0499: second mutable borrow while `a` is still used
*a -= 30;
*b -= 25;
```

Two `&mut` to the same thing would be two writers racing. Use them one after the other instead: `a`'s last use ends its borrow, then `b` may begin.

### No dangling references

```rust,compile_fail
fn conjure_name() -> &String {     // ❌ E0106: missing lifetime specifier
    let name = String::from("Tiberius");
    &name                          // `name` is dropped at the `}` …
}                                  // … so the reference would point at nothing
```

The error message talks about *lifetimes* (a later chapter). The real fix here is simpler: don't lend — **give**. Return the owned `String`.

<div class="compare">

🐍 **Python:** modifying a list while iterating it → silently skipped elements, or a `RuntimeError` for dicts — at runtime.

🐹 **Go:** concurrent map writes → `fatal error: concurrent map writes` at runtime; slices alias after `append` sometimes, sometimes not.

🦀 **Rust:** the same class of bugs is a **compile-time** error. It's the same rule for one thread or a hundred.

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **The owner counts as a user too.** While a `&mut` borrow is alive you can't even *read* the original variable.
- **Method calls borrow.** `book.push(x)` takes `&mut book`; `book.len()` takes `&book`. Errors often point at an innocent-looking method call.
- **Reading the error the right way round.** E0502 shows three spans: where the first borrow starts, where the conflicting borrow is, and the *later use* that keeps the first one alive. Move or shorten that later use.
- **Cloning to escape.** Again: it compiles. It's rarely the right fix here.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/02_ownership_chronicles/05_borrowing_rules`

❌ The file **does not compile**. Each function breaks the Compact once — a reader alive during a write, two writers, a glance kept too long, and a dangling reference. Fix them without `.clone()`.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Many `&T` **or** one `&mut T` — never both at the same time
- A borrow lasts until its **last use**, not the end of the block
- E0502: shared + mutable conflict · E0499: two mutable · E0106/E0515: dangling
- Fix by reordering, by copying out what you need (`let n = s.len();`), or by returning an owned value
- Can't return a reference to a local — return the value itself

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — grimoire_growth & glance_then_cast</summary>

You don't need to *keep* a reference — you need a number. `let before = grimoire.len();` and `let glance = mana;` (a `u32` copy) end the borrow immediately.

</details>
<details class="hint"><summary>Hint 2 — double_drain</summary>

Use `first` completely (`*first -= 30;`) *before* creating `second`. Then the two borrows never overlap.

</details>
<details class="hint"><summary>Hint 3 — learn_and_describe & conjure_name</summary>

Push the new spell first, then take `&book[0]` — the expected text needs the *new* length anyway. For `conjure_name`, change `-> &String` to `-> String` and return `name` (or just `String::from("Tiberius")`).

</details>
