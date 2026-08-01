use std::println;

const MAX_MANA: u32 = 10_000;
const RESTORE_RATE: u32 =25;
const SPELL_NAMES: &str = "Fireball, Frostbolt, Arcane Blast";
const XP_PER_LEVEL:u32 = 10;

fn main() {
    println!("=== Constants ===");
    println!("Max mana: {MAX_MANA}");
    println!("Restore rate: {RESTORE_RATE} mana/tick");
    println!("Known spells: {SPELL_NAMES}");
    
    println!("\n=== Shadowing (type change) ===");
    let mana = 100;
    println!("Mana as number: {mana}");
    let mana = format!("{mana}/100");
    println!("Mana as display: {mana}");

    println!("\n== Shadowing (block scope) ===");
    let x = 1;
    {
        let x =2;
        println!("Inner x: {x}");
        {
            let x =3;
            println!("Deepest x: {x}");
        }
        println!("Inner x again: {x}");
    }
    println!("Outer x: {x}");

    println!("\n=== Shadowing (transformation) ===");
    let spell = "  fireball  ";
    let spell = spell.trim();
    let spell = spell.to_uppercase();
    println!("Ready to cast: {spell}");

    println!("\n=== chalanges ===");
    let xp_till_ten = XP_PER_LEVEL * 10;
    println!("XP needed to reach level 10: {xp_till_ten} xp");

    let num = "42";
    let num:u32 = num.parse().expect("not a number");
    let num = num*2;

    println!("num = {num}");

    let outer:&str = "hellow world";
    println!("outer: {outer}");
    {
        let outer:u32 = 67;
        println!("Inner: {outer}");
    }
    println!("outer: {outer}");

}
