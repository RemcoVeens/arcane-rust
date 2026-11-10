// ⚔️ Quest 3.1 — Tuple & Unit Structs: Minimal Forms
// 📖 Read: book/src/03_grimoire_of_forms/01_tuple_unit_structs.md
//
// ❌ This file does not compile yet: three types are missing.
// Define them, then replace each `todo!()`.

pub const MAX_HEALTH: u32 = 100;

/// Newtypes: a u32 in a labelled box. `Mana(30)` and `Health(30)` are
/// different types — you can't mix them up by accident.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mana(pub u32);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health(pub u32);

// TODO: define these, each with #[derive(Debug, Clone, Copy, PartialEq)]:
//   - `pub struct Gold`: a newtype around a u32 (public field)
//   - `pub struct Position`: a tuple struct of two i32 (x, y), both public
//   - `pub struct Ward`: a unit struct (no fields at all)

/// Spend mana on a spell. Never below zero.
pub fn drain(mana: Mana, cost: u32) -> Mana {
    todo!()
}

/// Heal, capped at MAX_HEALTH.
pub fn heal(health: Health, amount: u32) -> Health {
    todo!()
}

/// Split the loot evenly between `mages` mages (integer division).
/// Assume mages > 0.
pub fn share(loot: Gold, mages: u32) -> Gold {
    todo!()
}

/// One step north increases y by 1.
pub fn step_north(pos: Position) -> Position {
    todo!()
}

/// Manhattan distance: |x1 - x2| + |y1 - y2|.
/// Try destructuring: `let Position(x1, y1) = a;`
/// Hint: `i32::abs_diff` returns a u32.
pub fn distance(a: Position, b: Position) -> u32 {
    todo!()
}

/// Raise a ward. A unit struct is created just by writing its name.
pub fn raise_ward() -> Ward {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draining_mana() {
        assert_eq!(drain(Mana(100), 30), Mana(70));
        assert_eq!(drain(Mana(10), 30), Mana(0), "💀 mana can't go below zero");
    }

    #[test]
    fn healing_is_capped() {
        assert_eq!(heal(Health(50), 30), Health(80));
        assert_eq!(heal(Health(90), 30), Health(MAX_HEALTH));
    }

    #[test]
    fn sharing_gold() {
        assert_eq!(share(Gold(100), 3), Gold(33));
    }

    #[test]
    fn walking_north() {
        assert_eq!(step_north(Position(2, -1)), Position(2, 0));
    }

    #[test]
    fn manhattan_distance() {
        assert_eq!(distance(Position(0, 0), Position(3, 4)), 7);
        assert_eq!(distance(Position(-2, 5), Position(2, -5)), 14);
    }

    #[test]
    fn wards_take_no_space() {
        assert_eq!(raise_ward(), Ward);
        assert_eq!(size_of::<Ward>(), 0, "a unit struct is zero-sized");
        assert_eq!(size_of::<Mana>(), size_of::<u32>(), "a newtype costs nothing extra");
    }
}
