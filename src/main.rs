const MAX_MANA: u32 = 100;

fn print_banner() {
    println!("╔══════════════════════════════════╗");
    println!("║      ARCANE GRIMOIRE v0.1        ║");
    println!("╚══════════════════════════════════╝");
    println!();
}

fn main() {
    let mage_name = "tiberius";
    let mut mana = 100;
    let mut enemy_healt = 100;
    let level = 7;

    let spells = [
        ("Fireball", 30, 45, 1),
        ("Frostbolt", 25, 35, 2),
        ("Arcane Blast", 40, 60, 5),
        ("Heal", 20, 0, 1),
        ("Shield", 15, 0, 10),
    ];

    print_banner();

    loop {
        println!();
        println!("-----------------------------");
        println!("{mage_name} | Level {level} | Mana: {mana}/{MAX_MANA}");
        println!("-----------------------------");
        println!("Commands: cast <name>, list, rest, potion, help, quit");
        println!();

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim();

        if input == "quit" {
            println!("Farwell, {mage_name}. May your magic never fade.");
            break;
        } else if input == "list" {
            println!("\n=== Known Spells ===");
            for (i, spell) in spells.iter().enumerate() {
                let (name, cost, damage, req_level) = spell;
                println!(
                    " {}, {name} -- Mana: {cost}, Power: {damage}, Requierd level: {req_level}",
                    i + 1
                );
            }
        } else if input == "potion" {
            mana += 50;
            println!("You drank a potion and gained an additional 50 mana (current = {mana})")
        } else if input == "rest" {
            mana = MAX_MANA;
            println!("you meditate and restore to full mana.")
        } else if input.starts_with("cast ") {
            let spell_name = input.trim_start_matches("cast ");
            let mut found = false;
            for spell in spells {
                let (name, cost, damage, req_level) = spell;
                if name.to_lowercase() == spell_name.to_lowercase() {
                    if level < req_level {
                        println!("You are not powerfull enouth to cast this spell.");
                        println!("Current level: {level}, need to be level {req_level}");
                    } else if mana >= cost {
                        mana -= cost;
                        let cirt = std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .subsec_nanos()
                            % 5
                            == 0;
                        let mut dealt_damage = damage;
                        if cirt {
                            dealt_damage = damage * 2;
                        }
                        println!("You cast {name}! (Dealt {dealt_damage} damage)");
                        enemy_healt -= dealt_damage;
                        if enemy_healt <= 0 {
                            println!("Enemy defeated!")
                        }
                    } else {
                        println!("Not enough mana! Need {cost}, have {mana}");
                    }
                    found = true;
                    break;
                }
            }
            if !found {
                println!("Unknow spell: '{spell_name}', Check your grimore with 'list'.")
            }
        } else if input == "help" {
            println!("cast [name]: cast a spell");
            println!("list:        list the spells");
            println!("rest:        restore to full mana");
            println!("potion:      drink a potion to restore 50 mana");
            println!("help:        print this menue");
            println!("quit:        exit game");
        } else {
            println!("Unknown command: '{input}'. Try: cast, list, rest, help, quit")
        }
    }
}
