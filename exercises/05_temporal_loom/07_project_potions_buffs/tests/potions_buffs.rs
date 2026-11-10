use ch05_07_project_potions_buffs::*;

fn wounded_mage() -> Mage {
    let mut mage = Mage::new("Tiberius", 7);
    mage.hp = 40;
    mage.mana = 10;
    mage
}

// ── Phase 1: potions ────────────────────────────────────────────────

#[test]
fn custom_potions_run_their_effect_once() {
    let mut mage = Mage::new("Tiberius", 7);
    let potion = Potion::new("Elixir of Wisdom", |m: &mut Mage| m.level += 1);
    assert_eq!(potion.name, "Elixir of Wisdom");
    potion.drink(&mut mage);
    assert_eq!(mage.level, 8);
}

#[test]
fn potions_can_carry_captured_treasure() {
    // An FnOnce may move captured values out — a Fn could not.
    let secret_spell = Spell::new("Starfall", 50, 80, 3);
    let scroll = Potion::new("Scroll Draught", move |m: &mut Mage| m.spells.push(secret_spell));
    let mut mage = Mage::new("Tiberius", 7);
    scroll.drink(&mut mage);
    assert!(mage.find_spell("starfall").is_ok(), "the potion should teach Starfall");
}

#[test]
fn healing_and_mana_potions_are_capped() {
    let mut mage = wounded_mage();
    healing_draught(50).drink(&mut mage);
    assert_eq!(mage.hp, 90);
    healing_draught(50).drink(&mut mage);
    assert_eq!(mage.hp, 100, "❤️ HP never exceeds max_hp");

    mana_elixir(60).drink(&mut mage);
    assert_eq!(mage.mana, 70);
    mana_elixir(60).drink(&mut mage);
    assert_eq!(mage.mana, MAX_MANA);
}

#[test]
fn drinking_from_the_satchel() {
    let mut mage = wounded_mage();
    let mut satchel = Satchel::new();
    satchel.add(healing_draught(30));
    satchel.add(mana_elixir(50));
    satchel.add(healing_draught(10));
    assert_eq!(satchel.names(), vec!["Healing Draught", "Mana Elixir", "Healing Draught"]);

    assert!(satchel.drink("healing draught", &mut mage));
    assert_eq!(mage.hp, 70, "the FIRST Healing Draught (30) is drunk");
    assert_eq!(satchel.names(), vec!["Mana Elixir", "Healing Draught"]);

    assert!(!satchel.drink("Potion of Flight", &mut mage));
    assert_eq!(satchel.potions.len(), 2);
}

// ── Phase 2: buffs & effect chains ──────────────────────────────────

#[test]
fn buffs_modify_power() {
    let buff = Buff::new("Moonlight", 3, |p| p + 1);
    assert_eq!((buff.name.as_str(), buff.turns_left), ("Moonlight", 3));
    assert_eq!(buff.apply(10), 11);

    assert_eq!(empower(50, 2).apply(45), 67);
    assert_eq!(empower(100, 2).name, "Empower");
    assert_eq!(arcane_focus(15, 2).apply(45), 60);
    assert_eq!(arcane_focus(15, 2).name, "Arcane Focus");
}

#[test]
fn buffs_chain_in_order() {
    let buffs = vec![arcane_focus(5, 3), empower(100, 3)];
    assert_eq!(chain_power(&buffs, 45), 100, "(45 + 5) * 2");
    let reversed = vec![empower(100, 3), arcane_focus(5, 3)];
    assert_eq!(chain_power(&reversed, 45), 95, "45 * 2 + 5");
    assert_eq!(chain_power(&[], 45), 45);
}

#[test]
fn casting_through_a_modifier() {
    let mut mage = Mage::new("Tiberius", 7);
    let mut goblin = Goblin { hp: 100 };
    let buffs = vec![empower(100, 2)];

    let dealt = mage.cast_with("Fireball", &mut goblin, |p| chain_power(&buffs, p));
    assert_eq!(dealt, Ok(90));
    assert_eq!(goblin.hp, 10);
    assert_eq!(mage.mana, 70);
}

#[test]
fn modified_casts_still_follow_the_rules() {
    let mut mage = Mage::new("Tiberius", 7);
    let mut wraith = Wraith { hp: 100 };
    assert_eq!(
        mage.cast_with("Meteor", &mut wraith, |p| p * 2),
        Err(CastError::LevelTooLow { required: 10, current: 7 })
    );
    mage.mana = 5;
    assert_eq!(
        mage.cast_with("Heal", &mut wraith, |p| p),
        Err(CastError::NotEnoughMana { needed: 20, available: 5 })
    );
    assert_eq!(wraith.hp, 100, "failed casts don't hurt anyone");
}

// ── Phase 3: timed effects ──────────────────────────────────────────

#[test]
fn buffs_expire() {
    let mut buffs = vec![empower(50, 1), arcane_focus(10, 2)];
    tick_buffs(&mut buffs);
    assert_eq!(buffs.len(), 1, "Empower had only one turn left");
    assert_eq!(buffs[0].name, "Arcane Focus");
    assert_eq!(buffs[0].turns_left, 1);
    tick_buffs(&mut buffs);
    assert!(buffs.is_empty());
}

#[test]
fn burns_deal_damage_every_turn() {
    let mut dragon = Dragon { hp: 100 };
    let mut burns = vec![
        Burn { source: String::from("Fireball"), damage_per_turn: 10, turns_left: 2 },
        Burn { source: String::from("Dragonfire"), damage_per_turn: 5, turns_left: 1 },
    ];
    assert_eq!(tick_burns(&mut burns, &mut dragon), 15);
    assert_eq!(dragon.hp, 85);
    assert_eq!(burns.len(), 1, "the 1-turn burn is gone");

    assert_eq!(tick_burns(&mut burns, &mut dragon), 10);
    assert_eq!(dragon.hp, 75);
    assert!(burns.is_empty());
    assert_eq!(tick_burns(&mut burns, &mut dragon), 0);
}

#[test]
fn burns_respect_creature_resistances() {
    let mut wraith = Wraith { hp: 50 };
    let mut burns = vec![Burn { source: String::from("Fireball"), damage_per_turn: 20, turns_left: 3 }];
    tick_burns(&mut burns, &mut wraith);
    assert_eq!(wraith.hp, 40, "👻 wraiths take half damage, even from burns");
    assert_eq!(burns[0].turns_left, 2);
}

// ── Phase 4: battle statistics ──────────────────────────────────────

fn log() -> Vec<CastRecord> {
    let record = |turn, spell: &str, damage| CastRecord { turn, spell: spell.to_string(), damage };
    vec![
        record(1, "Frostbolt", 35),
        record(2, "Fireball", 45),
        record(3, "Frostbolt", 70),
        record(4, "Fireball", 90),
        record(5, "Heal", 0),
    ]
}

#[test]
fn totals_and_counts() {
    let log = log();
    assert_eq!(total_damage(&log), 240);
    let counts = spell_counts(&log); // borrows from `log`
    assert_eq!(counts.len(), 3);
    assert_eq!(counts["Fireball"], 2);
    assert_eq!(counts["Heal"], 1);
}

#[test]
fn favourite_spells() {
    let log = log();
    assert_eq!(favourite_spell(&log), Some("Fireball"), "on a tie, alphabetical order decides");
    assert_eq!(favourite_spell(&log[2..]), Some("Fireball"));
    assert_eq!(favourite_spell(&log[..3]), Some("Frostbolt"));
    assert_eq!(favourite_spell(&[]), None);
}

#[test]
fn the_battle_report() {
    let log = log();
    let report = battle_report(&log[..2]);
    assert_eq!(report, "Turn 1: Frostbolt for 35\nTurn 2: Fireball for 45");
    assert_eq!(battle_report(&[]), "");
}

#[test]
fn surveying_the_battlefield() {
    let creatures: Vec<Box<dyn Creature>> = vec![
        Box::new(Goblin { hp: 0 }),
        Box::new(Wraith { hp: 30 }),
        Box::new(Dragon { hp: 120 }),
        Box::new(Goblin { hp: 30 }),
    ];
    assert_eq!(living(&creatures), vec!["Wraith", "Dragon", "Goblin"]);
    assert_eq!(weakest_living(&creatures), Some(1), "tie → the first one");

    let fallen: Vec<Box<dyn Creature>> = vec![Box::new(Goblin { hp: 0 })];
    assert!(living(&fallen).is_empty());
    assert_eq!(weakest_living(&fallen), None);
}
