// ⚔️ Quest 7.5 — The Tokio Runtime: The Engine of Patience
// 📖 Read: book/src/07_conclave_of_threads/05_tokio_runtime.md
//
// Replace each `todo!()` with real code.
// The timing tests use `#[tokio::test(start_paused = true)]`: tokio's clock
// is frozen and jumps forward whenever every task is sleeping. A "3 second"
// sleep finishes instantly in real time — but `Instant` still sees 3 s pass.

use std::time::Duration;
use tokio::task::JoinHandle;

/// Spawn `n` imp tasks with `tokio::spawn`. Imp number `i` (from 0) returns
/// `i * 2`. Await every `JoinHandle` and return the sum.
/// `summon_imps(4)` → 0 + 2 + 4 + 6 = 12
pub async fn summon_imps(n: u32) -> u32 {
    todo!()
}

/// Cast a slow spell: sleep for `cast_ms` milliseconds
/// (`tokio::time::sleep`, NOT `std::thread::sleep`!), then return
/// "<spell> unleashed!".
pub async fn slow_cast(spell: String, cast_ms: u64) -> String {
    todo!()
}

/// Start casting all spells AT ONCE: spawn one task per `(spell, cast_ms)`
/// running `slow_cast`. Return the results in the same order as the input.
///
/// Because the casts overlap, the total time is the slowest cast, not the sum.
pub async fn cast_concurrently(spells: Vec<(String, u64)>) -> Vec<String> {
    todo!()
}

/// Start a ritual in the background and hand back its `JoinHandle` right
/// away. The task sleeps `ms` milliseconds and then returns `power`.
pub fn begin_ritual(power: u32, ms: u64) -> JoinHandle<u32> {
    todo!()
}

/// Spawn a task that runs `ritual` (it might panic!). Await its handle:
/// - finished normally → `Ok(value)`
/// - panicked          → `Err("the ritual collapsed".to_string())`
pub async fn guarded_ritual(ritual: fn() -> u32) -> Result<u32, String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test]
    async fn imps_are_summoned_and_report_back() {
        assert_eq!(summon_imps(4).await, 12);
        assert_eq!(summon_imps(0).await, 0);
    }

    #[tokio::test(start_paused = true)]
    async fn slow_casts_take_their_time() {
        let start = Instant::now();
        assert_eq!(slow_cast("Meteor".to_string(), 3_000).await, "Meteor unleashed!");
        assert!(
            start.elapsed() >= Duration::from_millis(3_000),
            "⏳ Meteor needs its full 3 s cast time (use tokio::time::sleep)"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn concurrent_casts_overlap() {
        let start = Instant::now();
        let results = cast_concurrently(vec![
            ("Fireball".to_string(), 1_000),
            ("Frostbolt".to_string(), 500),
            ("Arcane Blast".to_string(), 2_000),
        ])
        .await;
        assert_eq!(
            results,
            vec!["Fireball unleashed!", "Frostbolt unleashed!", "Arcane Blast unleashed!"]
        );
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(2_000), "the slowest cast still takes 2 s");
        assert!(
            elapsed < Duration::from_millis(3_500),
            "🐌 took {elapsed:?} — the casts ran one after another instead of together"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn rituals_run_in_the_background() {
        let start = Instant::now();
        let handle = begin_ritual(42, 1_000);
        assert!(start.elapsed() < Duration::from_millis(1_000), "begin_ritual must not wait");
        assert_eq!(handle.await.unwrap(), 42);
        assert!(start.elapsed() >= Duration::from_millis(1_000));
    }

    fn stable() -> u32 {
        7
    }

    fn unstable() -> u32 {
        panic!("the runes crack")
    }

    #[tokio::test]
    async fn a_stable_ritual_succeeds() {
        assert_eq!(guarded_ritual(stable).await, Ok(7));
    }

    #[tokio::test]
    async fn a_collapsing_ritual_is_caught() {
        assert_eq!(guarded_ritual(unstable).await, Err("the ritual collapsed".to_string()));
    }
}
