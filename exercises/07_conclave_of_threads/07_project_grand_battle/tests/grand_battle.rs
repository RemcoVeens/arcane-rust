use ch07_07_project_grand_battle::*;
use std::sync::mpsc;
use std::thread;

fn horde() -> Vec<Creature> {
    vec![Creature::goblin(), Creature::wraith()]
}

fn sorted(mut v: Vec<String>) -> Vec<String> {
    v.sort();
    v
}

// ── Phase 1: the battlefield ─────────────────────────────────────────

#[test]
fn a_new_battlefield_holds_the_combatants() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    let state = field.snapshot();
    assert_eq!(state.mage, Mage::tiberius());
    assert_eq!(state.creatures, horde());
}

#[test]
fn clones_share_one_battlefield() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    let other = field.clone();
    other.strike_creature(0, 10);
    assert_eq!(field.snapshot().creatures[0].hp, 20, "🛡️ a clone must point at the SAME state");
}

#[test]
fn only_the_killing_blow_reports_a_defeat() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    assert!(!field.strike_creature(0, 20), "Goblin survives with 10 hp");
    assert!(field.strike_creature(0, 20), "this blow fells the Goblin");
    assert!(!field.strike_creature(0, 20), "a fallen Goblin can't fall twice");
    assert!(!field.strike_creature(99, 20), "no creature at index 99");
    assert_eq!(field.snapshot().creatures[0].hp, 0);
}

#[test]
fn many_threads_strike_the_mage_but_only_one_fells_him() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    let handles: Vec<_> = (0..20)
        .map(|_| {
            let field = field.clone();
            thread::spawn(move || field.strike_mage(10))
        })
        .collect();
    let killing_blows = handles.into_iter().map(|h| h.join().unwrap()).filter(|&b| b).count();
    assert_eq!(killing_blows, 1, "exactly one blow takes Tiberius from 10 hp to 0");
    assert_eq!(field.snapshot().mage.hp, 0);
}

#[test]
fn casting_spends_shared_mana() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    assert_eq!(field.cast(&find_spell("Arcane Blast").unwrap()), Ok(60));
    assert_eq!(field.snapshot().mage.mana, 60);
    assert!(matches!(
        field.cast(&find_spell("Meteor").unwrap()),
        Err(BattleError::LevelTooLow { .. })
    ));
}

#[test]
fn outcomes_follow_the_fallen() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    assert_eq!(field.outcome(), None);
    field.strike_creature(0, 100);
    assert_eq!(field.outcome(), None, "the Wraith still stands");
    field.strike_creature(1, 100);
    assert_eq!(field.outcome(), Some(Outcome::Victory));
    field.strike_mage(100);
    assert_eq!(field.outcome(), Some(Outcome::Defeat), "a fallen mage is a defeat, always");
}

// ── Phase 2: the chronicler ──────────────────────────────────────────

#[test]
fn the_report_records_events() {
    let mut report = BattleReport::default();
    report.record(&BattleEvent::Attack { attacker: "Goblin".into(), target: "Tiberius".into(), damage: 8 });
    report.record(&BattleEvent::Attack { attacker: "Goblin".into(), target: "Tiberius".into(), damage: 8 });
    report.record(&BattleEvent::SpellCast {
        caster: "Tiberius".into(),
        spell: "Fireball".into(),
        target: "Goblin".into(),
        damage: 90,
    });
    report.record(&BattleEvent::Defeated { name: "Goblin".into() });
    report.record(&BattleEvent::Fizzle { caster: "Tiberius".into(), reason: "tired".into() });

    assert_eq!(report.lines.len(), 5);
    assert_eq!(report.lines[0], "🗡️  Goblin attacks Tiberius for 8", "lines use the event's Display");
    assert_eq!(report.damage_by("Goblin"), 16);
    assert_eq!(report.damage_by("Tiberius"), 90);
    assert_eq!(report.damage_by("Dragon"), 0);
    assert_eq!(report.defeated, vec!["Goblin"]);
    assert_eq!(report.fizzles, 1);
}

#[test]
fn the_chronicler_hears_every_thread() {
    let (tx, rx) = mpsc::channel();
    let chronicler = spawn_chronicler(rx);
    let senders: Vec<_> = (0..5)
        .map(|_| {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(BattleEvent::Attack { attacker: "Imp".into(), target: "Tiberius".into(), damage: 3 })
                    .unwrap()
            })
        })
        .collect();
    for s in senders {
        s.join().unwrap();
    }
    drop(tx);
    let report = chronicler.join().unwrap();
    assert_eq!(report.lines.len(), 5);
    assert_eq!(report.damage_by("Imp"), 15);
}

// ── Phase 3: concurrent turns ────────────────────────────────────────

#[test]
fn a_creature_turn_strikes_and_reports() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    let (tx, rx) = mpsc::channel();
    creature_turn(&field, &Creature::wraith(), "Tiberius", &tx);
    drop(tx);
    let events: Vec<_> = rx.iter().collect();
    assert_eq!(
        events,
        vec![BattleEvent::Attack { attacker: "Wraith".into(), target: "Tiberius".into(), damage: 12 }]
    );
    assert_eq!(field.snapshot().mage.hp, 88);
}

#[test]
fn the_mage_targets_the_first_living_creature() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    field.strike_creature(0, 100); // the Goblin is already down
    let (tx, rx) = mpsc::channel();
    mage_turn(&field, "arcane blast", &tx);
    drop(tx);
    let events: Vec<_> = rx.iter().collect();
    assert_eq!(
        events,
        vec![
            BattleEvent::SpellCast {
                caster: "Tiberius".into(),
                spell: "Arcane Blast".into(),
                target: "Wraith".into(),
                damage: 120,
            },
            BattleEvent::Defeated { name: "Wraith".into() },
        ],
        "Arcane Blast hits the Wraith's weakness: 60 × 2"
    );
    assert_eq!(field.snapshot().mage.mana, 60);
}

#[test]
fn failed_casts_fizzle() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    let (tx, rx) = mpsc::channel();
    mage_turn(&field, "Meteor", &tx);
    mage_turn(&field, "Summon Pizza", &tx);
    drop(tx);
    let events: Vec<_> = rx.iter().collect();
    assert_eq!(events.len(), 2);
    for event in &events {
        assert!(matches!(event, BattleEvent::Fizzle { .. }), "expected a fizzle, got {event:?}");
    }
    assert_eq!(field.snapshot().mage.mana, 100, "fizzles cost nothing");
}

#[test]
fn everyone_alive_at_the_start_of_a_round_acts() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    let (tx, rx) = mpsc::channel();
    play_round(&field, "Fireball", &tx);
    drop(tx);
    let lines = sorted(rx.iter().map(|e| e.to_string()).collect());
    assert_eq!(
        lines,
        sorted(vec![
            "✨ Tiberius casts Fireball at Goblin for 90".to_string(),
            "💀 Goblin is defeated!".to_string(),
            "🗡️  Goblin attacks Tiberius for 8".to_string(),
            "🗡️  Wraith attacks Tiberius for 12".to_string(),
        ]),
        "the Goblin falls this round, but still gets its (simultaneous) blow in"
    );
    let state = field.snapshot();
    assert_eq!(state.mage.hp, 80);
    assert_eq!(state.mage.mana, 70);
}

#[test]
fn the_fallen_do_not_act() {
    let field = Battlefield::new(Mage::tiberius(), horde());
    field.strike_creature(0, 100);
    let (tx, rx) = mpsc::channel();
    play_round(&field, "Frostbolt", &tx);
    drop(tx);
    let attackers: Vec<_> = rx
        .iter()
        .filter_map(|e| match e {
            BattleEvent::Attack { attacker, .. } => Some(attacker),
            _ => None,
        })
        .collect();
    assert_eq!(attackers, vec!["Wraith"], "a goblin that was already down stays down");
}

// ── Phase 4: the grand battle ────────────────────────────────────────

#[test]
fn tiberius_clears_the_pass() {
    let report = run_battle(Mage::tiberius(), horde(), &["Fireball", "Frostbolt", "Fireball", "Fireball"]);
    assert_eq!(report.outcome, Outcome::Victory);
    assert_eq!(report.rounds, 3, "the battle stops as soon as the Wraith falls");
    assert_eq!(report.damage_by("Tiberius"), 90 + 35 + 45);
    assert_eq!(report.damage_by("Goblin"), 8);
    assert_eq!(report.damage_by("Wraith"), 36);
    assert_eq!(sorted(report.defeated.clone()), vec!["Goblin", "Wraith"]);
    assert_eq!(report.lines.len(), 9);
}

#[test]
fn the_dragon_is_too_much() {
    let report = run_battle(Mage::tiberius(), vec![Creature::dragon()], &["Fireball"; 5]);
    assert_eq!(report.outcome, Outcome::Defeat);
    assert_eq!(report.rounds, 3, "40 damage a round: Tiberius lasts three rounds");
    assert_eq!(report.defeated, vec!["Tiberius"]);
    assert_eq!(report.damage_by("Tiberius"), 135);
    assert_eq!(report.fizzles, 0);
}

#[test]
fn running_out_of_mana_and_plan_means_retreat() {
    let dummy = Creature::new("Training Dummy", 500, 5, None);
    let report = run_battle(Mage::tiberius(), vec![dummy], &["Arcane Blast", "Arcane Blast", "Arcane Blast"]);
    assert_eq!(report.outcome, Outcome::Retreat);
    assert_eq!(report.rounds, 3);
    assert_eq!(report.fizzles, 1, "the third Arcane Blast has no mana left");
    assert_eq!(report.damage_by("Tiberius"), 120);
    assert_eq!(report.damage_by("Training Dummy"), 15);
}

#[test]
fn battles_are_deterministic() {
    let plan = ["Frostbolt", "Fireball", "Arcane Blast", "Fireball"];
    let first = run_battle(Mage::tiberius(), vec![Creature::goblin(), Creature::wraith(), Creature::ice_golem()], &plan);
    for _ in 0..20 {
        let again = run_battle(Mage::tiberius(), vec![Creature::goblin(), Creature::wraith(), Creature::ice_golem()], &plan);
        assert_eq!(again.outcome, first.outcome);
        assert_eq!(again.damage_dealt, first.damage_dealt);
        assert_eq!(sorted(again.lines.clone()), sorted(first.lines.clone()), "🎲 thread order must not change the result");
    }
}
