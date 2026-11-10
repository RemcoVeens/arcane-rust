// ⚔️ Quest 4.2 — HashMaps: The Lexicon of Magic
// 📖 Read: book/src/04_forge_of_resilience/02_hashmaps.md
//
// Replace each `todo!()` with real code. The `entry` API is your friend.

use std::collections::HashMap;

/// An inventory maps item names to how many of them you carry.
pub type Inventory = HashMap<String, u32>;

/// Put `qty` of `item` in the bag. If you already have some, add to the pile.
pub fn add_item(inv: &mut Inventory, item: &str, qty: u32) {
    todo!()
}

/// How many of `item` do you carry? (0 if none.)
pub fn count(inv: &Inventory, item: &str) -> u32 {
    todo!()
}

/// Use one `item`. Returns `false` if you have none.
/// When the last one is used, the item disappears from the map entirely.
pub fn use_item(inv: &mut Inventory, item: &str) -> bool {
    todo!()
}

/// The total number of things in the bag.
pub fn total_items(inv: &Inventory) -> u32 {
    todo!()
}

/// All items as `(name, qty)` pairs, sorted alphabetically by name.
/// (HashMap order is random — sort it so the shopkeeper can read it!)
pub fn sorted_items(inv: &Inventory) -> Vec<(String, u32)> {
    todo!()
}

/// Count how often each word appears in an incantation.
/// Words are separated by whitespace and counted case-insensitively:
/// `"Ignis ignis MAGNA"` → `{"ignis": 2, "magna": 1}`
pub fn word_frequency(text: &str) -> HashMap<String, u32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_new_items() {
        let mut bag = Inventory::new();
        add_item(&mut bag, "mana potion", 2);
        assert_eq!(count(&bag, "mana potion"), 2);
        assert_eq!(count(&bag, "dragon scale"), 0, "missing items count as zero");
    }

    #[test]
    fn adding_to_an_existing_pile() {
        let mut bag = Inventory::new();
        add_item(&mut bag, "mana potion", 2);
        add_item(&mut bag, "mana potion", 3);
        assert_eq!(count(&bag, "mana potion"), 5, "🧪 piles should stack, not overwrite");
    }

    #[test]
    fn using_items_until_they_are_gone() {
        let mut bag = Inventory::new();
        add_item(&mut bag, "phoenix feather", 1);
        assert!(use_item(&mut bag, "phoenix feather"));
        assert!(!use_item(&mut bag, "phoenix feather"), "the feather is spent");
        assert!(
            !bag.contains_key("phoenix feather"),
            "an item with 0 left must be removed from the map"
        );
    }

    #[test]
    fn using_what_you_never_had() {
        let mut bag = Inventory::new();
        assert!(!use_item(&mut bag, "unicorn horn"));
        assert!(bag.is_empty(), "using a missing item must not create an entry");
    }

    #[test]
    fn counting_everything() {
        let mut bag = Inventory::new();
        add_item(&mut bag, "mana potion", 2);
        add_item(&mut bag, "scroll", 5);
        assert_eq!(total_items(&bag), 7);
    }

    #[test]
    fn the_shopkeeper_reads_alphabetically() {
        let mut bag = Inventory::new();
        add_item(&mut bag, "scroll", 5);
        add_item(&mut bag, "elixir", 1);
        add_item(&mut bag, "mana potion", 2);
        assert_eq!(
            sorted_items(&bag),
            vec![
                ("elixir".to_string(), 1),
                ("mana potion".to_string(), 2),
                ("scroll".to_string(), 5),
            ]
        );
    }

    #[test]
    fn counting_words_in_an_incantation() {
        let freq = word_frequency("Ignis ignis MAGNA\n ignis");
        assert_eq!(freq.get("ignis"), Some(&3));
        assert_eq!(freq.get("magna"), Some(&1));
        assert_eq!(freq.len(), 2);
    }
}
