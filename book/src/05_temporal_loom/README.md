# ⏳ Chapter 5 — The Temporal Loom

> *"A potion's effect has a duration. A buff's power depends on when it was cast. A chain of spells can be woven like threads on a loom — each affecting the next. This is the magic of time."*

This chapter builds on ownership with three tools that separate apprentices from mages:

1. **Lifetimes** — how the compiler knows a reference is still valid, and how you explain it when it can't tell.
2. **Closures** — anonymous functions that capture their surroundings by borrow, mutable borrow, or move.
3. **Iterators** — lazy, chainable sequences that replace most hand-written loops.

Coming from Python or Go, lifetimes are the new idea: there is no garbage collector keeping things alive, so *signatures* have to say what borrows from what. Closures and iterators will feel familiar — generators, lambdas, comprehensions — but Rust tracks exactly what they capture.

| Quest | Topic |
|---|---|
| 5.0 | [Lifetimes — The Thread of Existence](00_lifetimes.md) |
| 5.1 | [Lifetime Elision — The Unspoken Components](01_elision.md) |
| 5.2 | [Lifetimes in Structs — The Crystal Ball](02_lifetimes_in_structs.md) |
| 5.3 | [Closures — Incantations on the Fly](03_closures.md) |
| 5.4 | [Capturing — Borrow, Mutate, or Consume](04_closure_capture.md) |
| 5.5 | [Iterators — The River of Values](05_iterators.md) |
| 5.6 | [Iterator Adapters — Spell Analytics](06_adapters.md) |
| 🏗️ | [Project: Potions, Buffs & Effect Chains](07_project_potions_buffs.md) |
