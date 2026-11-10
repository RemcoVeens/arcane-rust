// ⚔️ Quest 1.3 — Compound Types: Tuples & Arrays
// 📖 Read: book/src/01_call_to_adventure/03_compound_types.md

/// A spell as a tuple: (name, mana cost, power).
/// Return the Fireball: name "Fireball", cost 30, power 45.
pub fn fireball() -> (&'static str, u32, u32) {
    ("Fireball", 30, 45)
}

/// Return only the power (the third element) of a spell tuple.
pub fn spell_power(spell: (&'static str, u32, u32)) -> u32 {
    spell.2
}

/// Destructure the pair and return it the other way around.
pub fn swap(pair: (u32, &'static str)) -> (&'static str, u32) {
    (pair.1, pair.0)
}

/// A spell circle always has exactly 5 rune costs.
/// Return their total. (Loops come later — use indexing for now!)
pub fn circle_cost(costs: [u32; 5]) -> u32 {
    costs[0] + costs[1] + costs[2] + costs[3] + costs[4]
}

/// Create a spell circle where every rune costs `cost`.
/// Hint: there's a short syntax for "the same value N times".
pub fn uniform_circle(cost: u32) -> [u32; 5] {
    [cost; 5]
}

/// The grimoire grid is 3x3. Return the rune in the very center.
pub fn center_rune(grid: [[char; 3]; 3]) -> char {
    grid[1][1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fireball_tuple() {
        assert_eq!(fireball(), ("Fireball", 30, 45));
    }

    #[test]
    fn power_is_the_third_element() {
        assert_eq!(spell_power(("Frostbolt", 25, 35)), 35);
    }

    #[test]
    fn swapping_a_pair() {
        assert_eq!(swap((7, "level")), ("level", 7));
    }

    #[test]
    fn circle_cost_sums_all_runes() {
        assert_eq!(circle_cost([1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn uniform_circle_repeats_cost() {
        assert_eq!(uniform_circle(9), [9, 9, 9, 9, 9]);
    }

    #[test]
    fn center_of_the_grid() {
        let grid = [['a', 'b', 'c'], ['d', '✦', 'f'], ['g', 'h', 'i']];
        assert_eq!(center_rune(grid), '✦');
    }
}
