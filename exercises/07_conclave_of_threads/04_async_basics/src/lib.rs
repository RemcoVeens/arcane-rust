// ⚔️ Quest 7.4 — Async Basics: The Patient Incantation
// 📖 Read: book/src/07_conclave_of_threads/04_async_basics.md
//
// Replace each `todo!()` with real code. The tests use `#[tokio::test]`,
// which builds a small runtime that drives your futures to completion.

use std::future::Future;
use std::sync::Mutex;

/// Brew a potion: returns "Potion of <ingredient>".
pub async fn brew_potion(ingredient: &str) -> String {
    todo!()
}

/// Brew every ingredient, one after the other (`.await` each brew in turn),
/// returning the potions in order.
pub async fn brew_all(ingredients: &[&str]) -> Vec<String> {
    todo!()
}

/// Charging a spell doubles its power.
pub async fn charge(power: u32) -> u32 {
    power * 2
}

/// A charged Fireball: charge the Fireball's base power (45) TWICE
/// (charge the result of the first charge), then add 10 for style.
/// → 45 * 2 * 2 + 10 = 190
pub async fn charged_fireball() -> u32 {
    todo!()
}

/// Return a future (an `async move` block) that, when awaited, yields
/// `power * 3`. This function itself is NOT `async` — it hands back a
/// future that hasn't run yet.
pub fn delayed_power(power: u32) -> impl Future<Output = u32> {
    // TODO: replace the block below with an `async move { ... }` block
    std::future::ready(todo!())
}

/// Brewing with a log: pushes "brewing <ingredient>" onto the log when the
/// future actually *runs*, then returns "Potion of <ingredient>".
pub async fn brew_logged(ingredient: &str, log: &Mutex<Vec<String>>) -> String {
    log.lock().unwrap().push(format!("brewing {ingredient}"));
    format!("Potion of {ingredient}")
}

/// Prove that futures are lazy. In THIS order:
///   1. create the future `brew_logged("moonpetal", log)` — but don't await it yet
///   2. push "cauldron lit" onto the log
///   3. await the future and push the potion it returns onto the log
///
/// The log must end up as ["cauldron lit", "brewing moonpetal", "Potion of moonpetal"].
pub async fn lazy_cauldron(log: &Mutex<Vec<String>>) {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn a_single_potion() {
        assert_eq!(brew_potion("nightshade").await, "Potion of nightshade");
    }

    #[tokio::test]
    async fn brewing_a_whole_batch() {
        assert_eq!(
            brew_all(&["moonpetal", "dragonroot", "frostmoss"]).await,
            vec!["Potion of moonpetal", "Potion of dragonroot", "Potion of frostmoss"]
        );
    }

    #[tokio::test]
    async fn charging_twice_makes_a_mighty_fireball() {
        assert_eq!(charged_fireball().await, 190);
    }

    #[tokio::test]
    async fn delayed_power_is_a_future() {
        let fut = delayed_power(20);
        assert_eq!(fut.await, 60);
    }

    #[test]
    fn creating_a_future_runs_nothing() {
        // No runtime here at all: merely *creating* futures must not run them.
        let log = Mutex::new(Vec::new());
        let _never_awaited = brew_logged("ghostcap", &log);
        let _also_never = lazy_cauldron(&log);
        assert!(log.lock().unwrap().is_empty(), "💤 futures do nothing until awaited");
    }

    #[tokio::test]
    async fn the_lazy_cauldron() {
        let log = Mutex::new(Vec::new());
        lazy_cauldron(&log).await;
        assert_eq!(
            *log.lock().unwrap(),
            vec!["cauldron lit", "brewing moonpetal", "Potion of moonpetal"],
            "🧪 the brew only starts when it is awaited"
        );
    }
}
