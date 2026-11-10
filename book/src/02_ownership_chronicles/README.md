# 📜 Chapter 2 — The Ownership Chronicles

> *"A spell does not belong to two mages. A staff cannot be wielded in two hands across a room. Ownership is the first law of magic — and of Rust."*

Python has a garbage collector. Go has a garbage collector. Rust has **neither** — and still never leaks or double-frees memory. The trick is *ownership*: every value has exactly one owner, the compiler tracks who that is, and the value is freed the moment its owner goes away.

This is the chapter where Rust stops feeling like "Go with stricter types" and starts feeling like its own thing. Expect the compiler to say *no* a lot. Every *no* is a bug that would have been a crash, a data race or a use-after-free elsewhere.

| Quest | Topic |
|---|---|
| 2.0 | [Stack & Heap — The Two Realms of Memory](00_stack_heap.md) |
| 2.1 | [The Ownership Rules — Three Laws of Magic](01_ownership_rules.md) |
| 2.2 | [Moves — The Passing of the Staff](02_moves.md) |
| 2.3 | [Clone & Copy — The Scribe's Duplicates](03_clone_copy.md) |
| 2.4 | [References — Borrowing the Grimoire](04_references.md) |
| 2.5 | [Borrowing Rules — The Compact of the Arcane](05_borrowing_rules.md) |
| 2.6 | [Slices — A Window Into the Grimoire](06_slices.md) |
| 🏗️ | [Project: The Mana System](07_project_mana_system.md) |

<div class="grimoire">

**The whole chapter on one card**

- Each value has **one owner**; when the owner goes out of scope, the value is **dropped**.
- Assigning or passing a heap-owning value **moves** it. Small fixed-size values (`u32`, `char`, `bool`, …) are **copied**.
- Want a second, independent copy? `.clone()` — explicitly, because it costs.
- Want to *use* without *owning*? Borrow: `&T` (many readers) **or** `&mut T` (one writer).
- Prefer `&str` and `&[T]` in parameters — slices accept more callers.

</div>
