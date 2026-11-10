use ch02_07_project_mana_system::{MAX_MANA, STARTING_SPELLS, TOWER_SCROLLS, handle};
use std::io::Write;

fn main() {
    let mage_name = "Tiberius";
    let level = 7;
    let mut mana = MAX_MANA;
    let mut spellbook = STARTING_SPELLS.to_vec();
    let mut scrolls = TOWER_SCROLLS.to_vec();

    println!("╔══════════════════════════════════╗");
    println!("║   ARCANE GRIMOIRE v0.2           ║");
    println!("║   ✦ The Ownership Edition ✦      ║");
    println!("╚══════════════════════════════════╝");
    println!("Welcome, {mage_name}. Type 'help' for commands.");

    loop {
        print!(
            "\n[{mage_name} | Lv {level} | ✦ {mana}/{MAX_MANA} | 📜 {}] > ",
            scrolls.len()
        );
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).unwrap() == 0 {
            break; // Ctrl-D
        }

        let (message, keep_playing) = handle(&input, &mut mana, level, &mut spellbook, &mut scrolls);
        println!("{message}");
        if !keep_playing {
            break;
        }
    }
}
