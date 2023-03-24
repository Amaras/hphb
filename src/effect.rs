// use crate::character;
use crate::target::Target;

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) enum Effect {
    // Villains may control (+) and heroes may heal (-)
    LocationControl { amount: i8 },
    // Villains may get healed (-) and heroes may damage villains (+)
    VillainDamage { amount: i8 },
    // Heroes may gain power (+) or use/discard some for effects (-)
    PowerGain { amount: i8 },
    // Heroes may heal (+) or get damaged (-)
    HeroHealth { amount: i8 },
    // Heroes may gain influence (+) and use/discard some for effects (-)
    InfluenceGain { amount: i8 },
    // Heroes may draw cards (+) or discard some (-)
    Cards { amount: i8 },
}

impl Effect {
    pub fn apply(&self, _target: &mut Target) -> Result<(), ()> {
        Ok(())
    }
}
