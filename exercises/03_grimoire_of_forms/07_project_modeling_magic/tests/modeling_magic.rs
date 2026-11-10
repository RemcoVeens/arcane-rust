use ch03_07_project_modeling_magic::*;

fn goblin() -> Creature {
    Creature::new("Frost Goblin", 30, Element::Frost, 20)
}

fn imp() -> Creature {
    Creature::new("Fire Imp", 60, Element::Fire, 35)
}

// ── Phase 1: forms ──────────────────────────────────────────────────

#[test]
fn strong_elements_double_damage() {
    assert_eq!(Element::Fire.multiplier(Element::Frost), 2.0);
    assert_eq!(Element::Frost.multiplier(Element::Arcane), 2.0);
    assert_eq!(Element::Arcane.multiplier(Element::Fire), 2.0);
}

#[test]
fn reversed_and_neutral_elements() {
    assert_eq!(Element::Frost.multiplier(Element::Fire), 0.5);
    assert_eq!(Element::Fire.multiplier(Element::Arcane), 0.5);
    assert_eq!(Element::Arcane.multiplier(Element::Frost), 0.5);
    assert_eq!(Element::Fire.multiplier(Element::Fire), 1.0);
    assert_eq!(Element::Life.multiplier(Element::Arcane), 1.0);
}

#[test]
fn describing_spells() {
    assert_eq!(STARTING_SPELLS[0].describe(), "Fireball (Fire) — mana 30, power 45, level 1");
    assert_eq!(TOWER_SCROLLS[1].describe(), "Meteor (Fire) — mana 90, power 150, level 10");
}

#[test]
fn creatures_are_born_alive() {
    let goblin = goblin();
    assert_eq!(goblin.name, "Frost Goblin");
    assert_eq!((goblin.hp, goblin.element, goblin.xp), (30, Element::Frost, 20));
    assert!(goblin.is_alive());
}

#[test]
fn taking_hits() {
    let mut imp = imp();
    assert_eq!(imp.take_hit(35, Element::Frost), 17, "35 × 0.5 = 17.5, rounded down");
    assert_eq!(imp.hp, 43);
    assert_eq!(imp.take_hit(60, Element::Arcane), 120, "super effective!");
    assert_eq!(imp.hp, 0, "HP never below zero");
    assert!(!imp.is_alive());
}

// ── Phase 2: the mage ───────────────────────────────────────────────

#[test]
fn a_successful_cast() {
    let mut mage = Mage::new("Tiberius", 7);
    let mut imp = imp();
    let outcome = mage.cast("FROSTBOLT", &mut imp);
    assert_eq!(outcome, CastOutcome::Hit { spell: "Frostbolt", damage: 17, defeated: false });
    assert_eq!(mage.mana, 75);
    assert_eq!(imp.hp, 43);
}

#[test]
fn a_defeating_cast() {
    let mut mage = Mage::new("Tiberius", 7);
    let mut goblin = goblin();
    let outcome = mage.cast("fireball", &mut goblin);
    assert_eq!(outcome, CastOutcome::Hit { spell: "Fireball", damage: 90, defeated: true });
}

#[test]
fn casting_unknown_spells() {
    let mut mage = Mage::new("Tiberius", 7);
    let mut goblin = goblin();
    assert_eq!(mage.cast("Arcane Blast", &mut goblin), CastOutcome::UnknownSpell);
    assert_eq!((mage.mana, goblin.hp), (MAX_MANA, 30));
}

#[test]
fn casting_above_your_level() {
    let mut mage = Mage::new("Tiberius", 7);
    mage.read_scroll();
    mage.read_scroll();
    let mut goblin = goblin();
    assert_eq!(mage.cast("Meteor", &mut goblin), CastOutcome::LevelTooLow { required: 10 });
    assert_eq!((mage.mana, goblin.hp), (MAX_MANA, 30));
}

#[test]
fn casting_without_mana() {
    let mut mage = Mage::new("Tiberius", 7);
    mage.mana = 20;
    let mut goblin = goblin();
    assert_eq!(
        mage.cast("Fireball", &mut goblin),
        CastOutcome::NotEnoughMana { needed: 30, available: 20 }
    );
    assert_eq!((mage.mana, goblin.hp), (20, 30));
}

// ── Phase 3: commands & the game ────────────────────────────────────

#[test]
fn parsing_simple_commands() {
    assert_eq!(Command::parse("list\n"), Command::List);
    assert_eq!(Command::parse("  REST "), Command::Rest);
    assert_eq!(Command::parse("potion"), Command::Potion);
    assert_eq!(Command::parse("Read"), Command::Read);
    assert_eq!(Command::parse("look"), Command::Look);
    assert_eq!(Command::parse("help"), Command::Help);
    assert_eq!(Command::parse("quit"), Command::Quit);
}

#[test]
fn parsing_cast_and_unknown_commands() {
    assert_eq!(Command::parse("CAST arcane blast\n"), Command::Cast(String::from("arcane blast")));
    assert_eq!(Command::parse("cast"), Command::Unknown(String::from("cast")));
    assert_eq!(Command::parse(" dance wildly "), Command::Unknown(String::from("dance wildly")));
}

#[test]
fn looking_around() {
    let mut game = Game::start();
    let msg = game.handle(Command::Look);
    assert!(msg.contains("Frost Goblin") && msg.contains("30"), "got: {msg}");
    game.foes.clear();
    assert!(game.handle(Command::Look).contains("quiet"));
}

#[test]
fn defeating_a_foe_awards_xp() {
    let mut game = Game::start();
    let msg = game.handle(Command::Cast(String::from("fireball")));
    assert!(msg.contains("Fireball") && msg.contains("90"), "got: {msg}");
    assert!(msg.contains("defeated"), "got: {msg}");
    assert_eq!(game.mage.xp, 20);
    assert_eq!(game.foes.len(), 3, "the goblin is gone");
    assert_eq!(game.foes[0].name, "Fire Imp");
}

#[test]
fn a_hit_that_does_not_defeat() {
    let mut game = Game::start();
    game.foes.remove(0);
    let msg = game.handle(Command::Cast(String::from("Frostbolt")));
    assert!(msg.contains("17"), "got: {msg}");
    assert!(!msg.contains("defeated"), "got: {msg}");
    assert_eq!((game.foes.len(), game.foes[0].hp, game.mage.xp), (3, 43, 0));
}

#[test]
fn failed_casts_explain_why() {
    let mut game = Game::start();
    assert!(game.handle(Command::Cast(String::from("Meteor"))).contains("Unknown spell"));
    game.handle(Command::Read);
    game.handle(Command::Read);
    assert!(game.handle(Command::Cast(String::from("Meteor"))).contains("level"));
    game.mage.mana = 5;
    assert!(game.handle(Command::Cast(String::from("Heal"))).contains("Not enough mana"));
    game.foes.clear();
    assert!(game.handle(Command::Cast(String::from("Heal"))).contains("nothing to fight"));
}

#[test]
fn reading_resting_and_potions() {
    let mut game = Game::start();
    assert!(game.handle(Command::Read).contains("Arcane Blast"));
    assert!(game.handle(Command::Read).contains("Meteor"));
    assert!(game.handle(Command::Read).contains("No scrolls"));
    assert_eq!(game.handle(Command::List), game.mage.spell_list());
    game.mage.mana = 10;
    assert!(game.handle(Command::Potion).contains("60"));
    assert!(game.handle(Command::Rest).contains("100"));
}

#[test]
fn help_unknown_and_quit() {
    let mut game = Game::start();
    let help = game.handle(Command::Help);
    for cmd in ["list", "cast", "rest", "potion", "read", "look", "help", "quit"] {
        assert!(help.contains(cmd), "help should mention '{cmd}'");
    }
    assert!(game.handle(Command::Unknown(String::from("dance"))).contains("Unknown command"));
    assert!(game.running);
    game.handle(Command::Quit);
    assert!(!game.running);
}
