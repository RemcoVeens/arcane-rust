// ⚔️ Quest 5.1 — Lifetime Elision: The Unspoken Components
// 📖 Read: book/src/05_temporal_loom/01_elision.md
//
// ❌ This file does not compile yet.
// Most functions here need NO lifetime annotations — the elision rules fill
// them in for you. Exactly ONE signature needs explicit help. Find it.
// Then replace every `todo!()`.

/// The first word of an incantation (split on whitespace).
/// An empty (or all-whitespace) incantation gives `""`.
///   "Ignis Maxima Totalis" → "Ignis"
pub fn first_word(incantation: &str) -> &str {
    todo!()
}

/// Strip all leading sigils (`'✦'`) and surrounding whitespace from a name.
///   "✦✦ Fireball " → "Fireball"
pub fn strip_sigils(name: &str) -> &str {
    todo!()
}

/// Choose between two spell names: the `primary` if `prefer_primary`,
/// otherwise the `backup`.
pub fn choose(primary: &str, backup: &str, prefer_primary: bool) -> &str {
    if prefer_primary { primary } else { backup }
}

/// A mage's grimoire: a title and a list of spell names.
pub struct Grimoire {
    pub title: String,
    pub spells: Vec<String>,
}

impl Grimoire {
    /// The title of the grimoire.
    pub fn title(&self) -> &str {
        todo!()
    }

    /// The longest spell name, or `None` when the grimoire is empty.
    /// On a tie, the first one wins.
    pub fn longest_spell(&self) -> Option<&str> {
        todo!()
    }

    /// The first spell whose name starts with `prefix`.
    ///
    /// Note: `prefix` is a reference too — but the result borrows from
    /// `self`, and elision rule 3 already says exactly that.
    pub fn find(&self, prefix: &str) -> Option<&str> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tiberius_grimoire() -> Grimoire {
        Grimoire {
            title: String::from("Tiberius' Pocket Grimoire"),
            spells: vec![
                String::from("Fireball"),
                String::from("Frostbolt"),
                String::from("Arcane Blast"),
                String::from("Heal"),
            ],
        }
    }

    #[test]
    fn first_words() {
        assert_eq!(first_word("Ignis Maxima Totalis"), "Ignis");
        assert_eq!(first_word("   Glacius"), "Glacius");
        assert_eq!(first_word("   "), "");
    }

    #[test]
    fn sigils_are_stripped() {
        assert_eq!(strip_sigils("✦✦ Fireball "), "Fireball");
        assert_eq!(strip_sigils("Heal"), "Heal");
    }

    #[test]
    fn choosing_a_spell() {
        let primary = String::from("Meteor");
        let backup = String::from("Fireball");
        assert_eq!(choose(&primary, &backup, true), "Meteor");
        assert_eq!(choose(&primary, &backup, false), "Fireball");
    }

    #[test]
    fn grimoire_title() {
        assert_eq!(tiberius_grimoire().title(), "Tiberius' Pocket Grimoire");
    }

    #[test]
    fn longest_spell_in_the_grimoire() {
        assert_eq!(tiberius_grimoire().longest_spell(), Some("Arcane Blast"));
        let empty = Grimoire { title: String::from("Blank"), spells: vec![] };
        assert_eq!(empty.longest_spell(), None);
    }

    #[test]
    fn finding_outlives_the_prefix() {
        let grimoire = tiberius_grimoire();
        let found;
        {
            let prefix = String::from("Fro");
            found = grimoire.find(&prefix);
        } // the prefix is gone, the found spell still borrows from `grimoire`
        assert_eq!(found, Some("Frostbolt"));
        assert_eq!(grimoire.find("Summon"), None);
    }
}
