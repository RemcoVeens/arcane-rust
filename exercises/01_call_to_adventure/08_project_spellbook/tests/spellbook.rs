use ch01_08_project_spellbook::*;

// ── Phase 1: the grimoire ────────────────────────────────────────────

#[test]
fn banner_names_the_grimoire() {
    assert!(banner().contains("ARCANE GRIMOIRE"));
}

#[test]
fn finds_spells_by_exact_name() {
    assert_eq!(find_spell("Fireball"), Some(("Fireball", 30, 45, 1)));
}

#[test]
fn finds_spells_ignoring_case() {
    assert_eq!(find_spell("arcane blast"), Some(("Arcane Blast", 40, 60, 5)));
    assert_eq!(find_spell("FROSTBOLT"), Some(("Frostbolt", 25, 35, 2)));
}

#[test]
fn unknown_spells_are_none() {
    assert_eq!(find_spell("Summon Pizza"), None);
}

#[test]
fn spell_list_is_numbered() {
    let list = spell_list();
    let lines: Vec<&str> = list.lines().collect();
    assert_eq!(lines.len(), 5, "one line per spell");
    assert_eq!(lines[0], "1. Fireball — mana 30, power 45, level 1");
    assert_eq!(lines[4], "5. Meteor — mana 90, power 150, level 10");
}

// ── Phase 2: mana ────────────────────────────────────────────────────

#[test]
fn potions_restore_mana() {
    assert_eq!(drink_potion(10), 60);
}

#[test]
fn potions_never_overfill() {
    assert_eq!(drink_potion(80), MAX_MANA);
}

#[test]
fn casting_spends_mana() {
    let (mana, msg) = cast("fireball", 100, 7);
    assert_eq!(mana, 70);
    assert!(msg.contains("Fireball"), "message should name the spell: {msg}");
    assert!(msg.contains("45"), "message should mention the damage: {msg}");
}

#[test]
fn casting_without_mana_fails() {
    let (mana, msg) = cast("Arcane Blast", 20, 7);
    assert_eq!(mana, 20);
    assert!(msg.contains("Not enough mana"), "got: {msg}");
}

#[test]
fn casting_above_your_level_fails() {
    let (mana, msg) = cast("meteor", 100, 7);
    assert_eq!(mana, 100);
    assert!(msg.to_lowercase().contains("level"), "got: {msg}");
}

#[test]
fn casting_unknown_spells_fails() {
    let (mana, msg) = cast("Summon Pizza", 100, 7);
    assert_eq!(mana, 100);
    assert!(msg.contains("Unknown spell"), "got: {msg}");
}

// ── Phase 3: commands ────────────────────────────────────────────────

#[test]
fn list_command() {
    let (mana, msg, go_on) = handle("list\n", 42, 7);
    assert_eq!((mana, go_on), (42, true));
    assert_eq!(msg, spell_list());
}

#[test]
fn rest_command() {
    let (mana, _, go_on) = handle("rest", 3, 7);
    assert_eq!((mana, go_on), (MAX_MANA, true));
}

#[test]
fn potion_command() {
    let (mana, _, _) = handle("  potion  ", 30, 7);
    assert_eq!(mana, 80);
}

#[test]
fn cast_command() {
    let (mana, msg, go_on) = handle("cast frostbolt\n", 100, 7);
    assert_eq!((mana, go_on), (75, true));
    assert!(msg.contains("Frostbolt"), "got: {msg}");
}

#[test]
fn commands_ignore_case() {
    let (mana, _, _) = handle("CAST Heal", 100, 7);
    assert_eq!(mana, 80);
}

#[test]
fn help_mentions_every_command() {
    let (_, msg, go_on) = handle("help", 100, 7);
    assert!(go_on);
    for cmd in ["list", "rest", "potion", "cast", "help", "quit"] {
        assert!(msg.contains(cmd), "help should mention '{cmd}'");
    }
}

#[test]
fn quit_command_stops_the_game() {
    let (_, _, go_on) = handle("quit", 100, 7);
    assert!(!go_on);
}

#[test]
fn unknown_commands() {
    let (mana, msg, go_on) = handle("dance", 100, 7);
    assert_eq!((mana, go_on), (100, true));
    assert!(msg.contains("Unknown command"), "got: {msg}");
}
