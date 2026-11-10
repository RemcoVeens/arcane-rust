use ch08_06_project_complete_saga::*;
use std::rc::Rc;

fn party_of_two() -> (Party, Hero, Hero) {
    let mut party = Party::new(2);
    let tiberius = party.recruit(Mage::tiberius());
    let morgana = party.recruit(Mage::new("Morgana", 3));
    (party, tiberius, morgana)
}

// ── Phase 1: the grimoire macro ──────────────────────────────────────

#[test]
fn the_grimoire_holds_five_spells() {
    let book = grimoire();
    assert_eq!(book.len(), 5);
    assert_eq!(book[0], Spell::new("Fireball", Element::Fire, 30, 45, 1));
    assert_eq!(book[4], Spell::new("Meteor", Element::Fire, 90, 150, 10));
}

#[test]
fn spell_names_may_contain_spaces() {
    let blast = find_spell("arcane blast").unwrap();
    assert_eq!(blast, Spell::new("Arcane Blast", Element::Arcane, 40, 60, 5));
}

#[test]
fn the_macro_works_outside_the_crate() {
    let base = 5;
    let custom = grimoire! {
        "Spark"      => Arcane, cost 1, power base * 2, level 1;
        "Holy Light" => Holy,   cost 15, power 20, level 3
    };
    assert_eq!(
        custom,
        vec![
            Spell::new("Spark", Element::Arcane, 1, 10, 1),
            Spell::new("Holy Light", Element::Holy, 15, 20, 3),
        ]
    );
    let nothing: Vec<Spell> = grimoire! {};
    assert!(nothing.is_empty());
}

// ── Phase 2: the party ───────────────────────────────────────────────

#[test]
fn recruits_are_shared_handles() {
    let (party, tiberius, _) = party_of_two();
    tiberius.borrow_mut().mana = 7;
    assert_eq!(party.members().len(), 2);
    assert_eq!(party.members()[0].borrow().mana, 7, "🤝 the party and the caller share one Mage");
    assert_eq!(Rc::strong_count(&tiberius), 2, "one handle in the party, one returned");
}

#[test]
fn the_stash_is_shared_too() {
    let (party, _, _) = party_of_two();
    party.stash().borrow_mut().gold += 99;
    assert_eq!(party.stash().borrow().gold, 99);
    assert_eq!(party.stash().borrow().potions, 2);
}

#[test]
fn the_leader_is_the_first_hero_standing() {
    let (party, tiberius, morgana) = party_of_two();
    assert!(Rc::ptr_eq(&party.leader().unwrap(), &tiberius));
    tiberius.borrow_mut().hp = 0;
    assert!(Rc::ptr_eq(&party.leader().unwrap(), &morgana), "Tiberius has fallen — Morgana leads");
    morgana.borrow_mut().hp = 0;
    assert!(party.leader().is_none());
}

#[test]
fn potions_heal_but_run_out() {
    let (party, tiberius, _) = party_of_two();
    assert_eq!(party.total_hp(), 200);
    tiberius.borrow_mut().hp = 30;
    assert!(party.drink_potion(&tiberius));
    assert_eq!(tiberius.borrow().hp, 80);
    assert!(party.drink_potion(&tiberius));
    assert_eq!(tiberius.borrow().hp, 100, "potions never overheal");
    assert!(!party.drink_potion(&tiberius), "🧪 the stash is out of potions");
    assert_eq!(party.stash().borrow().potions, 0);
}

#[test]
fn resting_restores_everyone() {
    let (party, tiberius, morgana) = party_of_two();
    tiberius.borrow_mut().hp = 0;
    tiberius.borrow_mut().mana = 5;
    morgana.borrow_mut().hp = 12;
    party.rest();
    assert_eq!((tiberius.borrow().hp, tiberius.borrow().mana), (100, 100));
    assert_eq!(morgana.borrow().hp, 100);
}

// ── Phase 3: bounties ────────────────────────────────────────────────

#[test]
fn creatures_carry_a_bounty() {
    let loot: Loot = Creature::wraith().bounty();
    assert_eq!(loot, Loot { gold: 25, trophy: "Wraith Trophy".to_string() });
}

#[test]
fn total_gold_works_for_any_loot_bounty() {
    assert_eq!(total_gold(&[Creature::goblin(), Creature::dragon()]), 15 + 150);
    assert_eq!(total_gold(&[Chest { gold: 7 }, Chest { gold: 3 }]), 10);
    assert_eq!(total_gold::<Chest>(&[]), 0);
}

#[test]
fn only_the_fallen_pay_out() {
    let mut goblin = Creature::goblin();
    goblin.hp = 0;
    let (gold, trophies) = spoils(&[goblin, Creature::wraith()]);
    assert_eq!(gold, 15);
    assert_eq!(trophies, vec!["Goblin Trophy"]);
}

// ── Phase 4: the saga ────────────────────────────────────────────────

#[test]
fn a_victorious_encounter() {
    let (party, tiberius, _) = party_of_two();
    let mut saga = Saga::new(party);
    let outcome = saga.encounter(
        vec![Creature::goblin(), Creature::wraith()],
        &["Fireball", "Frostbolt", "Fireball", "Fireball"],
    );
    assert_eq!(outcome, Ok(Outcome::Victory));
    assert_eq!(
        (tiberius.borrow().hp, tiberius.borrow().mana),
        (56, 15),
        "⚔️ the battle's wounds are written back into the hero"
    );
    let stash = saga.party.stash();
    assert_eq!(stash.borrow().gold, 40);
    assert_eq!(stash.borrow().trophies, vec!["Goblin Trophy", "Wraith Trophy"]);
    assert_eq!(saga.chronicle, vec!["Tiberius vs Goblin, Wraith: Victory in 3 rounds"]);
}

#[test]
fn a_retreat_brings_no_spoils() {
    let (party, _, _) = party_of_two();
    let mut saga = Saga::new(party);
    let dummy = Creature::new("Training Dummy", 500, 1, None);
    let outcome = saga.encounter(vec![Creature::goblin(), dummy], &["Fireball"]);
    assert_eq!(outcome, Ok(Outcome::Retreat));
    assert_eq!(saga.party.stash().borrow().gold, 0, "the goblin fell, but you fled: no loot");
    assert_eq!(saga.chronicle, vec!["Tiberius vs Goblin, Training Dummy: Retreat in 1 rounds"]);
}

#[test]
fn when_the_leader_falls_the_next_hero_steps_up() {
    let (party, tiberius, morgana) = party_of_two();
    let mut saga = Saga::new(party);
    assert_eq!(saga.encounter(vec![Creature::dragon()], &["Fireball"; 5]), Ok(Outcome::Defeat));
    assert_eq!(tiberius.borrow().hp, 0);
    assert!(Rc::ptr_eq(&saga.party.leader().unwrap(), &morgana));

    assert_eq!(saga.encounter(vec![Creature::goblin()], &["Frostbolt"]), Ok(Outcome::Victory));
    assert_eq!(morgana.borrow().mana, 75, "Morgana fought this one");
    assert_eq!(saga.chronicle.len(), 2);
    assert!(saga.chronicle[1].starts_with("Morgana vs Goblin"), "got: {}", saga.chronicle[1]);
}

#[test]
fn no_heroes_no_saga() {
    let (party, _, _) = party_of_two();
    let mut saga = Saga::new(party);
    saga.encounter(vec![Creature::dragon()], &["Fireball"; 5]).unwrap();
    saga.encounter(vec![Creature::dragon()], &["Fireball"; 5]).unwrap();
    assert_eq!(saga.encounter(vec![Creature::goblin()], &["Fireball"]), Err(BattleError::NoHeroes));
    assert_eq!(saga.chronicle.len(), 2, "no battle, no chronicle line");
}

#[test]
fn rest_then_fight_again() {
    let (party, tiberius, _) = party_of_two();
    let mut saga = Saga::new(party);
    saga.encounter(vec![Creature::dragon()], &["Fireball"; 5]).unwrap();
    saga.party.rest();
    assert!(Rc::ptr_eq(&saga.party.leader().unwrap(), &tiberius), "rested and revived");
    assert_eq!(saga.encounter(vec![Creature::wraith()], &["Arcane Blast"]), Ok(Outcome::Victory));
    assert_eq!(tiberius.borrow().hp, 88);
}
