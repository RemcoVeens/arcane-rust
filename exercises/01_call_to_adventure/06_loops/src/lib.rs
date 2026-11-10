// ⚔️ Quest 1.6 — Loops: The Cyclical Incantations
// 📖 Read: book/src/01_call_to_adventure/06_loops.md

/// How many casts of `damage` does it take to bring an enemy to 0 HP?
/// Use `while`. (Assume damage > 0.)
pub fn casts_to_defeat(enemy_hp: u32, damage: u32) -> u32 {
    let mut count = 0;
    let mut enemy_hp_clone = enemy_hp;
    while enemy_hp_clone > 0 {
        enemy_hp_clone = enemy_hp_clone.saturating_sub(damage);
        count += 1;
    }
    count
}

/// Sum the costs of all spells in the circle using a `for` loop.
pub fn total_cost(costs: [u32; 5]) -> u32 {
    let mut total = 0;
    for cost in costs {
        total += cost
    }
    total
}

/// Count down from `n` to 1, then shout the spell name:
///   countdown(3, "Fireball") → "3... 2... 1... Fireball!"
/// Hint: `(1..=n).rev()`
pub fn countdown(n: u32, spell: &str) -> String {
    let mut res = "".to_string();
    for i in (1..=n).rev() {
        res = format!("{res}{i}... ")
    }
    format!("{res}{spell}!").to_string()
}

/// Meditate until full: each turn restores `regen` mana (never above `max`).
/// Return the number of turns it took.
/// Use `loop` and `break` with a value: `let turns = loop { ... break count; };`
pub fn turns_to_full(mana: u32, regen: u32, max: u32) -> u32 {
    let mut count = 0;
    let mut mmana = mana;
    loop {
        if mmana == max {
            break count;
        } else {
            mmana = (mmana + regen).min(max);
            count += 1
        }
    }
}

/// Search the 3x3 rune grid for `target` and return its (row, col).
/// If the rune isn't there, return (99, 99).
/// Bonus style points: use a labeled loop `'search: for ...` and `break 'search`.
pub fn find_rune(grid: [[char; 3]; 3], target: char) -> (usize, usize) {
    for (i, row) in grid.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            if *value == target {
                return (i, j);
            }
        }
    }
    (99, 99)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defeating_enemies() {
        assert_eq!(casts_to_defeat(100, 30), 4);
        assert_eq!(casts_to_defeat(90, 30), 3);
        assert_eq!(casts_to_defeat(0, 30), 0);
    }

    #[test]
    fn summing_a_circle() {
        assert_eq!(total_cost([30, 25, 40, 20, 15]), 130);
    }

    #[test]
    fn counting_down() {
        assert_eq!(countdown(3, "Fireball"), "3... 2... 1... Fireball!");
        assert_eq!(countdown(1, "Heal"), "1... Heal!");
    }

    #[test]
    fn meditating_until_full() {
        assert_eq!(turns_to_full(50, 20, 100), 3);
        assert_eq!(turns_to_full(100, 20, 100), 0);
    }

    #[test]
    fn finding_runes() {
        let grid = [['a', 'b', 'c'], ['d', 'e', 'f'], ['g', 'h', '✦']];
        assert_eq!(find_rune(grid, '✦'), (2, 2));
        assert_eq!(find_rune(grid, 'd'), (1, 0));
        assert_eq!(find_rune(grid, 'z'), (99, 99));
    }
}
