// Integration tests: this file is compiled as a SEPARATE crate that depends
// on `ch06_04_integration_doc_tests`. It only sees the public API.

use crate::{Duelist, duel};

#[test]
fn armor_absorbs_damage() {
    assert_eq!(damage_after_armor(30, 10), 20);
}

#[test]
fn the_first_striker_can_win() {
    let mut tiberius = Duelist::new("Tiberius", 100);
    let mut goblin = Duelist::new("Goblin", 30);
    assert_eq!(duel(&mut tiberius, &mut goblin, 45, 10), "Tiberius");
    assert!(goblin.is_defeated());
}

// TODO: at least 3 more tests. Ideas: HP never goes below 0, a hit always
// deals at least 1 damage, the second striker can win, `with_armor`…
