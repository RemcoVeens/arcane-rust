// ⚔️ Quest 3.5 — If Let & Let-Else: The Single-Glance Divination
// 📖 Read: book/src/03_grimoire_of_forms/05_if_let.md
//
// Replace each `todo!()`. Use `if let`, `while let` and `let ... else`
// — a full `match` would work too, but that's not the quest.

pub const MAX_MANA: u32 = 100;

#[derive(Debug, Clone, PartialEq)]
pub enum Loot {
    Gold(u32),
    Scroll(String),
    Potion { mana: u32 },
    Nothing,
}

/// The gold value of a piece of loot: Gold(n) is worth n, all else 0.
/// Use `if let ... else`.
pub fn gold_value(loot: &Loot) -> u32 {
    todo!()
}

/// If the loot is a potion, drink it: add its mana (capped at MAX_MANA)
/// and return true. Anything else: leave the mana alone, return false.
pub fn drink_if_potion(mana: &mut u32, loot: &Loot) -> bool {
    todo!()
}

/// The names of all scrolls in a chest, in order.
/// A `for` loop with an `if let` inside.
pub fn scroll_names(chest: &[Loot]) -> Vec<&str> {
    todo!()
}

/// Read scrolls from the top of the pile (the END of the Vec) until none
/// are left, and return what was read, joined with ", ".
///   ["Fireball", "Heal", "Meteor"] → "Meteor, Heal, Fireball"
/// Use `while let Some(scroll) = pile.pop()`.
pub fn read_pile(pile: Vec<String>) -> String {
    todo!()
}

/// Provided: the mana cost of a known spell.
pub fn spell_cost(name: &str) -> Option<u32> {
    match name {
        "Fireball" => Some(30),
        "Frostbolt" => Some(25),
        "Heal" => Some(20),
        _ => None,
    }
}

/// Parse a cast command with two `let ... else` guards:
///   "cast Fireball" → "Casting Fireball for 30 mana."
///   "cast Dance"    → "Unknown spell: Dance"
///   "dance"         → "That is not a cast command."
/// Hint: `input.strip_prefix("cast ")` returns an Option<&str>.
pub fn parse_cast(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold_has_value() {
        assert_eq!(gold_value(&Loot::Gold(42)), 42);
        assert_eq!(gold_value(&Loot::Scroll(String::from("Heal"))), 0);
        assert_eq!(gold_value(&Loot::Nothing), 0);
    }

    #[test]
    fn drinking_potions_from_loot() {
        let mut mana = 30;
        assert!(drink_if_potion(&mut mana, &Loot::Potion { mana: 50 }));
        assert_eq!(mana, 80);
        assert!(drink_if_potion(&mut mana, &Loot::Potion { mana: 50 }));
        assert_eq!(mana, MAX_MANA);
        mana = 10;
        assert!(!drink_if_potion(&mut mana, &Loot::Gold(5)));
        assert_eq!(mana, 10);
    }

    #[test]
    fn finding_scrolls_in_a_chest() {
        let chest = [
            Loot::Gold(5),
            Loot::Scroll(String::from("Fireball")),
            Loot::Nothing,
            Loot::Scroll(String::from("Meteor")),
        ];
        assert_eq!(scroll_names(&chest), vec!["Fireball", "Meteor"]);
        assert!(scroll_names(&[]).is_empty());
    }

    #[test]
    fn reading_the_pile_from_the_top() {
        let pile = vec![String::from("Fireball"), String::from("Heal"), String::from("Meteor")];
        assert_eq!(read_pile(pile), "Meteor, Heal, Fireball");
        assert_eq!(read_pile(Vec::new()), "");
    }

    #[test]
    fn parsing_cast_commands() {
        assert_eq!(parse_cast("cast Fireball"), "Casting Fireball for 30 mana.");
        assert_eq!(parse_cast("cast Dance"), "Unknown spell: Dance");
        assert_eq!(parse_cast("dance"), "That is not a cast command.");
    }
}
