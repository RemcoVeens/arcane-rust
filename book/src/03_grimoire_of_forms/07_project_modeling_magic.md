# 🏗️ Project 3 — Modeling Magic

> *"The grimoire grows. What were raw incantations are now structured spells. What was a flat list is now a tower full of creatures, each with an element, each with a weakness."*

Your Chapter 2 game passed tuples around: `("Fireball", 30, 45, 1)`. Now every concept gets a real type, commands become an enum, and the game loop becomes a `match`. And there's finally something to cast *at*: a tower of creatures with elemental weaknesses.

## 🎯 The Goal

```text
Welcome, Tiberius. Type 'help' for commands.
A Frost Goblin (30 HP, Frost) blocks your path.

[Tiberius | Lv 7 | ✦ 100/100 | XP 0 | 📜 2] > cast fireball
You cast Fireball! 90 damage — the Frost Goblin is defeated! +20 XP

[Tiberius | Lv 7 | ✦ 70/100 | XP 20 | 📜 2] > look
A Fire Imp (60 HP, Fire) blocks your path.

[Tiberius | Lv 7 | ✦ 70/100 | XP 20 | 📜 2] > cast frostbolt
You cast Frostbolt! Fire Imp takes 17 damage.
```

The project lives in `exercises/03_grimoire_of_forms/07_project_modeling_magic`:

- `src/main.rs` — the game loop, **already written**: `Command::parse` each line, `game.handle(command)`, print, repeat while `game.running`.
- `src/lib.rs` — the types. Everything marked **✅ Provided** is the Chapter 2 feature set (rest, potions, scrolls, spell list, `split_command`) ported to methods. Everything with `todo!()` is yours.
- `tests/modeling_magic.rs` — the spec, in three phases.

## 🗺️ The model

```text
Game ─┬─ mage: Mage ─┬─ spellbook: Vec<Spell>   Spell { name, cost, power, level, element }
      │              └─ scrolls:   Vec<Spell>
      ├─ foes: Vec<Creature>                    Creature { name, hp, element, xp }
      └─ running: bool

Element  = Fire | Frost | Arcane | Life
Command  = List | Cast(String) | Rest | Potion | Read | Look | Help | Quit | Unknown(String)
CastOutcome = Hit { spell, damage, defeated } | UnknownSpell
            | LevelTooLow { required } | NotEnoughMana { needed, available }
```

Fire beats Frost, Frost beats Arcane, Arcane beats Fire. Life (Heal) is neutral against everything.

## 🧱 Phase 1 — Forms

Implement `Element::multiplier`, `Spell::describe`, and `Creature::new` / `is_alive` / `take_hit`.

- `multiplier` is a `match` on `(self, defender)` — strong pairs 2.0, reversed pairs 0.5, `_` 1.0. A `use Element::*;` at the top of the method keeps the patterns short.
- `describe` prints the element with `{:?}` — that's what `#[derive(Debug)]` is for.
- `take_hit` converts to `f64` for the multiplier and back with `as u32` (which rounds *down*: 17.5 → 17), then subtracts with `saturating_sub`.

## 🧱 Phase 2 — The mage casts

Implement `Mage::cast(&mut self, name: &str, target: &mut Creature) -> CastOutcome`.

Instead of building message strings deep inside the logic, `cast` returns **what happened** as an enum. The caller decides how to phrase it. That makes the logic easy to test (`assert_eq!(outcome, CastOutcome::LevelTooLow { required: 10 })`) and keeps text out of the model.

Check in order: unknown → level → mana. `let Some(spell) = self.find_spell(name) else { return CastOutcome::UnknownSpell; };` is the perfect opening line. Remember the mana *before* spending it, for `NotEnoughMana { available }`.

<div class="pitfall">

`cast` borrows the mage mutably (`&mut self`) **and** the creature mutably (`target`). That's fine — they are two different values. The interesting part comes in Phase 3, where both live inside the same `Game`.

</div>

## 🧱 Phase 3 — Commands & the game

Implement `Command::parse` and `Game::handle`.

`parse` uses the provided `split_command`, lowercases the command word, and `match`es on `word.as_str()` (you can't match a `String` against string literals directly, but a `&str` you can). A guard handles `cast` without a spell name: `"cast" if !argument.is_empty() => …`.

`handle` is one big `match command { … }` — exhaustive, so if you ever add a `Command` variant, the compiler will point you here. Casting is the tricky arm:

```rust,ignore
let Some(foe) = self.foes.first_mut() else {
    return String::from("There is nothing to fight.");
};
match self.mage.cast(&name, foe) { /* … */ }
```

`self.foes.first_mut()` borrows `self.foes`; `self.mage.cast(…)` borrows `self.mage`. Two **different fields** of `self` — Rust tracks borrows per field, so this compiles. (Calling a `&mut self` *method* of `Game` while `foe` is alive would not: that borrows all of `self`.)

When the foe is defeated, you need to `self.foes.remove(0)` — which needs `foe`'s borrow to be over. It is, as long as you don't use `foe` again in that arm: take the name and xp from the value `remove` hands back.

Consider moving the cast arm into a private helper method (`fn cast_at_foe(&mut self, name: &str) -> String`) — `handle` stays readable, and a method without `pub` is only visible inside this module.

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/03_grimoire_of_forms/07_project_modeling_magic`

Implement every `todo!()` in `src/lib.rs`, phase by phase, until all tests in `tests/modeling_magic.rs` pass. Then climb the tower: can you defeat the Elder Dragon at level 7? (Hint: read your scrolls, and mind the elements.)

**Bonus ideas** (no tests — your call): let defeated foes drop a potion, add a `Status` command, or level the mage up every 100 XP.

</div>

## 📜 The Grimoire

<div class="grimoire">

- Model with types: struct = *and*, enum = *or*; return *outcomes* as enums
- `match (a, b) { … }` for pairs · `{:?}` prints derived `Debug`
- `x as f64` / `y as u32` (truncates) · `saturating_sub`
- `let Some(x) = … else { return …; };` for early exits
- `match s.as_str() { "list" => …, "cast" if cond => …, _ => … }`
- Disjoint field borrows: `&mut self.mage` and `&mut self.foes[0]` may coexist
- `vec.first()` / `first_mut()` → `Option<&T>` / `Option<&mut T>` · `vec.remove(0)`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — Phase 1</summary>

`(Fire, Frost) | (Frost, Arcane) | (Arcane, Fire) => 2.0` and the mirrored three `=> 0.5`. `take_hit`: `let damage = (power as f64 * element.multiplier(self.element)) as u32;` then `self.hp = self.hp.saturating_sub(damage); damage`.

</details>
<details class="hint"><summary>Hint 2 — Mage::cast</summary>

After the `let … else` for the spell: `if self.level < spell.level { return CastOutcome::LevelTooLow { required: spell.level }; }`, then `let available = self.mana; if !self.spend_mana(spell.cost) { return … }`, then `let damage = target.take_hit(spell.power, spell.element);` and build `Hit { spell: spell.name, damage, defeated: !target.is_alive() }`.

</details>
<details class="hint"><summary>Hint 3 — handle's cast arm</summary>

Match on the outcome with two separate `Hit` arms: `CastOutcome::Hit { spell, damage, defeated: false } => format!(…, foe.name)` and `CastOutcome::Hit { spell, damage, defeated: true } => { let foe = self.foes.remove(0); self.mage.xp += foe.xp; format!(…) }`. The other three outcomes each become one `format!`.

</details>
