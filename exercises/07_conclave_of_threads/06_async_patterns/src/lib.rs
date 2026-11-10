// ⚔️ Quest 7.6 — Async Patterns: The Weaving of Many Spells
// 📖 Read: book/src/07_conclave_of_threads/06_async_patterns.md
//
// Replace each `todo!()` with real code using `tokio::join!`,
// `tokio::select!` and `tokio::time::timeout`.
// All timing tests run on tokio's paused clock: they finish instantly.

use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::sleep;

async fn fetch(item: &str, ms: u64) -> String {
    sleep(Duration::from_millis(ms)).await;
    item.to_string()
}

async fn count_coins(ms: u64) -> u32 {
    sleep(Duration::from_millis(ms)).await;
    250
}

/// Gather all reagents AT THE SAME TIME with `tokio::join!`:
/// - `fetch("dragon scale", 300)`
/// - `fetch("phoenix feather", 200)`
/// - `count_coins(100)`
///
/// Return `(scale, feather, coins)`. Total time ≈ 300 ms, not 600 ms.
pub async fn gather_reagents() -> (String, String, u32) {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum CastError {
    /// The spell took longer than the allowed time.
    Fizzled,
}

/// Cast a spell that takes `cast_ms` to complete, but give up after
/// `limit_ms` (`tokio::time::timeout`).
///
/// - finished in time → `Ok("<spell> lands!")`
/// - too slow         → `Err(CastError::Fizzled)`
pub async fn cast_with_timeout(spell: &str, cast_ms: u64, limit_ms: u64) -> Result<String, CastError> {
    todo!()
}

/// Two mages race: one casts Fireball (taking `fire_ms`), the other
/// Frostbolt (taking `frost_ms`). Use `tokio::select!` to return the name of
/// the spell that lands FIRST: "Fireball" or "Frostbolt".
/// The losing spell is dropped (cancelled) automatically.
pub async fn duel(fire_ms: u64, frost_ms: u64) -> &'static str {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Channeling {
    /// The spell completed after the full channel time.
    Completed,
    /// A message arrived on the interrupt channel first.
    Interrupted,
}

/// Channel a long spell for `channel_ms`, unless an interrupt arrives first
/// on `interrupts` (`tokio::select!` between a sleep and `interrupts.recv()`).
///
/// ⚠️ If the interrupt channel is CLOSED (all senders dropped), `recv()`
/// returns `None` immediately — that is NOT an interrupt; keep channeling.
/// (Hint: a select! branch pattern like `Some(()) = interrupts.recv()` is
/// disabled when the pattern doesn't match.)
pub async fn channel_spell(channel_ms: u64, mut interrupts: mpsc::Receiver<()>) -> Channeling {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::Instant;

    #[tokio::test(start_paused = true)]
    async fn reagents_are_gathered_together() {
        let start = Instant::now();
        let reagents = gather_reagents().await;
        assert_eq!(reagents, ("dragon scale".to_string(), "phoenix feather".to_string(), 250));
        assert!(
            start.elapsed() < Duration::from_millis(400),
            "🐌 took {:?} — use join! so the fetches overlap",
            start.elapsed()
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_quick_spell_lands() {
        assert_eq!(cast_with_timeout("Fireball", 100, 500).await, Ok("Fireball lands!".to_string()));
    }

    #[tokio::test(start_paused = true)]
    async fn a_slow_spell_fizzles_on_time() {
        let start = Instant::now();
        assert_eq!(cast_with_timeout("Meteor", 5_000, 1_000).await, Err(CastError::Fizzled));
        assert!(
            start.elapsed() < Duration::from_millis(2_000),
            "⌛ the timeout should cut the cast short"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn the_faster_spell_wins_the_duel() {
        assert_eq!(duel(300, 500).await, "Fireball");
        assert_eq!(duel(800, 200).await, "Frostbolt");
    }

    #[tokio::test(start_paused = true)]
    async fn an_uninterrupted_channel_completes() {
        let (tx, rx) = mpsc::channel(1);
        let start = Instant::now();
        assert_eq!(channel_spell(1_000, rx).await, Channeling::Completed);
        assert!(start.elapsed() >= Duration::from_millis(1_000));
        drop(tx);
    }

    #[tokio::test(start_paused = true)]
    async fn an_interrupt_breaks_the_channel() {
        let (tx, rx) = mpsc::channel(1);
        tokio::spawn(async move {
            sleep(Duration::from_millis(300)).await;
            tx.send(()).await.unwrap();
        });
        let start = Instant::now();
        assert_eq!(channel_spell(1_000, rx).await, Channeling::Interrupted);
        assert!(start.elapsed() < Duration::from_millis(1_000), "interrupted after ~300 ms");
    }

    #[tokio::test(start_paused = true)]
    async fn a_closed_interrupt_channel_is_not_an_interrupt() {
        let (tx, rx) = mpsc::channel::<()>(1);
        drop(tx); // nobody can interrupt any more
        assert_eq!(channel_spell(1_000, rx).await, Channeling::Completed);
    }
}
