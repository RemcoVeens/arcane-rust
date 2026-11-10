use std::io::Write;
use std::sync::mpsc;

use ch07_07_project_grand_battle::{Battlefield, Creature, Mage, Outcome, grimoire, play_round};

fn main() {
    let field = Battlefield::new(
        Mage::tiberius(),
        vec![Creature::goblin(), Creature::wraith(), Creature::ice_golem()],
    );
    let (tx, rx) = mpsc::channel();

    println!("⚔️  THE GRAND BATTLE ⚔️");
    println!("A horde blocks the mountain pass. Every round, you and every creature strike at once.");
    println!("Spells:");
    for spell in grimoire() {
        println!("  {:<13} mana {:>3}, power {:>3}, level {:>2}, {:?}", spell.name, spell.cost, spell.power, spell.level, spell.element);
    }
    println!("Type a spell name each round, or 'quit'.");

    let mut round = 1;
    loop {
        let state = field.snapshot();
        println!("\n── Round {round} ──  {} ❤️ {}/{}  ✦ {}", state.mage.name, state.mage.hp, state.mage.max_hp, state.mage.mana);
        for c in state.creatures.iter().filter(|c| c.is_alive()) {
            println!("   {} ❤️ {}/{}", c.name, c.hp, c.max_hp);
        }
        print!("> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).unwrap() == 0 || input.trim().eq_ignore_ascii_case("quit") {
            println!("You retreat into the mist.");
            break;
        }

        play_round(&field, input.trim(), &tx);
        // Every combatant thread has finished, so all events are waiting.
        for event in rx.try_iter() {
            println!("  {event}");
        }

        match field.outcome() {
            Some(Outcome::Victory) => {
                println!("\n🏆 VICTORY! The pass is clear.");
                break;
            }
            Some(Outcome::Defeat) => {
                println!("\n☠️  DEFEAT. The horde marches on...");
                break;
            }
            _ => round += 1,
        }
    }
}
