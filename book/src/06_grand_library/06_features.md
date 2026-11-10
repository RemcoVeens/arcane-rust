# ⚔️ 6.6 — Profiles & Features: The Forge Settings

> *"A forge needs different heat for different tasks. A spellbook needs different editions for apprentices, adepts and dragon-hunters. Rust lets you choose — when you build, not when you run."*

## ⚡ The Concept

### Profiles: how to compile

A **profile** is a set of compiler settings. You already use two:

| Command | Profile | Optimised | Debug info | Overflow checks |
|---|---|---|---|---|
| `cargo build` / `cargo test` | `dev` | no (fast compile) | yes | **panic** on overflow |
| `cargo build --release` | `release` | yes (`opt-level = 3`) | no | **wrap** silently |

You can tune them in the (workspace root) `Cargo.toml`:

```toml
[profile.dev]
opt-level = 1          # a bit faster debug builds

[profile.release]
lto = true             # link-time optimisation: smaller, faster binary
```

<div class="pitfall">

The overflow row matters: `u8` arithmetic that panics in your tests *wraps* in release. That's why this course keeps asking for `saturating_sub`, `checked_add` and `.min()` — they behave the same in every profile.

</div>

### Features: what to compile

A **feature** is a named on/off switch, declared in `Cargo.toml`:

```toml
[features]
default = []                    # features on unless --no-default-features
enchanted = []                  # a plain switch
dragon_lore = ["enchanted"]     # turning this on turns enchanted on too
```

Code reads the switches at compile time:

```rust,ignore
#[cfg(feature = "enchanted")]           // item exists only with the feature
pub fn arcane_blast() -> u32 { 60 }

#[cfg(feature = "dragon_lore")]
pub mod dragon_lore { /* … */ }         // works on whole modules too

pub fn multiplier() -> u32 {
    if cfg!(feature = "enchanted") { 2 } else { 1 }   // cfg! = a const bool
}
```

| | `#[cfg(…)]` | `cfg!(…)` |
|---|---|---|
| What | attribute: removes the item/statement entirely | macro: evaluates to `true`/`false` |
| Code in the "off" branch | not compiled at all | must still compile |
| Use for | items, modules, imports, statements | simple value choices |

`#[cfg]` also works on statements inside a function, e.g. `#[cfg(feature = "enchanted")] spells.push("Arcane Blast");`. Combinators: `all(…)`, `any(…)`, `not(…)` — e.g. `#[cfg(not(feature = "enchanted"))]`.

Turn features on from the command line:

```bash
cargo test -p ch06_06_features --features enchanted
cargo test -p ch06_06_features --features "enchanted dragon_lore"
cargo test -p ch06_06_features --all-features
```

or from a dependent crate: `spellbook = { path = "…", features = ["enchanted"] }`. Crates on crates.io use features to make heavy parts optional — like `tokio = { features = ["full"] }` in our workspace root.

<div class="compare">

🐍 Python: "extras" (`pip install lib[fast]`) choose *dependencies*, but the code checks at runtime with `try: import`. · 🐹 Go: build tags (`//go:build enchanted`) — the closest match, per file. · 🦀 Rust: features choose both dependencies and code, per item, at compile time — the switched-off code isn't even in the binary.

</div>

## ⚠️ Arcane Pitfalls

<div class="pitfall">

- **Features must be additive.** Cargo unifies features: if any crate in the build enables `enchanted`, everyone gets it. A feature that *removes* functionality breaks other users. Never write `feature = "no_fireball"`.
- **Typos are silent-ish.** `#[cfg(feature = "enchantd")]` is simply never true. Rust warns (`unexpected_cfgs`) about features not declared in `Cargo.toml` — read your warnings.
- **Test every combination you support.** Code behind a feature is not even type-checked when it's off. CI often runs `cargo test`, `--all-features` and `--no-default-features`.
- **`let mut` that's only mutated behind a cfg** triggers an `unused_mut` warning when the feature is off.

</div>

## 🎯 Your Quest

<div class="quest">

⚔️ **Quest:** `exercises/06_grand_library/06_features`

1. Declare the features `enchanted` and `dragon_lore` (which enables `enchanted`) in the quest's `Cargo.toml`.
2. Implement `edition`, `spell_list` and `empowered` so each edition behaves as documented.
3. Add a `dragon_lore` module that only exists with the `dragon_lore` feature.

</div>

`wr` runs `cargo test`, `cargo test --features enchanted` and `cargo test --features dragon_lore` — all three must pass. Notice that the tests are feature-gated too: each run checks a different edition.

## 📜 The Grimoire

<div class="grimoire">

- Profiles: `dev` (debug, overflow panics) · `release` (`--release`, optimised, overflow wraps) · `[profile.*]`
- `[features] a = []`, `b = ["a"]`, `default = [...]`
- `#[cfg(feature = "a")]` removes code · `cfg!(feature = "a")` is a bool
- `all()`, `any()`, `not()` · `--features a` · `--all-features` · `--no-default-features`
- Features are additive and unified across the build

</div>

## 🔮 Hints

<details class="hint"><summary>Hint 1 — Cargo.toml</summary>

Add a `[features]` table at the bottom of the quest's `Cargo.toml`. A feature that implies another lists it in its array.

</details>
<details class="hint"><summary>Hint 2 — the functions</summary>

`edition` and `empowered` are a good fit for `if cfg!(feature = "…") { … } else { … }`. Check `dragon_lore` *before* `enchanted` — when dragon_lore is on, both are true.

</details>
<details class="hint"><summary>Hint 3 — spell_list and the module</summary>

Build the base `vec![…]` with `let mut`, then `#[cfg(feature = "enchanted")] spells.push("Arcane Blast");` and the same for Meteor. Put `#[allow(unused_mut)]` on the `let` to keep the Apprentice Edition warning-free. The module is `#[cfg(feature = "dragon_lore")] pub mod dragon_lore { pub fn true_name(…) -> String { format!(…) } }`.

</details>
