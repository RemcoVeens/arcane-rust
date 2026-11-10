use ch02_07_project_mana_system::*;

fn book() -> Vec<Spell> {
    STARTING_SPELLS.to_vec()
}

// ── Phase 1: reading through borrows ────────────────────────────────

#[test]
fn splitting_commands_into_slices() {
    assert_eq!(split_command("  cast   Arcane Blast \n"), ("cast", "Arcane Blast"));
    assert_eq!(split_command("list\n"), ("list", ""));
    assert_eq!(split_command("   "), ("", ""));
}

#[test]
fn spell_list_of_a_borrowed_book() {
    let list = spell_list(&book());
    let lines: Vec<&str> = list.lines().collect();
    assert_eq!(lines.len(), 3, "one line per spell");
    assert_eq!(lines[0], "1. Fireball — mana 30, power 45, level 1");
    assert_eq!(lines[2], "3. Heal — mana 20, power 0, level 1");
}

#[test]
fn spell_list_works_on_any_slice() {
    assert_eq!(spell_list(&TOWER_SCROLLS[1..]), "1. Meteor — mana 90, power 150, level 10");
    assert_eq!(spell_list(&[]), "Your spellbook is empty.");
}

#[test]
fn finding_spells_in_the_book() {
    let book = book();
    assert_eq!(find_spell(&book, "FROSTBOLT"), Some(("Frostbolt", 25, 35, 2)));
    assert_eq!(find_spell(&book, "Meteor"), None, "☄️ Meteor is not in the starting book");
    assert_eq!(book.len(), 3, "📖 the book is only borrowed");
}

// ── Phase 2: mana through &mut ──────────────────────────────────────

#[test]
fn resting_and_potions() {
    let mut mana = 10;
    drink_potion(&mut mana);
    assert_eq!(mana, 60);
    drink_potion(&mut mana);
    assert_eq!(mana, MAX_MANA, "potions never overfill");
    mana = 3;
    rest(&mut mana);
    assert_eq!(mana, MAX_MANA);
}

#[test]
fn spending_mana() {
    let mut mana = 50;
    assert!(spend_mana(&mut mana, 30));
    assert_eq!(mana, 20);
    assert!(!spend_mana(&mut mana, 30), "can't spend what you don't have");
    assert_eq!(mana, 20, "a failed spend leaves the mana untouched");
}

#[test]
fn casting_spends_mana() {
    let mut mana = 100;
    let msg = cast(&book(), "fireball", &mut mana, 7);
    assert_eq!(mana, 70);
    assert!(msg.contains("Fireball"), "message should name the spell: {msg}");
    assert!(msg.contains("45"), "message should mention the damage: {msg}");
}

#[test]
fn casting_without_mana_fails() {
    let mut mana = 20;
    let msg = cast(&book(), "Frostbolt", &mut mana, 7);
    assert_eq!(mana, 20);
    assert!(msg.contains("Not enough mana"), "got: {msg}");
}

#[test]
fn casting_above_your_level_fails() {
    let mut mana = 100;
    let msg = cast(&TOWER_SCROLLS, "meteor", &mut mana, 7);
    assert_eq!(mana, 100);
    assert!(msg.to_lowercase().contains("level"), "got: {msg}");
}

#[test]
fn casting_unknown_spells_fails() {
    let mut mana = 100;
    let msg = cast(&book(), "Arcane Blast", &mut mana, 7);
    assert_eq!(mana, 100);
    assert!(msg.contains("Unknown spell"), "you haven't learned it yet! got: {msg}");
}

// ── Phase 3: scrolls & commands ─────────────────────────────────────

#[test]
fn reading_a_scroll_moves_the_spell() {
    let mut scrolls = TOWER_SCROLLS.to_vec();
    let mut book = book();
    let msg = read_scroll(&mut scrolls, &mut book);
    assert!(msg.contains("Arcane Blast"), "got: {msg}");
    assert_eq!(scrolls, vec![("Meteor", 90, 150, 10)], "the scroll is used up");
    assert_eq!(book.len(), 4);
    assert_eq!(book[3], ("Arcane Blast", 40, 60, 5), "new spells go at the end");
}

#[test]
fn no_scrolls_left() {
    let mut scrolls = Vec::new();
    let mut book = book();
    let msg = read_scroll(&mut scrolls, &mut book);
    assert!(msg.contains("No scrolls"), "got: {msg}");
    assert_eq!(book.len(), 3);
}

fn play(input: &str, mana: &mut u32, book: &mut Vec<Spell>, scrolls: &mut Vec<Spell>) -> (String, bool) {
    handle(input, mana, 7, book, scrolls)
}

#[test]
fn list_rest_and_potion_commands() {
    let (mut mana, mut book, mut scrolls) = (42, book(), TOWER_SCROLLS.to_vec());
    let (msg, go_on) = play("list\n", &mut mana, &mut book, &mut scrolls);
    assert!(go_on);
    assert_eq!(msg, spell_list(&book));

    let (msg, _) = play("  potion ", &mut mana, &mut book, &mut scrolls);
    assert_eq!(mana, 92);
    assert!(msg.contains("92"), "potion message should mention the new mana: {msg}");

    mana = 1;
    play("REST", &mut mana, &mut book, &mut scrolls);
    assert_eq!(mana, MAX_MANA);
}

#[test]
fn learning_then_casting() {
    let (mut mana, mut book, mut scrolls) = (100, book(), TOWER_SCROLLS.to_vec());
    let (msg, _) = play("cast Arcane Blast", &mut mana, &mut book, &mut scrolls);
    assert!(msg.contains("Unknown spell"), "not learned yet: {msg}");
    play("read", &mut mana, &mut book, &mut scrolls);
    let (msg, go_on) = play("CAST arcane blast\n", &mut mana, &mut book, &mut scrolls);
    assert!(go_on);
    assert_eq!(mana, 60, "got: {msg}");
}

#[test]
fn help_and_quit() {
    let (mut mana, mut book, mut scrolls) = (100, book(), TOWER_SCROLLS.to_vec());
    let (msg, go_on) = play("help", &mut mana, &mut book, &mut scrolls);
    assert!(go_on);
    for cmd in ["list", "rest", "potion", "cast", "read", "help", "quit"] {
        assert!(msg.contains(cmd), "help should mention '{cmd}'");
    }
    let (_, go_on) = play("quit", &mut mana, &mut book, &mut scrolls);
    assert!(!go_on);
}

#[test]
fn unknown_commands() {
    let (mut mana, mut book, mut scrolls) = (100, book(), TOWER_SCROLLS.to_vec());
    let (msg, go_on) = play("dance", &mut mana, &mut book, &mut scrolls);
    assert!(go_on);
    assert_eq!(mana, 100);
    assert!(msg.contains("Unknown command"), "got: {msg}");
}
