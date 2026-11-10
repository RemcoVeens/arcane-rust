// ⚔️ Quest 8.4 — Macros: The Art of Enchantment
// 📖 Read: book/src/08_final_incantation/04_macros.md
//
// Each macro below has a placeholder rule that matches anything and
// expands to `todo!()`. Replace each placeholder with real rules.
// (Tip: `cargo expand` is not installed here, but the compiler's errors
// point inside the macro expansion when something is off.)

/// A spell, as written in the DSL.
#[derive(Debug, Clone, PartialEq)]
pub struct Spell {
    pub name: &'static str,
    pub cost: u32,
    pub power: u32,
}

/// `spell!(Fireball => 30, 45)` expands to
/// `Spell { name: "Fireball", cost: 30, power: 45 }`.
///
/// The name is an identifier (`$name:ident`), turned into a string with
/// `stringify!`. Refer to the struct as `$crate::Spell` so the macro also
/// works from other crates.
#[macro_export]
macro_rules! spell {
    ($($anything:tt)*) => {
        todo!()
    };
}

/// A whole spellbook: entries separated by `;`, trailing `;` allowed.
///
/// ```text
/// spellbook! {
///     Fireball  => 30, 45;
///     Frostbolt => 25, 35;
/// }
/// ```
/// expands to `vec![spell!(Fireball => 30, 45), spell!(Frostbolt => 25, 35)]`
/// (so its type is `Vec<Spell>`). An empty `spellbook! {}` is an empty Vec.
#[macro_export]
macro_rules! spellbook {
    ($($anything:tt)*) => {
        todo!()
    };
}

/// Cast a spell from a book by its (identifier) name. Two forms:
///
/// - `cast!(book, Fireball)`      → `Option<u32>`: the spell's power, or `None`
/// - `cast!(book, Fireball x 3)`  → `Option<u32>`: power times 3
///
/// `book` is any expression that has `.iter()` over `Spell`s (e.g. a `Vec<Spell>`).
#[macro_export]
macro_rules! cast {
    // placeholder: the type annotation only helps the starter compile
    ($($anything:tt)*) => {{
        let placeholder: Option<u32> = todo!();
        placeholder
    }};
}

/// The largest of one or more `u32` powers, computed by a RECURSIVE macro:
/// - `max_power!(x)`          → `x`
/// - `max_power!(x, rest...)` → the larger of `x` and `max_power!(rest...)`
#[macro_export]
macro_rules! max_power {
    // placeholder: the type annotation only helps the starter compile
    ($($anything:tt)*) => {{
        let placeholder: u32 = todo!();
        placeholder
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    fn the_book() -> Vec<Spell> {
        spellbook! {
            Fireball   => 30, 45;
            Frostbolt  => 25, 35;
            ArcaneBlast => 40, 60;
            Heal       => 20, 0;
            Meteor     => 90, 150;
        }
    }

    #[test]
    fn spell_builds_a_spell() {
        let s: Spell = spell!(Fireball => 30, 45);
        assert_eq!(s, Spell { name: "Fireball", cost: 30, power: 45 });
    }

    #[test]
    fn spell_accepts_expressions() {
        let base = 10;
        let s: Spell = spell!(Meteor => base * 9, base * 15);
        assert_eq!((s.cost, s.power), (90, 150));
    }

    #[test]
    fn a_spellbook_from_the_dsl() {
        let book = the_book();
        assert_eq!(book.len(), 5);
        assert_eq!(book[2], Spell { name: "ArcaneBlast", cost: 40, power: 60 });
        assert_eq!(book[4].name, "Meteor");
    }

    #[test]
    fn spellbooks_without_a_trailing_semicolon_and_empty_ones() {
        let short: Vec<Spell> = spellbook! { Heal => 20, 0; Fireball => 30, 45 };
        assert_eq!(short.len(), 2);
        let empty: Vec<Spell> = spellbook! {};
        assert!(empty.is_empty());
    }

    #[test]
    fn casting_from_the_book() {
        let book = the_book();
        assert_eq!(cast!(book, Fireball), Some(45));
        assert_eq!(cast!(book, Fireball x 3), Some(135));
        assert_eq!(cast!(book, SummonPizza), None, "🍕 not in the book");
    }

    #[test]
    fn the_mightiest_power() {
        assert_eq!(max_power!(45), 45);
        assert_eq!(max_power!(45, 35), 45);
        assert_eq!(max_power!(45, 35, 60, 0, 150, 7), 150);
    }
}
