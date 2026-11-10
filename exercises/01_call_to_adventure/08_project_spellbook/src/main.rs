use ch01_08_project_spellbook::{MAX_MANA, banner, handle};
use std::io::Write;

fn main() {
    let mage_name = "Tiberius";
    let level = 7;
    let mut mana = MAX_MANA;

    println!("{}", banner());
    println!("Welcome, {mage_name}. Type 'help' for commands.");

    loop {
        print!("\n[{mage_name} | Lv {level} | ✦ {mana}/{MAX_MANA}] > ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        if std::io::stdin().read_line(&mut input).unwrap() == 0 {
            break; // Ctrl-D
        }

        let (new_mana, message, keep_playing) = handle(&input, mana, level);
        mana = new_mana;
        println!("{message}");
        if !keep_playing {
            break;
        }
    }
}
