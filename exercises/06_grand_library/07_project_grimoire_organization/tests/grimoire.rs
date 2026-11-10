// These tests only see the PUBLIC API of the `grimoire` library — exactly
// like any other crate that depends on it. They pin down the module layout.

// ── Phase 1: the first shelves — spells & errors ─────────────────────

#[test]
fn spells_live_in_the_spells_module() {
    use grimoire::spells::{MAX_MANA, Spell, spellbook};

    assert_eq!(MAX_MANA, 100);
    let book = spellbook();
    assert_eq!(book.len(), 5);
    assert_eq!(book[0], Spell::new("Fireball", 30, 45, 1));
}

#[test]
fn errors_live_in_the_error_module() {
    use grimoire::error::CastError;

    let err = CastError::NotEnoughMana { needed: 90, available: 10 };
    assert_eq!(err.to_string(), "Not enough mana: need 90, have 10");
    let boxed: Box<dyn std::error::Error> = Box::new(CastError::UnknownSpell("Pizza".into()));
    assert!(boxed.to_string().contains("Pizza"));
}

// ── Phase 2: creatures & the mage ────────────────────────────────────

#[test]
fn creatures_live_in_the_creatures_module() {
    use grimoire::creatures::{Creature, Dragon, Goblin, Wraith};

    let mut horde: Vec<Box<dyn Creature>> =
        vec![Box::new(Goblin { hp: 40 }), Box::new(Wraith { hp: 40 }), Box::new(Dragon { hp: 40 })];
    for creature in horde.iter_mut() {
        creature.take_damage(20);
    }
    let hps: Vec<u32> = horde.iter().map(|c| c.hp()).collect();
    assert_eq!(hps, vec![20, 30, 20], "👻 the wraith still takes half damage");
}

#[test]
fn the_mage_lives_in_the_mage_module() {
    use grimoire::creatures::Goblin;
    use grimoire::mage::Mage;

    let mut mage = Mage::new("Tiberius", 7);
    let mut goblin = Goblin { hp: 50 };
    assert_eq!(mage.cast("fireball", &mut goblin), Ok(45));
    assert_eq!((mage.mana, goblin.hp), (70, 5));
}

#[test]
fn the_mage_still_follows_the_rules() {
    use grimoire::creatures::Dragon;
    use grimoire::error::CastError;
    use grimoire::mage::Mage;

    let mut mage = Mage::new("Tiberius", 7);
    let mut dragon = Dragon { hp: 200 };
    assert_eq!(
        mage.cast("Meteor", &mut dragon),
        Err(CastError::LevelTooLow { required: 10, current: 7 })
    );
    assert_eq!(mage.find_spell("Summon Pizza"), Err(CastError::UnknownSpell("Summon Pizza".into())));
}

#[test]
fn casting_with_a_modifier() {
    use grimoire::creatures::Goblin;
    use grimoire::mage::Mage;

    let mut mage = Mage::new("Tiberius", 7);
    let mut goblin = Goblin { hp: 100 };
    assert_eq!(mage.cast_with("Frostbolt", &mut goblin, |p| p + 5), Ok(40));
    assert_eq!(goblin.hp, 60);
}

// ── Phase 3: the effects wing — potions, buffs, burns ────────────────

#[test]
fn potions_live_in_effects_potions() {
    use grimoire::effects::potions::{Satchel, healing_draught, mana_elixir};
    use grimoire::mage::Mage;

    let mut mage = Mage::new("Tiberius", 7);
    mage.hp = 30;
    mage.mana = 0;
    let mut satchel = Satchel::new();
    satchel.add(healing_draught(50));
    satchel.add(mana_elixir(40));
    assert!(satchel.drink("Healing Draught", &mut mage));
    assert!(satchel.drink("mana elixir", &mut mage));
    assert_eq!((mage.hp, mage.mana), (80, 40));
    assert!(satchel.names().is_empty());
}

#[test]
fn buffs_live_in_effects_buffs() {
    use grimoire::effects::buffs::{arcane_focus, chain_power, empower, tick_buffs};

    let mut buffs = vec![arcane_focus(5, 1), empower(100, 2)];
    assert_eq!(chain_power(&buffs, 45), 100);
    tick_buffs(&mut buffs);
    assert_eq!(buffs.len(), 1);
    assert_eq!(chain_power(&buffs, 45), 90);
}

#[test]
fn burns_live_in_effects_burns() {
    use grimoire::creatures::Wraith;
    use grimoire::effects::burns::{Burn, tick_burns};

    let mut wraith = Wraith { hp: 50 };
    let mut burns = vec![Burn { source: "Ignite".into(), damage_per_turn: 10, turns_left: 1 }];
    assert_eq!(tick_burns(&mut burns, &mut wraith), 10);
    assert_eq!(wraith.hp, 45);
    assert!(burns.is_empty());
}

#[test]
fn the_effects_module_reexports_its_types() {
    // `grimoire::effects::Buff` must be the SAME type as
    // `grimoire::effects::buffs::Buff` — a re-export, not a copy.
    let buff: grimoire::effects::Buff = grimoire::effects::buffs::empower(10, 1);
    let burn: grimoire::effects::Burn =
        grimoire::effects::burns::Burn { source: "x".into(), damage_per_turn: 1, turns_left: 1 };
    let potion: grimoire::effects::Potion = grimoire::effects::potions::mana_elixir(1);
    let satchel: grimoire::effects::Satchel = grimoire::effects::potions::Satchel::new();
    assert_eq!((buff.name.as_str(), burn.turns_left, potion.name.as_str()), ("Empower", 1, "Mana Elixir"));
    assert!(satchel.potions.is_empty());
}

// ── Phase 4: the front door — stats, re-exports, prelude ─────────────

#[test]
fn stats_live_in_the_stats_module() {
    use grimoire::creatures::{Creature, Goblin};
    use grimoire::stats::{
        CastRecord, battle_report, favourite_spell, living, spell_counts, total_damage,
        weakest_living,
    };

    let log = vec![
        CastRecord { turn: 1, spell: "Fireball".into(), damage: 45 },
        CastRecord { turn: 2, spell: "Fireball".into(), damage: 45 },
    ];
    assert_eq!(total_damage(&log), 90);
    assert_eq!(favourite_spell(&log), Some("Fireball"));
    assert_eq!(spell_counts(&log)["Fireball"], 2);
    assert_eq!(battle_report(&log[..1]), "Turn 1: Fireball for 45");

    let creatures: Vec<Box<dyn Creature>> = vec![Box::new(Goblin { hp: 0 }), Box::new(Goblin { hp: 9 })];
    assert_eq!(living(&creatures), vec!["Goblin"]);
    assert_eq!(weakest_living(&creatures), Some(1));
}

#[test]
fn the_most_used_types_are_reexported_at_the_root() {
    let mage: grimoire::Mage = grimoire::mage::Mage::new("Tiberius", 7);
    let spell: grimoire::Spell = grimoire::spells::spellbook().remove(0);
    let err: grimoire::CastError = grimoire::error::CastError::UnknownSpell("?".into());
    let goblin = grimoire::creatures::Goblin { hp: 3 };
    let creature: &dyn grimoire::Creature = &goblin;
    assert_eq!((mage.level, spell.cost, creature.hp()), (7, 30, 3));
    assert!(err.to_string().contains("Unknown spell"));
}

#[test]
fn the_prelude_brings_everything_a_battle_needs() {
    use grimoire::prelude::*;

    let mut mage = Mage::new("Tiberius", 7);
    let mut creatures: Vec<Box<dyn Creature>> = vec![
        Box::new(Goblin { hp: 40 }),
        Box::new(Wraith { hp: 40 }),
        Box::new(Dragon { hp: 40 }),
    ];
    let mut buffs: Vec<Buff> = vec![arcane_focus(15, 2), empower(0, 1)];
    let mut burns: Vec<Burn> = vec![Burn { source: "Ignite".into(), damage_per_turn: 10, turns_left: 2 }];
    let mut satchel = Satchel::new();
    satchel.add(healing_draught(10));
    satchel.add(mana_elixir(10));
    let _: Option<Potion> = None;
    let _: Option<CastError> = None;
    let _: Vec<Spell> = spellbook();

    let power = mage.cast_with("Fireball", creatures[0].as_mut(), |p| chain_power(&buffs, p));
    assert_eq!(power, Ok(60));
    assert_eq!(tick_burns(&mut burns, creatures[2].as_mut()), 10);
    tick_buffs(&mut buffs);
    assert_eq!(buffs.len(), 1);
    assert!(satchel.drink("Mana Elixir", &mut mage));
    assert_eq!(mage.mana, 80.min(MAX_MANA));
}
