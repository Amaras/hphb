use crate::character::Character;
use crate::effect::Effect;
use crate::target::Target;

#[derive(Debug, Clone, PartialEq)]
pub enum Card {
    Hogwarts(HogwartsCard),
    //DarksArts(DarkArtsCard),
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CardType {
    Ally,
    Spell,
    Object,
}

#[derive(Debug, Clone)]
pub struct HogwartsCard {
    card_type: CardType,
    cost: u8,
    id: usize,
    play_effects: Vec<Effect>,
    discard_effects: Vec<Effect>,
    target: Target,
}

impl PartialEq for HogwartsCard {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl HogwartsCard {
    fn play(&mut self, targets: &mut [&mut Target]) -> Result<(), ()> {
        for effect in &self.play_effects {
            for target in targets.iter_mut() {
                effect.apply(target)?
            }
        }
        Ok(())
    }

    fn discard(&mut self, targets: &mut [&mut Target]) -> Result<(), ()> {
        for effect in &self.discard_effects {
            for target in targets.iter_mut() {
                effect.apply(target)?
            }
        }
        Ok(())
    }
}
