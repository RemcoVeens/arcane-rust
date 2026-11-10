# 🏗️ Project 7 — The Grand Battle

> *"One mage against one goblin is a duel. One mage against a horde, every claw and every spell landing in the same heartbeat — that is a battle. And battles need a chronicler."*

## 🎯 The Goal

Turn the turn-based duel into **simultaneous combat**. Every round, Tiberius and every living creature act *at the same time*, each on its own thread. They all touch one shared battlefield, and every event they cause flows through a channel to a chronicler thread that writes the battle report.

```text
              ┌───────────────────────────┐
              │ Battlefield               │
              │ Arc<Mutex<BattleState>>   │
              └──────▲─────────▲──────▲───┘
         strike/cast │         │      │
       ┌─────────────┴┐ ┌──────┴────┐ ┌┴──────────┐
       │ mage thread  │ │ Goblin    │ │ Wraith    │   one thread per combatant,
       │ mage_turn    │ │ creature_ │ │ creature_ │   joined at the end of the round
       └──────┬───────┘ └────┬──────┘ └────┬──────┘
              │   BattleEvent (mpsc)       │
              └──────────────┬─────────────┘
                             ▼
                ┌──────────────────────────┐
                │ chronicler thread        │ → BattleReport
                └──────────────────────────┘
```

## 🧰 The Scaffold

The game you built in earlier chapters is already there, organised in modules — **you don't need to touch them**:

| File | Contents |
|---|---|
| `spells.rs` | `Element`, `Spell`, `grimoire()`, `find_spell()` |
| `creature.rs` | `Creature` with `goblin()`, `wraith()`, `ice_golem()`, `dragon()`, `take_damage`, `damage_from` (×2 on weakness) |
| `mage.rs` | `Mage::tiberius()` (level 5, 100 hp, 100 mana), `Mage::cast` (level & mana checks) |
| `error.rs` | `BattleError` with `Display` + `Error` |
| `battle/events.rs` | `BattleEvent` (with `Display`) and `Outcome` |

You fill in the 🔨 files in `battle/`: `field.rs`, `report.rs`, `round.rs`.

### Determinism in a concurrent world

Thread order is random, so the rules are designed so that **the result doesn't depend on it**:

- Who acts in a round is decided from a **snapshot at the start of the round**. A goblin killed this round still lands its blow — the blows are simultaneous.
- Only the mage damages creatures and only creatures damage the mage, so damage just adds up (saturating at 0) in any order.
- Events carry the *nominal* damage (the creature's attack value), not "how much hp was left".
- `strike_*` returns `true` only for the blow that took a target from alive to 0, so exactly one `Defeated` event is sent per fallen combatant — whichever thread happens to land it.

The tests compare **sorted** lines, totals and outcomes — and run the same battle 20 times to make sure.

## 🪜 Phase 1 — The Battlefield (`battle/field.rs`)

`Battlefield` wraps `Arc<Mutex<BattleState>>` and derives `Clone`: each clone is a handle to the *same* state. Implement `new`, `snapshot` (a cloned copy), `strike_creature`, `strike_mage`, `cast` and `outcome`. Each method locks once, does its work, and lets the guard drop.

<div class="pitfall">

`strike_mage` must check "was alive?" and apply the damage **under the same lock** — otherwise two threads could both report the killing blow (the test throws 20 threads at it).

</div>

## 🪜 Phase 2 — The Chronicler (`battle/report.rs`)

`BattleReport::record` turns one event into report data: push `event.to_string()` onto `lines`, add `Attack`/`SpellCast` damage to `damage_dealt` (keyed by attacker/caster), push `Defeated` names, count `Fizzle`s. `damage_by` reads a total.

`spawn_chronicler(rx)` spawns a thread that records every event until the channel closes, then returns the report through its `JoinHandle`.

## 🪜 Phase 3 — The Clash (`battle/round.rs`)

- `creature_turn`: strike the mage, send `Attack` (and `Defeated` if that blow felled him).
- `mage_turn`: pick the first living creature, find and cast the spell, strike, send `SpellCast` (+ `Defeated`). Every failure — no target, unknown spell, level, mana — becomes a `Fizzle` with the error's `Display` text as the reason.
- `play_round`: snapshot, spawn one thread per living combatant (each gets its own `field.clone()` and `tx.clone()`), join them all.

## 🪜 Phase 4 — The Grand Battle (`run_battle`)

Wire it together: battlefield, channel, chronicler; one round per spell in the plan, stopping as soon as `outcome()` is `Some`; then `drop(tx)`, join the chronicler, and set `rounds` and `outcome` (`Outcome::Retreat` if the plan ran out first).

Then play it:

```bash
cargo run -p ch07_07_project_grand_battle
```

<div class="quest">

🏗️ **Project:** `exercises/07_conclave_of_threads/07_project_grand_battle`

Make every test in `tests/grand_battle.rs` pass, phase by phase.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Shared state: `#[derive(Clone)] struct Handle { state: Arc<Mutex<State>> }`
- `match state.creatures.get_mut(i) { Some(c) if c.is_alive() => …, _ => false }`
- Report: one consumer thread, `for event in rx { … }`, return the report
- Fan out: clone `field` and `tx` per thread, `move` them in, join all handles
- Finish: `drop(tx)` before joining the chronicler, or it waits forever

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — strike_creature / strike_mage</summary>

`let mut state = self.state.lock().unwrap();` then for the mage: `state.mage.is_alive() && state.mage.take_damage(damage) == 0`. `&&` short-circuits, so a fallen mage takes no damage and reports nothing.

</details>
<details class="hint"><summary>Hint 2 — mage_turn</summary>

`state.creatures.iter().enumerate().find(|(_, c)| c.is_alive())` gives you `Option<(usize, &Creature)>`. `let … else { … return; }` keeps the happy path flat. A small closure `let fizzle = |err: BattleError| tx.send(BattleEvent::Fizzle { … }).unwrap();` saves repetition. Use the `Spell` returned by `find_spell` for the proper name (`"arcane blast"` → `"Arcane Blast"`).

</details>
<details class="hint"><summary>Hint 3 — play_round</summary>

Take `let start = field.snapshot();`. If `start.mage.is_alive()`, push `thread::spawn(move || mage_turn(&field, &spell, &tx))` (with clones and an owned `spell: String`) onto a `Vec` of handles. Then for each living creature in `start.creatures`, move the creature itself plus clones of `field`, `tx` and the mage's name into a thread running `creature_turn`. Join every handle.

</details>
