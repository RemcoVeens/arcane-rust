fn main() {
    let name = "Tiberius";
    let level = 7;
    let mana = 100;
    let current_exp = 5;

    greet_mage(name, level);

    let max_mana = calculate_max_mana(level);
    println!("Max mana: {max_mana}");

    let can_cast = can_cast_spell(mana, 30);
    println!("Can cast Fireball (30 mana): {can_cast}");

    let mana_after = cast_spell(mana, 30);
    println!("Mana after casting: {mana_after}");

    let can_level = can_level_up(current_exp, 50);
    println!("can {name} level up? {can_level}")
}
fn greet_mage(name: &str, level: u32) {
    println!("Welcome, {name}! You are level {level}.");
}

fn calculate_max_mana(level: u32) -> u32 {
    50 + (25 * level)
}
fn can_cast_spell(current_mana: u32, cost: u32) -> bool {
    current_mana >= cost
}

fn cast_spell(current_mana: u32, cost: u32) -> u32 {
    current_mana - cost
}

fn can_level_up(exp: u32, required_exp: u32) -> bool {
    exp >= required_exp
}
