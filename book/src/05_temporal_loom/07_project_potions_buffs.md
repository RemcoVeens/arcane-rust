# 🏗️ Project 5 — Potions, Buffs & Effect Chains

> *"A potion's power is spent the moment it is drunk. A buff fades with the turning of the hourglass. A curse lingers, burning turn after turn. Time is the dimension that gives magic meaning — and you now hold the loom."*

## 🎯 The Goal

Your Chapter 4 combat system gets **time**: consumable potions, buffs that modify spell power and wear off, burning damage-over-time, and battle statistics. Every new feature is built from this chapter's tools:

| Feature | Built with |
|---|---|
| Potions | `Box<dyn FnOnce(&mut Mage)>` — an effect you can use exactly once |
| Buffs & effect chains | `Box<dyn Fn(u32) -> u32>` + `fold` |
| `Mage::cast_with` | a generic `impl Fn(u32) -> u32` parameter |
| Timed effects | `iter_mut` + `retain` every turn |
| Battle stats | iterator chains, `HashMap`, elided lifetimes on borrowed results |

## 🧱 The Scaffold

<div class="quest">

⚔️ **Project:** `exercises/05_temporal_loom/07_project_potions_buffs`

- `src/lib.rs` — the top half is the **provided** Chapter 4 combat system: `Spell`, `spellbook()`, `CastError`, the `Creature` trait with `Goblin`/`Wraith`/`Dragon`, and `Mage` with `find_spell`, `spend_for` and `cast`. The bottom half is yours: every `todo!()`.
- `src/main.rs` — a playable battle, already written. It works once your lib does: `cargo run -p ch05_07_project_potions_buffs`
- `tests/potions_buffs.rs` — the tests, grouped by phase.

</div>

Read the provided code first. Notice `Mage::spend_for`: it checks level and mana, spends the mana and returns the spell. You'll reuse it.

## Phase 1 — Potions

A potion stores its effect as a boxed closure. Drinking uses the potion up, so `drink` takes `self` and the effect is `FnOnce`:

```rust,ignore
pub struct Potion {
    pub name: String,
    effect: Box<dyn FnOnce(&mut Mage)>,
}
```

1. `Potion::new` boxes any `impl FnOnce(&mut Mage) + 'static`. The `'static` bound means "the closure doesn't borrow anything short-lived" — it must own its captures, because the potion may sit in a satchel for a long time.
2. `Potion::drink` calls the effect. A boxed closure stored in a field is called with parentheses around the field: `(self.effect)(mage)`.
3. `healing_draught(amount)` and `mana_elixir(amount)` are just `Potion::new` with a `move` closure. Cap at `max_hp` / `MAX_MANA`.
4. `Satchel::names` and `Satchel::drink` — find the potion with `position`, take it out with `Vec::remove`, drink it.

Why `FnOnce`? The test `potions_can_carry_captured_treasure` brews a potion that *moves* a `Spell` into the mage's spellbook. Only an `FnOnce` closure may give away what it captured.

## Phase 2 — Buffs & effect chains

```rust,ignore
pub struct Buff {
    pub name: String,
    pub turns_left: u32,
    modifier: Box<dyn Fn(u32) -> u32>,
}
```

Buffs modify power over and over, so they're `Fn`.

1. `Buff::new` and `Buff::apply`.
2. `empower(percent, turns)` — "Empower", `power * (100 + percent) / 100`.
3. `arcane_focus(bonus, turns)` — "Arcane Focus", `power + bonus`.
4. `chain_power(buffs, power)` — feed the power through each buff in order. One `fold`.
5. `Mage::cast_with(name, target, modifier)` — like `cast`, but the spell's power goes through `modifier` first. Reuse `spend_for` and `?`.

The game plugs them together like this — a closure that borrows the buff list:

```rust,ignore
mage.cast_with("Fireball", target, |p| chain_power(&buffs, p))
```

## Phase 3 — Timed effects

The loom turns once per player action:

1. `tick_buffs` — every buff loses a turn; buffs with 0 turns left vanish. `iter_mut` to decrement, `retain` to clean up.
2. `tick_burns` — every `Burn` deals `damage_per_turn` to the target (through `take_damage`, so the Wraith's resistance applies!), loses a turn, and expired burns are removed. Return the total damage passed to `take_damage`.

<div class="pitfall">

`burns.iter_mut().map(|b| { target.take_damage(...); ... })` — the closure mutably borrows `target` while `iter_mut` mutably borrows `burns`. That's fine: they're two *different* values. What you can't do is `retain` while the `iter_mut` is still alive — finish one before starting the other.

</div>

## Phase 4 — Battle statistics

The game records a `CastRecord { turn, spell, damage }` for every successful cast.

1. `total_damage` and `battle_report` ("Turn 1: Fireball for 45", lines joined by `'\n'`).
2. `spell_counts` → `HashMap<&str, usize>`. The keys **borrow** from the log — elision ties them to `log` automatically.
3. `favourite_spell` → `Option<&str>`: the most frequent; on a tie the alphabetically first. Build on `spell_counts` and `max_by` with a comparison that breaks ties.
4. `living` and `weakest_living` over `&[Box<dyn Creature>]`.

Tip for the tie-break: `a.1.cmp(&b.1).then_with(|| b.0.cmp(a.0))` compares counts, and on equal counts prefers the *smaller* name as the "maximum".

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Calling a boxed closure field:** `self.modifier(power)` looks for a *method* called `modifier`. Write `(self.modifier)(power)`.
- **`'static` on closure bounds** rejects closures that borrow locals. Use `move` and capture owned/`Copy` values.
- **Temporaries and borrowed results:** `spell_counts(&log())` creates a temporary `Vec` that dies at the end of the statement — but the returned map borrows it. Bind the log to a variable first.
- **`min_by_key` keeps the first minimum**, `max_by_key` the last maximum.

</div>

## 📜 The Grimoire

<div class="grimoire">

- `Box<dyn FnOnce(&mut Mage)>` — call once by value: `(self.effect)(mage)` in a `fn drink(self, ..)`
- `Box<dyn Fn(u32) -> u32>` — call many times: `(self.modifier)(x)`
- `impl Fn(u32) -> u32 + 'static` — accept any owning closure for boxing
- `buffs.iter().fold(power, |p, b| b.apply(p))`
- `v.iter_mut().for_each(...)` / `v.retain(|x| x.turns_left > 0)`
- `v.iter().position(|p| ...)` + `v.remove(i)`
- `.enumerate().filter(..).min_by_key(..).map(|(i, _)| i)`

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — Phase 1</summary>

`Potion::new`: `Potion { name: name.to_string(), effect: Box::new(effect) }`. `healing_draught`: `Potion::new("Healing Draught", move |mage: &mut Mage| mage.hp = (mage.hp + amount).min(mage.max_hp))`. Annotating `mage: &mut Mage` helps the compiler infer the closure signature.

</details>
<details class="hint"><summary>Hint 2 — Phases 2 & 3</summary>

`cast_with`: `let spell = self.spend_for(name)?; let power = modifier(spell.power); target.take_damage(power); Ok(power)`. `tick_buffs`: decrement with `saturating_sub(1)` in a `for buff in buffs.iter_mut()` loop, then `buffs.retain(|b| b.turns_left > 0)`.

</details>
<details class="hint"><summary>Hint 3 — Phase 4</summary>

`spell_counts`: `log.iter().fold(HashMap::new(), |mut m, r| { *m.entry(r.spell.as_str()).or_insert(0) += 1; m })`. `favourite_spell`: `spell_counts(log).into_iter().max_by(|a, b| a.1.cmp(&b.1).then_with(|| b.0.cmp(a.0))).map(|(name, _)| name)`. `weakest_living`: `.iter().enumerate().filter(|(_, c)| c.is_alive()).min_by_key(|(_, c)| c.hp()).map(|(i, _)| i)`.

</details>

## 🏆 Victory Condition

All tests in `tests/potions_buffs.rs` pass — then play a round. Try `focus`, then `cast 3 arcane blast`, `ignite 3`, and watch the dragon burn.
