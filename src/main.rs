fn main() {
    let mana = 100;
    let health = 75;
    let has_treasure = false;
    let level = 7;
    let found_staff = true;
    let has_key = true;

    println!("=== Status Check ===");
    if health > 0 {
        println!("You are alive.")
    } else {
        println!("Your have fallen...")
    }

    let mana_status = if mana >= 80 {
        "brimming with power"
    } else if mana >= 30 {
        "adequately charged"
    } else {
        "dangerously low"
    };
    println!("Mana levels: {mana_status}");

    if health < 30 && !has_treasure {
        println!("You should retreat and heal.")
    } else if health < 50 && has_treasure {
        println!("You have treasure! Push forward carefully.")
    } else {
        println!("You are in good shape. Continue exploring")
    }
    if has_key {
        println!("You unlock the ancient door.")
    } else {
        println!("The door is sealed. Find the key.")
    }
    let equipable = can_equip(6, level);
    println!("Can equip: {equipable}");

    let weapon = if found_staff {
        "Staff of Fire"
    } else {
        "Woden Stick"
    };
    println!("Equipped weapon: {weapon}");
}
fn can_equip(item_level: u32, player_level: u32) -> bool {
    player_level >= item_level
}
