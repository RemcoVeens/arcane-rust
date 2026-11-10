// 🏗️ Project 3 — Modeling Magic
// 📖 Read: book/src/03_grimoire_of_forms/07_project_modeling_magic.md
//
// The tuples are gone. Spells, elements, creatures and the mage are now
// real types. The Chapter 2 mana features are already ported to methods
// on `Mage` (rest, potions, scrolls, …) — they work.
//
// Your job: the parts marked `todo!()` — elements, creatures, casting,
// commands and the game loop. Then play:
//
//     cargo run -p ch03_07_project_modeling_magic
//
// The tests in `tests/modeling_magic.rs` check your work, phase by phase.

pub const MAX_MANA: u32 = 100;
pub const POTION_STRENGTH: u32 = 50;

// ── Phase 1: forms ──────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Element {
    Fire,
    Frost,
    Arcane,
    Life,
}

impl Element {
    /// Damage multiplier when a spell of this element hits a creature of
    /// element `defender`. Fire beats Frost, Frost beats Arcane, Arcane
    /// beats Fire:
    ///   strong pairs   (Fire → Frost, …) → 2.0
    ///   reversed pairs (Frost → Fire, …) → 0.5
    ///   everything else (same element, Life) → 1.0
    pub fn multiplier(self, defender: Element) -> f64 {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spell {
    pub name: &'static str,
    pub cost: u32,
    pub power: u32,
    pub level: u32,
    pub element: Element,
}

impl Spell {
    /// "Fireball (Fire) — mana 30, power 45, level 1"
    /// Hint: `{:?}` prints an Element's variant name.
    pub fn describe(&self) -> String {
        todo!()
    }
}

pub const STARTING_SPELLS: [Spell; 3] = [
    Spell { name: "Fireball", cost: 30, power: 45, level: 1, element: Element::Fire },
    Spell { name: "Frostbolt", cost: 25, power: 35, level: 2, element: Element::Frost },
    Spell { name: "Heal", cost: 20, power: 0, level: 1, element: Element::Life },
];

pub const TOWER_SCROLLS: [Spell; 2] = [
    Spell { name: "Arcane Blast", cost: 40, power: 60, level: 5, element: Element::Arcane },
    Spell { name: "Meteor", cost: 90, power: 150, level: 10, element: Element::Fire },
];

#[derive(Debug, Clone, PartialEq)]
pub struct Creature {
    pub name: String,
    pub hp: u32,
    pub element: Element,
    pub xp: u32,
}

impl Creature {
    pub fn new(name: &str, hp: u32, element: Element, xp: u32) -> Self {
        todo!()
    }

    pub fn is_alive(&self) -> bool {
        todo!()
    }

    /// Take a hit from a spell of `power` and `element`.
    /// damage = power × element.multiplier(self.element), rounded down.
    /// HP never goes below 0. Returns the damage.
    pub fn take_hit(&mut self, power: u32, element: Element) -> u32 {
        todo!()
    }
}

// ── Phase 2: the mage ───────────────────────────────────────────────

/// What happened when the mage tried to cast.
#[derive(Debug, Clone, PartialEq)]
pub enum CastOutcome {
    Hit { spell: &'static str, damage: u32, defeated: bool },
    UnknownSpell,
    LevelTooLow { required: u32 },
    NotEnoughMana { needed: u32, available: u32 },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Mage {
    pub name: String,
    pub level: u32,
    pub mana: u32,
    pub xp: u32,
    pub spellbook: Vec<Spell>,
    pub scrolls: Vec<Spell>,
}

impl Mage {
    /// ✅ Provided. A mage with the starting spells and the tower scrolls.
    pub fn new(name: &str, level: u32) -> Self {
        Mage {
            name: String::from(name),
            level,
            mana: MAX_MANA,
            xp: 0,
            spellbook: STARTING_SPELLS.to_vec(),
            scrolls: TOWER_SCROLLS.to_vec(),
        }
    }

    /// ✅ Provided.
    pub fn rest(&mut self) {
        self.mana = MAX_MANA;
    }

    /// ✅ Provided.
    pub fn drink_potion(&mut self) {
        self.mana = (self.mana + POTION_STRENGTH).min(MAX_MANA);
    }

    /// ✅ Provided. Spend mana if there is enough.
    pub fn spend_mana(&mut self, cost: u32) -> bool {
        if self.mana >= cost {
            self.mana -= cost;
            true
        } else {
            false
        }
    }

    /// ✅ Provided. Find a spell in the spellbook, ignoring case.
    pub fn find_spell(&self, name: &str) -> Option<Spell> {
        for spell in &self.spellbook {
            if spell.name.to_lowercase() == name.to_lowercase() {
                return Some(*spell);
            }
        }
        None
    }

    /// ✅ Provided. Move the first scroll into the spellbook.
    pub fn read_scroll(&mut self) -> Option<Spell> {
        if self.scrolls.is_empty() {
            return None;
        }
        let spell = self.scrolls.remove(0);
        self.spellbook.push(spell);
        Some(spell)
    }

    /// ✅ Provided (uses your `Spell::describe`).
    pub fn spell_list(&self) -> String {
        let mut list = String::new();
        for (i, spell) in self.spellbook.iter().enumerate() {
            if i > 0 {
                list.push('\n');
            }
            list += &format!("{}. {}", i + 1, spell.describe());
        }
        list
    }

    /// Cast a spell from the spellbook at `target`. Check, in this order:
    /// unknown spell → level too low → not enough mana. Only a successful
    /// cast spends mana and hits the target.
    pub fn cast(&mut self, name: &str, target: &mut Creature) -> CastOutcome {
        todo!()
    }
}

// ── Phase 3: commands & the game ────────────────────────────────────

/// ✅ Provided (from Chapter 2): split input into (command, argument).
pub fn split_command(input: &str) -> (&str, &str) {
    let input = input.trim();
    let (command, argument) = input.split_once(' ').unwrap_or((input, ""));
    (command, argument.trim())
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    List,
    Cast(String),
    Rest,
    Potion,
    Read,
    Look,
    Help,
    Quit,
    Unknown(String),
}

impl Command {
    /// Parse one line of input. The command word may have any
    /// capitalisation; the spell name is kept as typed.
    ///   "list" → List, "CAST arcane blast" → Cast("arcane blast"),
    ///   "rest", "potion", "read", "look", "help", "quit" → the obvious,
    ///   "cast" without a spell name, or anything else
    ///         → Unknown(the trimmed input)
    pub fn parse(input: &str) -> Command {
        todo!()
    }
}

pub struct Game {
    pub mage: Mage,
    /// The creatures in the tower; the first one is the one you fight.
    pub foes: Vec<Creature>,
    pub running: bool,
}

impl Game {
    /// ✅ Provided. Tiberius (level 7) at the foot of the tower.
    pub fn start() -> Self {
        Game {
            mage: Mage::new("Tiberius", 7),
            foes: vec![
                Creature::new("Frost Goblin", 30, Element::Frost, 20),
                Creature::new("Fire Imp", 60, Element::Fire, 35),
                Creature::new("Arcane Wraith", 90, Element::Arcane, 60),
                Creature::new("Elder Dragon", 300, Element::Fire, 200),
            ],
            running: true,
        }
    }

    /// Handle one command and return the message to print.
    ///
    ///   List   → the mage's spell list
    ///   Rest / Potion → message mentions the new mana
    ///   Read   → "You learn <spell>!" or a message containing "No scrolls"
    ///   Look   → names the current foe and its HP, e.g.
    ///            "A Frost Goblin (30 HP, Frost) blocks your path."
    ///            no foes left → message contains "quiet"
    ///   Cast(name) → no foe? message contains "nothing to fight".
    ///            Otherwise cast at the first foe:
    ///            Hit → mention the spell and the damage. If the foe is
    ///                  defeated: remove it from `foes`, add its xp to the
    ///                  mage, and mention "defeated".
    ///            UnknownSpell / LevelTooLow / NotEnoughMana → messages
    ///                  containing "Unknown spell" / "level" / "Not enough mana"
    ///   Help   → mentions every command
    ///   Quit   → sets `running` to false
    ///   Unknown(text) → message contains "Unknown command"
    pub fn handle(&mut self, command: Command) -> String {
        todo!()
    }
}
