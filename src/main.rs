fn main() {
    let mut mana = 0;
    while mana < 100 {
        println!("Meditating... {mana} mana");
        mana += 25;
    }
    println!("Fully restored!\n");

    for level in 1..=5 {
        let spells_unlocked = level * 2;
        println!("Level {level}: {spells_unlocked} spells available");
    }
    println!();

    let spellbook = ["Fireball", "Frostbolt", "Arcane Blast", "Heal", "Shield"];
    println!("=== Spellbool ===");
    for spell in spellbook {
        println!("  - {spell}");
    }
    println!();

    for n in (1..=10).rev() {
        println!("{n} ");
    }
    println!("Liftoff!");

    println!();
    for i in 1..=5 {
        for j in 1..=5 {
            let sum = i * j;
            if sum < 10 {
                print!("  {sum}");
            } else {
                print!(" {sum}");
            }
        }
        println!()
    }

    println!();
    let mut i = 1;
    loop {
        if i % 7 == 0 && i % 13 == 0 {
            break;
        }
        i += 1;
    }
    println!("{i} is the first number divisible by both 7 and 13");
}
