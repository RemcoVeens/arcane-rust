use std::io::Write;

use ch08_06_project_complete_saga::{Creature, Mage, Outcome, Party, Saga, grimoire};

const TITLE: &str = r#"
   ╔══════════════════════════════════════╗
   ║   T H E   C O M P L E T E   S A G A  ║
   ║        a Rust Mage's last quest      ║
   ╚══════════════════════════════════════╝
"#;

fn read_plan(prompt: &str) -> Option<String> {
    print!("{prompt}");
    std::io::stdout().flush().unwrap();
    let mut line = String::new();
    if std::io::stdin().read_line(&mut line).unwrap() == 0 {
        return None;
    }
    Some(line.trim().to_string())
}

fn main() {
    println!("{TITLE}");

    let mut party = Party::new(3);
    party.recruit(Mage::tiberius());
    party.recruit(Mage::new("Morgana", 3));
    let mut saga = Saga::new(party);

    println!("Spells you know:");
    for s in grimoire() {
        println!("  {:<13} mana {:>3}  power {:>3}  level {:>2}  {:?}", s.name, s.cost, s.power, s.level, s.element);
    }

    let acts: Vec<(&str, Vec<Creature>)> = vec![
        ("Act I — The Goblin Caves", vec![Creature::goblin(), Creature::goblin(), Creature::wraith()]),
        ("Act II — The Frozen Tundra", vec![Creature::ice_golem(), Creature::wraith()]),
        ("Act III — The Dragon's Peak", vec![Creature::dragon()]),
    ];

    for (title, horde) in acts {
        println!("\n═══ {title} ═══");
        let names: Vec<_> = horde.iter().map(|c| format!("{} ({} hp)", c.name, c.hp)).collect();
        println!("Before you: {}", names.join(", "));
        if let Some(leader) = saga.party.leader() {
            let hero = leader.borrow();
            println!("{} steps forward: ❤️ {}/{}  ✦ {}", hero.name, hero.hp, hero.max_hp, hero.mana);
        }

        let Some(plan) = read_plan("Your battle plan (spells, comma-separated): ") else {
            println!("The saga ends here.");
            return;
        };
        let plan: Vec<&str> = plan.split(',').map(str::trim).filter(|s| !s.is_empty()).collect();

        match saga.encounter(horde, &plan) {
            Ok(outcome) => {
                println!("{}", saga.chronicle.last().unwrap());
                match outcome {
                    Outcome::Victory => println!("🏆 Victory!"),
                    Outcome::Defeat => println!("☠️  Your leader has fallen..."),
                    Outcome::Retreat => println!("🏃 You retreat to fight another day."),
                }
            }
            Err(err) => {
                println!("The saga is over: {err}.");
                return;
            }
        }

        let stash = saga.party.stash();
        println!("Stash: {} gold, {} potions, trophies: {:?}", stash.borrow().gold, stash.borrow().potions, stash.borrow().trophies);

        if let Some(leader) = saga.party.leader()
            && leader.borrow().hp < 50
            && saga.party.drink_potion(&leader)
        {
            println!("🧪 {} drinks a potion.", leader.borrow().name);
        }
        if saga.party.leader().is_none() {
            println!("\nNo hero is left standing. The saga ends in darkness.");
            return;
        }
        println!("🌙 The party rests at the inn.");
        saga.party.rest();
    }

    println!("\n✨ The saga is complete. Your name will be sung in the halls of the Academy. ✨");
}
