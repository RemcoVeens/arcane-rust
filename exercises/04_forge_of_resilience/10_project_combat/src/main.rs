use ch04_10_project_combat::{Battle, Command, Outcome, parse_command};
use std::io::Write;

fn main() {
    let mut battle = Battle::standard();

    println!("⚔️  An ambush! Three foes block the road to the Academy.");
    println!("Type 'help' for commands.\n");
    println!("{}", battle.status());

    loop {
        print!("\n> ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).unwrap() == 0 {
            break; // Ctrl-D
        }
        if parse_command(&input) == Some(Command::Quit) {
            println!("You flee into the forest. The Academy will have to wait.");
            break;
        }

        match battle.take_turn(&input) {
            Ok(message) => println!("{message}"),
            Err(error) => println!("⚠️  {error}"),
        }

        match battle.outcome() {
            Some(Outcome::Victory) => {
                println!("\n🏆 Victory! The road to the Academy is clear.");
                break;
            }
            Some(Outcome::Defeat) => {
                println!("\n💀 You have fallen. The Academy will remember your name.");
                break;
            }
            None => {}
        }
    }
}
