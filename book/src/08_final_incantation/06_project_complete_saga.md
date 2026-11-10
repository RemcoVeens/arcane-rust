# 🏗️ Project 8 — The Complete Saga

> *"Your grimoire is complete. Every incantation learned, every spell tested, every creature faced. One quest remains: gather your companions, write your spells in your own tongue, and walk the road from the Goblin Caves to the Dragon's Peak."*

## 🎯 The Goal

Tie the whole course together into one small, playable RPG:

| Chapter | Where it shows up |
|---|---|
| 1–3 basics, structs, enums | `Spell`, `Creature`, `Mage`, `Element`, `Outcome` |
| 4 errors | `BattleError` with `Display` + `Error`, `Result` + `?` in `Saga::encounter` |
| 5 generics & traits | `Bounty` + `total_gold<B: Bounty<Prize = Loot>>` |
| 6 iterators, modules, tests | iterator chains everywhere, one module per concern, `tests/saga.rs` |
| 7 concurrency | `run_battle`: every combatant on its own thread, a chronicler, `Arc<Mutex<…>>` |
| 8 advanced | the `grimoire!` macro, `Rc<RefCell<Mage>>` heroes, associated types |

## 🧰 The Scaffold

The Grand Battle from Chapter 7 is included **fully solved** (`battle/`), with one change: `run_battle` now returns `(BattleState, BattleReport)` so you can see the battle's final state. `error.rs` gained `BattleError::NoHeroes`. You work in four files:

```text
src/
├── spells.rs    🔨 Phase 1  grimoire! macro
├── party.rs     🔨 Phase 2  Party, Hero = Rc<RefCell<Mage>>, Stash
├── rewards.rs   🔨 Phase 3  Bounty for Creature, total_gold, spoils
├── saga.rs      🔨 Phase 4  Saga::encounter
├── battle/      ✅ Chapter 7
└── creature.rs, mage.rs, error.rs  ✅
```

The phases build on each other: Phase 1 first, because every battle looks spells up in the grimoire.

## 🪜 Phase 1 — The Grimoire Macro (`spells.rs`)

`grimoire()` is already written in a spell-table DSL:

```rust,ignore
grimoire! {
    "Fireball"     => Fire,   cost 30, power 45,  level 1;
    "Arcane Blast" => Arcane, cost 40, power 60,  level 5;
    // …
}
```

Replace the placeholder rule of `grimoire!` with one that matches this syntax and expands to a `vec![…]` of `Spell::new(…)` calls. The words `cost`, `power` and `level` are plain tokens in your matcher. The element is an `ident` you splice into a path: `$crate::spells::Element::$element`. One test calls the macro from the *test crate*, so the `$crate::` paths matter.

## 🪜 Phase 2 — The Party (`party.rs`)

A `Hero` is an `Rc<RefCell<Mage>>`: the party holds one handle, the caller (and later the saga) holds others, and everyone sees the same mage. The `Stash` is shared the same way.

- `new(potions)`, `recruit(mage) -> Hero` (store a clone, return the other), `stash()` (clone the `Rc`).
- `leader()`: the first hero still alive (`.find(…).cloned()`).
- `total_hp`, `drink_potion(&hero)` (uses a stash potion, heals up to `max_hp`), `rest()` (everyone to full hp and mana).

<div class="pitfall">

`drink_potion` borrows *two different* `RefCell`s mutably (stash and hero) — that's fine. Borrowing the *same* one twice is what panics.

</div>

## 🪜 Phase 3 — Bounties (`rewards.rs`)

`Bounty` has an associated type `Prize`. `Chest` is done; implement `Bounty for Creature` (gold `max_hp / 2`, trophy `"<name> Trophy"`), then:

- `total_gold<B: Bounty<Prize = Loot>>(&[B])` — works for creatures *and* chests.
- `spoils(&[Creature]) -> (u32, Vec<String>)` — only creatures with 0 hp pay out.

## 🪜 Phase 4 — The Saga (`saga.rs`)

`Saga::encounter` is where single-threaded `Rc` world meets the multi-threaded battle:

1. `let leader = self.party.leader().ok_or(BattleError::NoHeroes)?;`
2. `Rc<RefCell<Mage>>` is not `Send`, so it can't enter the battle threads. **Clone the `Mage` out** of the cell…
3. …`run_battle` with it…
4. …and **write the final mage back**: `*leader.borrow_mut() = state.mage;`
5. On victory, add the spoils of `state.creatures` to the stash.
6. Push the chronicle line, return the outcome.

Then play the full game:

```bash
cargo run -p ch08_06_project_complete_saga
```

<div class="quest">

🏗️ **Project:** `exercises/08_final_incantation/06_project_complete_saga`

Make every test in `tests/saga.rs` pass, phase by phase. Then make it yours: more acts, new creatures, a `Heal` that heals, a combo system from Quest 8.0… The saga is yours to write.

</div>

## 📜 The Grimoire

<div class="grimoire">

- DSL keywords: `$name:literal => $e:ident, cost $c:expr, …` · `$(…);* $(;)?`
- `Rc::new(RefCell::new(mage))` · `Rc::clone(&hero)` · `Rc::ptr_eq(&a, &b)`
- `iter().find(|h| h.borrow().is_alive()).cloned()` → `Option<Rc<…>>`
- `fn f<B: Bounty<Prize = Loot>>(…)` — constrain an associated type
- Crossing a thread boundary: clone out of the `RefCell`, write back after

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — grimoire!</summary>

Matcher: `($($name:literal => $element:ident, cost $cost:expr, power $power:expr, level $level:expr);* $(;)?)`. Expansion: `vec![$( $crate::spells::Spell::new($name, $crate::spells::Element::$element, $cost, $power, $level) ),*]`.

</details>
<details class="hint"><summary>Hint 2 — party</summary>

`Stash { potions, ..Stash::default() }` fills the rest with defaults. In `recruit`: `let hero = Rc::new(RefCell::new(mage)); self.members.push(Rc::clone(&hero)); hero`. In `rest`, loop over `&self.members` and `borrow_mut()` each hero.

</details>
<details class="hint"><summary>Hint 3 — spoils & encounter</summary>

`spoils`: `filter(|c| !c.is_alive())`, `map(Bounty::bounty)`, then a `fold` (or a simple `for` loop) collecting gold and trophies. `encounter`: collect the creature names *before* moving `creatures` into `run_battle`; the chronicle line is `format!("{} vs {}: {:?} in {} rounds", …, names.join(", "), report.outcome, report.rounds)`.

</details>
