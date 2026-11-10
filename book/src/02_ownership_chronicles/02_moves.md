# ⚔️ 2.2 — Moves: The Passing of the Staff

> *"A staff can only be attuned to one mage at a time. To give it to another, you must release your bond. That is the Move."*

## ⚡ The Concept

### Assignment moves

```rust,compile_fail
let staff = String::from("Staff of Embers");
let apprentice = staff;          // ownership MOVES to `apprentice`
println!("{staff}");             // ❌ error[E0382]: borrow of moved value: `staff`
```

Why not just let both point at the same heap text, like Python and Go do? Because then *two* owners would try to free it at the end of the scope — a **double free**. Rust's solution: after `let apprentice = staff;`, the old variable is simply dead. Only the 24-byte handle is copied; the heap text never moves.

```text
 before                          after `let apprentice = staff;`
 staff ──▶ "Staff of Embers"     staff      (moved — unusable)
                                 apprentice ──▶ "Staff of Embers"
```

<div class="compare">

🐍 **Python:** `b = a` creates a second name for the same object. Mutating through `b` changes `a`.

🐹 **Go:** `b := a` copies the slice/map header; both share the data. Aliasing bugs galore.

🦀 **Rust:** `let b = a;` transfers ownership. There is never an alias you didn't ask for.

</div>

### Moves happen everywhere

Not just in `let`. Any time an owned value goes somewhere, it moves:

```rust,compile_fail
fn learn(spell: String) { /* ... */ }

let spell = String::from("Fireball");
learn(spell);                 // moved into the parameter
let mut bag = Vec::new();
bag.push(spell);              // ❌ already moved into `learn`
```

- passing to a function: `learn(spell)`
- pushing into a collection: `bag.push(spell)` — the Vec owns it now
- returning from a function
- destructuring: `let name = spell_tuple.0;`

### Fixing a use-after-move

The compiler error always points at three places: where the value was **created**, where it **moved**, and where you tried to **use** it afterwards. Your options, best first:

1. **Use the new owner.** The value isn't gone — it lives under a new name.
2. **Reorder.** Do the reading *before* the move: `let len = potion.len(); bag.push(potion);`
3. **Get it back.** Have the function return the value, as in Quest 2.1.
4. **Borrow instead** (Quest 2.4) — the usual real-world fix.
5. **Clone** (Quest 2.3) — when you genuinely need two independent values.

### Copy types don't move

```rust
let mana = 100;
let backup = mana;     // u32 is `Copy`: the bits are duplicated
println!("{mana} {backup}");   // ✅ both fine
```

Integers, floats, `bool`, `char`, and tuples/arrays made only of those are **`Copy`**: copying them is as cheap as moving, and there's no heap memory to double-free. Next quest is all about this.

### Partial moves

You can move a *single field* out of a tuple. The rest stays usable — but the tuple as a whole does not:

```rust,compile_fail
let spell = (String::from("Frostbolt"), 35);
let name = spell.0;           // moves the String out of the tuple
println!("{}", spell.1);      // ✅ the u32 is still there (and Copy)
println!("{}", spell.0);      // ❌ that field has been moved out
```

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Don't reach for `.clone()` to silence the error.** It compiles, but it hides the real question: *who should own this?* Most of the time the answer is a reorder or a borrow.
- **"Moved" is about the variable, not the data.** Nothing is wiped or copied at runtime; the compiler just forbids using the old name.
- **Moves in a loop.** `for _ in 0..3 { learn(spell); }` fails: the first iteration moved it, the second has nothing.
- **Moves inside `if`.** If one branch moves a value, it counts as "maybe moved" afterwards — and Rust refuses "maybe".

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/02_ownership_chronicles/02_moves`

❌ The file **does not compile**: four functions use a value after it moved. Fix each one **without `.clone()`** — use the new owner, reorder, or give ownership back. `mana_twice` already works; make sure you understand why.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `let b = a;` / `f(a)` / `v.push(a)` → `a` is moved, unusable afterwards
- E0382 "use of moved value": look at the *"value moved here"* note
- Copy types (`u32`, `f64`, `bool`, `char`, tuples/arrays of them) are copied, not moved
- Moving out of `t.0` is a partial move: `t.1` still usable, `t` as a whole not
- Fix order: new owner → reorder → return it → borrow → clone

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — store_in_vault & pack_bag</summary>

After `let vault = staff;` the staff lives on as `vault` — use that name in the `format!`. In `pack_bag`, measure the label *before* the potion goes into the bag.

</details>
<details class="hint"><summary>Hint 2 — count_runes</summary>

Change the return type to `(String, usize)`: compute the count into a variable, then return `(scroll, count)`. In `read_after_counting`, destructure: `let (scroll, count) = count_runes(scroll);`.

</details>
<details class="hint"><summary>Hint 3 — describe</summary>

`spell.0` was moved into `name` — so use `name` in the format string. `spell.1` is a `u32`, still perfectly usable: `format!("{name} has power {}", spell.1)`.

</details>
