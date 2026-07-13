fn main() {
    let max_mana: u32 = 10_000;
    let current_mana = 3_250;
    let mana_pct = (current_mana as f64 / max_mana as f64) * 100.0;

    println!("Mage Report");
    println!("===========");
    println!("Max mana:    {max_mana}");
    println!("Current:     {current_mana}");
    println!("Percentage:  {mana_pct:.1}%");

    let name = "Tiberius";
    let rank: u8 = 7;
    let is_archmage = false;
    let element: char = '🔥';

    println!("\nCharacter: {name}");
    println!("Rank:      {rank}");
    println!("Archmage:  {is_archmage}");
    println!("Element:   {element}");

    let mana_spent = 420;
    let mana_remaining = current_mana - mana_spent;
    let recovery_rate = 15;
    let ticks_to_full = (max_mana - mana_remaining) / recovery_rate;
    println!("\nAfter casting: {mana_remaining} mana remaining");
    println!("Time to full recovery: {ticks_to_full} ticks");
}
