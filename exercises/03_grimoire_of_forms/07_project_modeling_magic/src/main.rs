use ch03_07_project_modeling_magic::{Command, Game, MAX_MANA};
use std::io::Write;

fn main() {
    let mut game = Game::start();

    println!("╔══════════════════════════════════╗");
    println!("║   ARCANE GRIMOIRE v0.3           ║");
    println!("║   ✦ The Tower of Forms ✦         ║");
    println!("╚══════════════════════════════════╝");
    println!("Welcome, {}. Type 'help' for commands.", game.mage.name);
    println!("{}", game.handle(Command::Look));

    while game.running {
        let mage = &game.mage;
        print!(
            "\n[{} | Lv {} | ✦ {}/{MAX_MANA} | XP {} | 📜 {}] > ",
            mage.name,
            mage.level,
            mage.mana,
            mage.xp,
            mage.scrolls.len()
        );
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).unwrap() == 0 {
            break; // Ctrl-D
        }

        let command = Command::parse(&input);
        println!("{}", game.handle(command));
    }
}
