//! The elemental wheel (a private module of `arcana_core`).

/// The three schools of the citadel.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Fire,
    Frost,
    Arcane,
}

impl Element {
    /// Fire melts Frost, Frost dampens Arcane, Arcane smothers Fire.
    pub fn strong_against(self) -> Element {
        match self {
            Element::Fire => Element::Frost,
            Element::Frost => Element::Arcane,
            Element::Arcane => Element::Fire,
        }
    }
}
