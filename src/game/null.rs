use crate::{card::Card, game::GameKind, rules::GameRules};

/// A null game. There are no trumps. The objective is for the soloist to win
/// zero tricks.
#[derive(Clone, Debug)]
pub struct NullGame {}

impl NullGame {
    pub fn card_follows_suit(&self, leading_card: Card, card: Card) -> bool {
        leading_card.suit == card.suit
    }
}

impl GameRules for NullGame {
    fn can_play_card(&self, trick: &[Card], hand: &[Card], card: Card) -> bool {
        // If there is no leading card, it is legal to play the card.
        let Some(leading_card) = trick.first() else {
            return true;
        };

        // If the player follows suit, it is legal to play the card.
        if self.card_follows_suit(*leading_card, card) {
            return true;
        }

        // If the player is able to follow suit but doesn't, it is illegal to
        // play the card.
        if hand
            .iter()
            .find(|card| self.card_follows_suit(*leading_card, **card))
            .is_some()
        {
            return false;
        }

        // If the player is unable to follow suit, it is legal to play the card.
        true
    }

    fn card_wins_trick(&self, trick: &[Card], card: Card) -> bool {
        // If there is no leading card, the card is the current winner of the
        // trick.
        let Some(leading_card) = trick.first() else {
            return true;
        };

        // If the card does not follow suit, it cannot win the trick.
        if !self.card_follows_suit(*leading_card, card) {
            return false;
        }

        // If the card is a higher rank than each of the other suit-following
        // cards in the trick, then it is the current winner of the trick.
        trick
            .iter()
            .filter(|card| card.suit == leading_card.suit)
            .all(|trick_card| trick_card.rank.compare(card.rank, GameKind::Null).is_lt())
    }
}
