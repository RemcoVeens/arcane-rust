# 🔥 Chapter 4 — The Forge of Resilience

> *"A mage without resilience is a candle in a hurricane. Spells fail. Creatures resist. Collections grow and shrink. You need tools that bend without breaking."*

Up to now your data had a fixed shape: tuples, arrays, structs. Real adventures are messier. This chapter forges four pillars of robust Rust:

1. **Collections** — `Vec<T>`, `String`, `HashMap<K, V>`: data that grows at runtime.
2. **Error handling** — `panic!` for bugs, `Result` for failures you expect, `?` to pass them up, and your own error types.
3. **Generics** — one function or struct for many types, checked at compile time.
4. **Traits** — shared behaviour (Go's interfaces, Python's protocols — but explicit), and trait objects for mixing different types in one collection.

By the end, your mage fights a real battle: a horde of different creatures in one `Vec<Box<dyn Creature>>`, a spellbook, an inventory, and every failure reported as a typed `CastError`.

| Quest | Topic |
|---|---|
| 4.0 | [Vectors — The Dynamic Grimoire](00_vectors.md) |
| 4.1 | [Strings — The Weave of Incantations](01_strings.md) |
| 4.2 | [HashMaps — The Lexicon of Magic](02_hashmaps.md) |
| 4.3 | [Panic! — When Magic Goes Critical](03_panic.md) |
| 4.4 | [Result — The Fork in the Road](04_result.md) |
| 4.5 | [The `?` Operator — The Path of Least Resistance](05_question_mark.md) |
| 4.6 | [Custom Errors — Forging Your Own Failure Modes](06_custom_errors.md) |
| 4.7 | [Generics — The Universal Incantations](07_generics.md) |
| 4.8 | [Traits — The Schools of Magic](08_traits.md) |
| 4.9 | [Trait Objects — The Shapeshifter's Art](09_trait_objects.md) |
| 🏗️ | [Project: The Combat System](10_project_combat.md) |

<div class="pitfall">

**No closures yet.** Many idiomatic one-liners (`.iter().map(|x| ...)`, `.find(|s| ...)`) use closures, which arrive in Chapter 5. In this chapter, a plain `for` loop is the right answer — and often just as clear.

</div>
