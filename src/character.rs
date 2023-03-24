use crate::card::HogwartsCard;
use crate::effect::Effect;

use rand::seq::SliceRandom;

pub trait Character {}

#[derive(Debug, Clone)]
pub struct Hero {
    health: u8,
    coins: u8,
    power: u8,
    deck: Vec<HogwartsCard>,
    hand: Vec<HogwartsCard>,
    discard: Vec<HogwartsCard>,
    special_power: Option<Effect>,
    id: usize,
}

impl PartialEq for Hero {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Character for Hero {}

impl Hero {
    pub fn damage_hero(&mut self, amount: u8) -> Result<(), ()> {
        self.health = self.health.saturating_sub(amount);
        if self.is_stunned() {
            self.stun()
        } else {
            Ok(())
        }
    }

    #[cfg(feature = "random")]
    fn stun(&mut self) -> Result<(), ()> {
        let to_discard = self.hand.len() / 2;
        self.discard_random_cards(to_discard)
    }

    #[cfg(all(feature = "cli", not(feature = "random")))]
    fn stun(&mut self) -> Result<(), ()> {
        let to_discard = self.hand.len() / 2;
        self.discard_cards_user(to_discard)
    }

    pub fn is_stunned(&self) -> bool {
        self.health <= 0
    }

    pub fn discard_card(&mut self, card: HogwartsCard) -> Result<(), ()> {
        if !self.hand.contains(&card) {
            // The card has already been popped from the hand.
            return Ok(());
        }

        if let Some(index) = self.hand.iter().position(|r| r == &card) {
            self.discard.extend_from_slice(&[card]);
            self.hand.remove(index);
            Ok(())
        } else {
            Err(())
        }
    }

    #[cfg(feature = "random")]
    pub fn discard_random_cards(&mut self, amount: usize) -> Result<(), ()> {
        let mut rng = rand::thread_rng();
        let cards: Vec<_> = self
            .hand
            .choose_multiple(&mut rng, amount)
            .cloned()
            .collect();
        for card in cards {
            self.discard_card(card)?
        }
        Ok(())
    }

    pub fn draw_card(&mut self) -> Result<(), ()> {
        if self.deck.len() < 1 {
            self.shuffle_deck();
        }
        if let Some(to_draw) = self.deck.pop() {
            self.hand.extend_from_slice(&[to_draw]);
            Ok(())
        } else {
            // TODO: decide what to do with pathological case
            todo!()
        }
    }

    fn shuffle_deck(&mut self) {
        let mut rng = rand::thread_rng();
        self.discard.shuffle(&mut rng);
        self.deck = self.discard.clone();
        self.discard.clear();
    }

}

#[derive(Debug, Clone, Copy)]
pub struct Villain {
    health: u8,
    current_damage: u8,
    effect: Effect,
    reward: Effect,
    id: usize,
}

impl PartialEq for Villain {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
