// The binary half of this package. It uses the *library* half by its crate
// name (the package name, with `-` turned into `_`), exactly like an outsider.
use ch06_01_crates_packages::{glyph_name, greeting_stone, spell_power};

fn main() {
    let mage = std::env::args().nth(1).unwrap_or_else(|| "Tiberius".to_string());
    let stone = greeting_stone(&mage);

    println!("🪨 {}", stone.inscription());
    println!("   in runes: {}", glyph_name(stone.inscription()));
    println!("   power:    {}", stone.power());

    for spell in ["Fireball", "Frostbolt", "Arcane Blast", "Heal", "Meteor"] {
        println!("   {spell:<12} → {:>3} rune power", spell_power(spell));
    }
}
