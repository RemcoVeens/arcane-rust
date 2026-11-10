use ch05_07_project_potions_buffs::*;
use std::io::Write;

const HELP: &str = "Commands:
  cast <target#> <spell>   cast a spell, e.g. `cast 2 fireball` (buffs apply!)
  ignite <target#>         set a creature ablaze: 10 dmg/turn for 3 turns (15 mana)
  focus                    Arcane Focus: +15 power for 3 turns (10 mana)
  drink <potion>           drink a potion from your satchel
  status                   show the battlefield
  report                   show the battle report
  help                     show this help
  quit                     flee the battle";

fn status(mage: &Mage, creatures: &[Box<dyn Creature>], buffs: &[Buff], satchel: &Satchel) -> String {
    let mut lines = vec![format!(
        "🧙 {} — HP {}/{} — ✦ {}/{MAX_MANA}",
        mage.name, mage.hp, mage.max_hp, mage.mana
    )];
    for (i, c) in creatures.iter().enumerate() {
        let state = if c.is_alive() { format!("HP {}", c.hp()) } else { String::from("defeated") };
        lines.push(format!("  {}. {} — {state}", i + 1, c.name()));
    }
    let active: Vec<String> = buffs.iter().map(|b| format!("{} ({} turns)", b.name, b.turns_left)).collect();
    lines.push(format!("  Buffs: {}", if active.is_empty() { "none".to_string() } else { active.join(", ") }));
    lines.push(format!("  Satchel: {}", satchel.names().join(", ")));
    lines.join("\n")
}

/// Parse "<target#>" into an index of a living creature.
fn pick_target(arg: &str, creatures: &[Box<dyn Creature>]) -> Option<usize> {
    let index = arg.parse::<usize>().ok()?.checked_sub(1)?;
    creatures.get(index).filter(|c| c.is_alive()).map(|_| index)
}

fn main() {
    let mut mage = Mage::new("Tiberius", 7);
    let mut creatures: Vec<Box<dyn Creature>> = vec![
        Box::new(Goblin { hp: 40 }),
        Box::new(Wraith { hp: 60 }),
        Box::new(Dragon { hp: 120 }),
    ];
    let mut burns: Vec<Vec<Burn>> = creatures.iter().map(|_| Vec::new()).collect();
    let mut buffs: Vec<Buff> = Vec::new();
    let mut satchel = Satchel::new();
    satchel.add(healing_draught(40));
    satchel.add(healing_draught(40));
    satchel.add(mana_elixir(50));
    let mut log: Vec<CastRecord> = Vec::new();
    let mut turn = 1;

    println!("⏳ THE TEMPORAL LOOM ⏳ — a goblin, a wraith and a dragon block your path.");
    println!("{HELP}\n");
    println!("{}", status(&mage, &creatures, &buffs, &satchel));

    loop {
        print!("\n[Turn {turn}] > ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).unwrap() == 0 {
            break;
        }
        let input = input.trim();
        let (command, rest) = input.split_once(' ').unwrap_or((input, ""));

        // `true` if the action used up the turn
        let acted = match command.to_lowercase().as_str() {
            "cast" => {
                let (target_arg, spell) = rest.split_once(' ').unwrap_or((rest, ""));
                match pick_target(target_arg, &creatures) {
                    None => {
                        println!("Pick a living target by number.");
                        false
                    }
                    Some(i) => {
                        // use the spell's proper name ("fireball" → "Fireball") for the log
                        let spell = match mage.find_spell(spell) {
                            Ok(found) => found.name.clone(),
                            Err(_) => spell.to_string(),
                        };
                        let target = creatures[i].as_mut();
                        match mage.cast_with(&spell, target, |p| chain_power(&buffs, p)) {
                            Ok(power) => {
                                println!("✨ {spell} hits the {} for {power}!", creatures[i].name());
                                log.push(CastRecord { turn, spell, damage: power });
                                true
                            }
                            Err(e) => {
                                println!("❌ {e}");
                                false
                            }
                        }
                    }
                }
            }
            "ignite" => match pick_target(rest, &creatures) {
                Some(i) if mage.mana >= 15 => {
                    mage.mana -= 15;
                    burns[i].push(Burn { source: String::from("Ignite"), damage_per_turn: 10, turns_left: 3 });
                    println!("🔥 The {} catches fire!", creatures[i].name());
                    true
                }
                Some(_) => {
                    println!("❌ Not enough mana (15 needed)");
                    false
                }
                None => {
                    println!("Pick a living target by number.");
                    false
                }
            },
            "focus" if mage.mana >= 10 => {
                mage.mana -= 10;
                buffs.push(arcane_focus(15, 3));
                println!("🔮 Your mind sharpens: +15 power for 3 turns.");
                true
            }
            "focus" => {
                println!("❌ Not enough mana (10 needed)");
                false
            }
            "drink" => {
                if satchel.drink(rest, &mut mage) {
                    println!("🧪 You drink the {rest}.");
                    true
                } else {
                    println!("You have no '{rest}'.");
                    false
                }
            }
            "status" => {
                println!("{}", status(&mage, &creatures, &buffs, &satchel));
                false
            }
            "report" => {
                println!("{}", battle_report(&log));
                println!("Total damage: {}", total_damage(&log));
                false
            }
            "help" => {
                println!("{HELP}");
                false
            }
            "quit" => break,
            _ => {
                println!("Unknown command. Type 'help'.");
                false
            }
        };

        if !acted {
            continue;
        }

        // ── the loom turns: effects tick, creatures strike back ──
        for (creature, creature_burns) in creatures.iter_mut().zip(burns.iter_mut()) {
            if creature.is_alive() {
                let dealt = tick_burns(creature_burns, creature.as_mut());
                if dealt > 0 {
                    println!("🔥 The {} burns for {dealt}.", creature.name());
                }
            }
        }
        tick_buffs(&mut buffs);

        if living(&creatures).is_empty() {
            println!("\n🏆 Victory! The path is clear.");
            println!("Favourite spell: {}", favourite_spell(&log).unwrap_or("none"));
            break;
        }

        let incoming: u32 = creatures.iter().filter(|c| c.is_alive()).map(|c| c.attack()).sum();
        mage.hp = mage.hp.saturating_sub(incoming);
        mage.mana = (mage.mana + 10).min(MAX_MANA);
        println!("⚔️  The creatures strike you for {incoming}.");
        if let Some(i) = weakest_living(&creatures) {
            println!("👁️  The {} (#{}) looks weakest.", creatures[i].name(), i + 1);
        }

        if mage.hp == 0 {
            println!("\n💀 Tiberius has fallen. The loom unravels...");
            break;
        }
        turn += 1;
    }
}
