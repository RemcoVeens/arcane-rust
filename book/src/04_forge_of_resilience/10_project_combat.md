# 🏗️ Project 4 — The Combat System

> *"Theory means nothing when a dragon is breathing down your neck. On the road back to the Academy, Tiberius walks into an ambush: a goblin, an ice wraith, and something much, much bigger. Time to make your grimoire fight."*

This project brings the whole chapter together:

| Concept | Where it shows up |
|---|---|
| Traits + default methods | `Creature` with `weakness`, `resistance`, `is_alive` |
| Trait objects | the enemies: `Vec<Box<dyn Creature>>` |
| `Vec` | the `Spellbook` |
| `HashMap` + entry API | the `Inventory` |
| Custom error enum + `Display` | `CastError` |
| `Result` + `?` | `cast`, `drink`, `take_turn` |

## 🧱 The Scaffold

Open `exercises/04_forge_of_resilience/10_project_combat/src/lib.rs`. The top part is the Chapter 3 game, already done:

- `Element` (`Fire`, `Ice`, `Arcane`, `Holy`) and `Spell` (name, cost, power, level, element), plus `starter_spells()` — the five spells of the Academy.
- `Mage` with `heal` and `restore_mana` (both capped at 100).
- `Command` and `parse_command`: `"cast 2 frostbolt"` becomes `Command::Cast { spell: "frostbolt", target: 1 }` (players count from 1, the code from 0).
- `src/main.rs` — the playable game loop. Don't touch it; make it work.

Everything below the `PHASE 1` banner is yours. Work through the phases in order; the tests in `tests/combat.rs` are grouped the same way. Run them all with `wr`, or one phase at a time while you work, e.g. `cargo test -p ch04_10_project_combat strike`.

<div class="pitfall">

**The crate doesn't compile at first**: `Battle::standard()` puts a `Dragon` into a `Vec<Box<dyn Creature>>`, and `Dragon` doesn't implement `Creature` yet. That's your very first task.

</div>

## Phase 1 — Creatures

The `Creature` trait has four required methods and three provided ones. `Goblin` is implemented as an example.

1. Write the default `is_alive` (more than 0 hp).
2. Fill in `impl Creature for IceWraith` — and override `weakness` (Fire) and `resistance` (Ice).
3. Write `impl Creature for Dragon` from scratch: 150 hp, attack 12, weak to Ice, resists Fire.
4. Write `strike(target: &mut dyn Creature, power, element)`: double damage against a weakness, half (rounded down) against a resistance, apply it, return it.

`strike` takes a `&mut dyn Creature`, so it works on a plain `Goblin` *and* on an element of the horde: `strike(horde[1].as_mut(), 45, Element::Fire)`.

## Phase 2 — Spellbook & Inventory

`CastError` is defined at the top of this phase; you'll need some of its variants now.

1. `Spellbook::find` — case-insensitive search through the `Vec<Spell>`. Return `Option<&Spell>`: a reference into the book, no cloning.
2. `Spellbook::learn` — `Err(CastError::AlreadyKnown(..))` if a spell with that name (any case) exists, else push. (You can reuse `find`!)
3. `Inventory::add`, `count`, `take` — the patterns from Quest 4.2, with `take` returning `Err(CastError::OutOfItem(..))` instead of `false`.

## Phase 3 — Casting with `Result`

1. `Display for CastError` — the exact messages are in the doc comment. Note `InvalidTarget` shows `i + 1`: players count from 1.
2. `Battle::cast` — the heart of the game. Check in order: spell known → level → mana. A `Holy` spell heals the mage and ignores the target. Any other spell needs a target that exists **and** is alive. Only spend mana once every check has passed.
3. `Battle::drink` — reject anything that isn't a mana or health potion with `NotDrinkable`, then `self.inventory.take(item)?` — the `?` passes `OutOfItem` straight through.
4. `enemies_turn` — every living enemy hits the mage; `outcome` — `Defeat` beats `Victory`.

<div class="pitfall">

**Borrowing two fields at once.** Inside `cast`, `self.spellbook.find(..)` gives you a `&Spell` that borrows `self.spellbook`. You can still mutate `self.mage` and `self.enemies` while holding it — Rust tracks borrows *per field* inside a method. What you can't do is call another `&mut self` method (like `self.drink(..)`) while the `&Spell` is alive.

</div>

To get a target out of the horde: `let Some(enemy) = self.enemies.get_mut(target) else { ... };` gives you a `&mut Box<dyn Creature>`; pass `enemy.as_mut()` to `strike`.

## Phase 4 — The Full Turn

`take_turn(input)` ties it all together:

1. `parse_command(input)` returns an `Option`; turn it into a `Result` with `.ok_or(CastError::UnknownCommand(...))?`.
2. `Cast` and `Drink` call your methods with `?` — a failed action returns early and the enemies don't move.
3. After a successful action, if `outcome()` is still `None`, the enemies strike back: append a line like `"The enemies strike back for 17 damage!"`.
4. `Status`, `Help` and `Quit` just return text and cost no turn.

Then play it:

```text
cargo run -p ch04_10_project_combat
```

A good opening: `cast 2 fireball` (fire against the wraith's weakness), then deal with the goblin, then freeze the dragon.

## 🎯 Your Quest

<div class="quest">

🏗️ **Project:** `exercises/04_forge_of_resilience/10_project_combat`

Implement the four phases until every test in `tests/combat.rs` passes, then win the battle.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Default trait method: `fn is_alive(&self) -> bool { self.hp() > 0 }`
- Override a default: just write the method in the `impl` block
- `Vec<Box<dyn Creature>>` · `boxed.as_mut()` → `&mut dyn Creature` · `vec.get_mut(i)` → `Option<&mut T>`
- Case-insensitive compare: `a.to_lowercase() == b.to_lowercase()`
- `*map.entry(k.to_string()).or_insert(0) += n;` · `let Some(x) = map.get_mut(k) else { ... };`
- `option.ok_or(err)?` · `self.inventory.take(item)?`
- `x.saturating_sub(y)` · `(a + b).min(MAX)`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — Phase 1 & 2</summary>

`strike` is an `if target.weakness() == Some(element) { power * 2 } else if target.resistance() == Some(element) { power / 2 } else { power }`, then `target.take_damage(damage)`. For `find`, lowercase `name` once, then loop `for spell in &self.spells` and `return Some(spell)` on a match. For `learn`, `if self.find(&spell.name).is_some() { return Err(CastError::AlreadyKnown(spell.name)); }`.

</details>
<details class="hint"><summary>Hint 2 — Battle::cast</summary>

Structure it as a series of guards: `let Some(spell) = self.spellbook.find(spell_name) else { return Err(...) };`, then two `if ... { return Err(...) }` checks. Handle `Element::Holy` next (spend mana, `self.mage.heal(HEAL_AMOUNT)`, return `Ok(..)`). For the rest: `get_mut(target)` with let-else, an `is_alive` check, then spend the mana and `strike(enemy.as_mut(), spell.power, spell.element)`.

</details>
<details class="hint"><summary>Hint 3 — take_turn</summary>

```rust,ignore
let command = parse_command(input).ok_or(CastError::UnknownCommand(input.trim().to_string()))?;
let mut message = match command {
    Command::Cast { spell, target } => self.cast(&spell, target)?,
    Command::Drink(item) => self.drink(&item)?,
    Command::Status => return Ok(self.status()),
    // ... Help and Quit return early too
};
// now: if the battle isn't over, let the enemies strike back and append to `message`
```

</details>
