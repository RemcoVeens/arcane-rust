fn main() {
    let fireball = ("Fireball", 30, 45);
    let frostbolt = ("Frostbolt", 25, 35);
    let arcane_blast = ("Arcane Blast", 40, 60);

    let (f_name, f_cost, f_dmg) = fireball;
    println!("Spell: {f_name} | Mana:{f_cost} | Damage: {f_dmg}");

    let costs = [f_cost, 25, 40];
    let cheapest = costs[0];

    println!("First spell costs: {cheapest}");

    let character = ("tiberius", 7, 100.0, true);
    let (c_name, c_level, c_hp, c_alive) = character;
    println!("\nCharacter: {c_name} (level {c_level})");
    println!("HP: {c_hp} | Alive: {c_alive}");

    let favorate_potion = ("immortality", "healing", 70, 60);
    let (p_name, p_effect, p_potency, p_duration) = favorate_potion;
    println!(
        "Favorite potion: {p_name} ({p_effect}) | Potency: {p_potency} | Duration: {p_duration}"
    );

    let spells_per_level: [u32; 10] = [3, 3, 5, 7, 10, 14, 19, 25, 32, 40];
    let spells_at_7 = spells_per_level[6];
    println!("At level 7, you know {spells_at_7} spells");
}
