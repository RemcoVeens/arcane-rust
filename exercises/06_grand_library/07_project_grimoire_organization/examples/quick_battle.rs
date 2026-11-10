//! A tiny scripted battle that shows off the public API.
//!
//!     cargo run -p ch06_07_project_grimoire_organization --example quick_battle

use grimoire::prelude::*;
use grimoire::stats::{CastRecord, battle_report};

fn main() {
    let mut mage = Mage::new("Tiberius", 7);
    let mut dragon = Dragon { hp: 120 };
    let buffs = vec![arcane_focus(15, 3), empower(50, 3)];
    let mut log = Vec::new();

    for (turn, spell) in ["Arcane Blast", "Fireball"].into_iter().enumerate() {
        match mage.cast_with(spell, &mut dragon, |p| chain_power(&buffs, p)) {
            Ok(damage) => log.push(CastRecord { turn: turn as u32 + 1, spell: spell.to_string(), damage }),
            Err(e) => println!("❌ {e}"),
        }
    }

    println!("{}", battle_report(&log));
    println!("The dragon has {} HP left.", dragon.hp());
}
