use ch04_10_project_combat::*;

/// A battle against a single enemy.
fn duel(enemy: Box<dyn Creature>) -> Battle {
    Battle::new(vec![enemy])
}

// ── Phase 1: creatures ───────────────────────────────────────────────

#[test]
fn creatures_know_whether_they_live() {
    let mut goblin = Goblin::new();
    assert!(goblin.is_alive());
    goblin.take_damage(100);
    assert_eq!(goblin.hp(), 0);
    assert!(!goblin.is_alive(), "💀 a goblin with 0 hp is not alive");
}

#[test]
fn the_ice_wraith() {
    let mut wraith = IceWraith::new();
    assert_eq!((wraith.name(), wraith.hp(), wraith.attack_power()), ("Ice Wraith", 50, 8));
    assert_eq!(wraith.weakness(), Some(Element::Fire));
    assert_eq!(wraith.resistance(), Some(Element::Ice));
    wraith.take_damage(80);
    assert_eq!(wraith.hp(), 0, "hp never goes below 0");
}

#[test]
fn the_dragon() {
    let dragon = Dragon::new();
    assert_eq!((dragon.name(), dragon.hp(), dragon.attack_power()), ("Dragon", 150, 12));
    assert_eq!(dragon.weakness(), Some(Element::Ice));
    assert_eq!(dragon.resistance(), Some(Element::Fire));
}

#[test]
fn striking_a_neutral_target() {
    let mut goblin = Goblin::new();
    assert_eq!(strike(&mut goblin, 20, Element::Fire), 20);
    assert_eq!(goblin.hp(), 10);
}

#[test]
fn weaknesses_double_and_resistances_halve() {
    let mut horde: Vec<Box<dyn Creature>> = vec![Box::new(IceWraith::new()), Box::new(Dragon::new())];
    assert_eq!(strike(horde[0].as_mut(), 20, Element::Fire), 40, "🔥 wraiths fear fire");
    assert_eq!(horde[0].hp(), 10);
    assert_eq!(strike(horde[1].as_mut(), 45, Element::Fire), 22, "🐉 dragons shrug off fire (45 / 2)");
    assert_eq!(horde[1].hp(), 128);
    assert_eq!(strike(horde[1].as_mut(), 35, Element::Ice), 70);
}

// ── Phase 2: spellbook & inventory ───────────────────────────────────

#[test]
fn finding_spells_ignoring_case() {
    let book = Spellbook::new();
    let blast = book.find("arcane BLAST").expect("case should not matter");
    assert_eq!(blast.name, "Arcane Blast");
    assert!(book.find("Summon Pizza").is_none());
}

#[test]
fn learning_new_spells() {
    let mut book = Spellbook::new();
    assert_eq!(book.learn(Spell::new("Lightning", 35, 50, 3, Element::Arcane)), Ok(()));
    assert_eq!(book.names().len(), 6);
    assert!(book.find("lightning").is_some());
}

#[test]
fn learning_a_spell_twice() {
    let mut book = Spellbook::new();
    let again = Spell::new("FIREBALL", 1, 999, 1, Element::Fire);
    assert_eq!(book.learn(again), Err(CastError::AlreadyKnown("FIREBALL".to_string())));
    assert_eq!(book.names().len(), 5);
}

#[test]
fn inventory_stacks_and_counts() {
    let mut bag = Inventory::new();
    bag.add("mana potion", 2);
    bag.add("mana potion", 1);
    assert_eq!(bag.count("mana potion"), 3);
    assert_eq!(bag.count("dragon scale"), 0);
}

#[test]
fn inventory_runs_out() {
    let mut bag = Inventory::new();
    bag.add("health potion", 1);
    assert_eq!(bag.take("health potion"), Ok(()));
    assert_eq!(bag.take("health potion"), Err(CastError::OutOfItem("health potion".to_string())));
    assert_eq!(bag.listing(), "", "empty piles must be removed");
}

// ── Phase 3: casting with Result ─────────────────────────────────────

#[test]
fn error_messages() {
    let cases = [
        (CastError::UnknownSpell("pizza".to_string()), "Unknown spell 'pizza'"),
        (CastError::LevelTooLow { needed: 10, have: 7 }, "Requires level 10, you are level 7"),
        (CastError::NotEnoughMana { needed: 40, have: 10 }, "Not enough mana: need 40, have 10"),
        (CastError::InvalidTarget(4), "There is no living enemy #5"),
        (CastError::OutOfItem("mana potion".to_string()), "You have no mana potion left"),
        (CastError::NotDrinkable("lava".to_string()), "You can't drink lava"),
        (CastError::UnknownCommand("dance".to_string()), "Unknown command 'dance' (type 'help')"),
        (CastError::AlreadyKnown("Heal".to_string()), "You already know Heal"),
    ];
    for (error, message) in cases {
        assert_eq!(error.to_string(), message);
    }
}

#[test]
fn a_successful_cast() {
    let mut battle = duel(Box::new(Goblin::new()));
    let msg = battle.cast("fireball", 0).expect("Tiberius can cast Fireball");
    assert!(msg.contains("Fireball"), "message should name the spell: {msg}");
    assert_eq!(battle.mage.mana, 70);
    assert_eq!(battle.enemies[0].hp(), 0);
}

#[test]
fn casting_checks_happen_in_order() {
    let mut battle = duel(Box::new(Goblin::new()));
    assert_eq!(battle.cast("pizza", 0), Err(CastError::UnknownSpell("pizza".to_string())));
    assert_eq!(battle.cast("meteor", 0), Err(CastError::LevelTooLow { needed: 10, have: 7 }));
    battle.mage.mana = 10;
    assert_eq!(battle.cast("arcane blast", 7), Err(CastError::NotEnoughMana { needed: 40, have: 10 }));
}

#[test]
fn targets_must_exist_and_live() {
    let mut battle = duel(Box::new(Goblin { hp: 0 }));
    assert_eq!(battle.cast("frostbolt", 0), Err(CastError::InvalidTarget(0)), "the goblin is already dead");
    assert_eq!(battle.cast("frostbolt", 3), Err(CastError::InvalidTarget(3)));
    assert_eq!(battle.mage.mana, MAX_MANA, "failed casts cost no mana");
}

#[test]
fn healing_needs_no_target() {
    let mut battle = duel(Box::new(Goblin { hp: 0 }));
    battle.mage.hp = 50;
    let msg = battle.cast("heal", 99).expect("heal ignores the target");
    assert!(msg.contains("Heal"), "got: {msg}");
    assert_eq!((battle.mage.hp, battle.mage.mana), (80, 80));
    battle.cast("heal", 0).unwrap();
    assert_eq!(battle.mage.hp, MAX_HP, "healing is capped");
}

#[test]
fn drinking_potions() {
    let mut battle = Battle::standard();
    battle.mage.mana = 10;
    battle.drink("mana potion").expect("we carry two");
    assert_eq!(battle.mage.mana, 60);
    assert_eq!(battle.inventory.count("mana potion"), 1);
    assert_eq!(battle.drink("lava"), Err(CastError::NotDrinkable("lava".to_string())));
    battle.drink("health potion").unwrap();
    assert_eq!(battle.drink("health potion"), Err(CastError::OutOfItem("health potion".to_string())));
}

#[test]
fn the_enemies_strike() {
    let mut battle = Battle::standard();
    battle.enemies[0].take_damage(100); // the goblin is dead
    assert_eq!(battle.enemies_turn(), 8 + 12, "only living enemies attack");
    assert_eq!(battle.mage.hp, 80);
    battle.mage.hp = 5;
    battle.enemies_turn();
    assert_eq!(battle.mage.hp, 0, "hp never goes below 0");
}

#[test]
fn victory_and_defeat() {
    let mut battle = duel(Box::new(Goblin::new()));
    assert_eq!(battle.outcome(), None);
    battle.enemies[0].take_damage(30);
    assert_eq!(battle.outcome(), Some(Outcome::Victory));
    battle.mage.hp = 0;
    assert_eq!(battle.outcome(), Some(Outcome::Defeat), "defeat is checked first");
}

// ── Phase 4: the full turn ───────────────────────────────────────────

#[test]
fn a_cast_turn_lets_the_enemies_strike_back() {
    let mut battle = Battle::standard();
    let msg = battle.take_turn("cast 2 fireball\n").expect("a valid turn");
    assert_eq!(battle.enemies[1].hp(), 0, "fireball doubles against the wraith (90 ≥ 50)");
    assert!(msg.contains("strike back"), "got: {msg}");
    assert_eq!(battle.mage.hp, MAX_HP - 5 - 12, "goblin and dragon attack, the wraith is dead");
}

#[test]
fn a_failed_turn_costs_nothing() {
    let mut battle = Battle::standard();
    assert_eq!(battle.take_turn("cast 1 meteor"), Err(CastError::LevelTooLow { needed: 10, have: 7 }));
    assert_eq!(battle.take_turn("  dance  "), Err(CastError::UnknownCommand("dance".to_string())));
    assert_eq!(battle.mage.hp, MAX_HP, "the enemies only act after a successful action");
}

#[test]
fn the_final_blow_ends_the_battle() {
    let mut battle = duel(Box::new(Goblin::new()));
    let msg = battle.take_turn("cast 1 fireball").unwrap();
    assert!(!msg.contains("strike back"), "a dead goblin can't strike back: {msg}");
    assert_eq!(battle.mage.hp, MAX_HP);
    assert_eq!(battle.outcome(), Some(Outcome::Victory));
}

#[test]
fn info_commands_are_free() {
    let mut battle = Battle::standard();
    assert_eq!(battle.take_turn("help"), Ok(HELP.to_string()));
    let status = battle.take_turn("STATUS").unwrap();
    assert!(status.contains("Ice Wraith"), "got: {status}");
    assert_eq!(battle.mage.hp, MAX_HP);
}
