# 📜 Chapter 7 — The Conclave of Threads

> *"A single mage casts one spell at a time. A conclave casts in unison. But many hands on one grimoire tear its pages — unless they follow the ancient pacts."*

Concurrency is where Rust's ownership rules pay their biggest dividend. The same borrow checker that stopped you from using a moved `String` now stops you from sharing a counter between threads without a lock. The Rust community calls it **fearless concurrency**: data races are *compile errors*, not 3 a.m. pager alerts.

You will meet two models:

- **Threads** (`std::thread`) — real OS threads, like Python's `threading` (minus the GIL) or Go's goroutines (minus the scheduler). Great for CPU work and simple parallelism.
- **Async** (`async`/`.await` + the **Tokio** runtime) — thousands of cheap tasks multiplexed on a few threads. Great for waiting: network, timers, I/O. Closest to Python's `asyncio`; Go hides this inside goroutines.

<div class="compare">

🐍 Python: the GIL serialises your threads; `asyncio` for I/O. · 🐹 Go: goroutines + channels, data races caught (sometimes) by `-race` at runtime. · 🦀 Rust: OS threads *and* async tasks, data races rejected at compile time.

</div>

| Quest | Topic |
|---|---|
| 7.0 | [Threads — The Summoning of Familiars](00_threads.md) |
| 7.1 | [Channels — The Whispering Stones](01_channels.md) |
| 7.2 | [Arc & Mutex — The Shared Mana Pool](02_arc_mutex.md) |
| 7.3 | [Send & Sync — The Pacts of Passage](03_send_sync.md) |
| 7.4 | [Async Basics — The Patient Incantation](04_async_basics.md) |
| 7.5 | [The Tokio Runtime — The Engine of Patience](05_tokio_runtime.md) |
| 7.6 | [Async Patterns — The Weaving of Many Spells](06_async_patterns.md) |
| 🏗️ | [Project: The Grand Battle](07_project_grand_battle.md) |

A word about the tests in this chapter: concurrent code runs in an unpredictable order, so the tests never depend on *which* thread finishes first. They check totals, sorted results, and — for async code — use Tokio's **paused clock**, so a "3 second" spell finishes instantly yet still *measures* 3 seconds.
