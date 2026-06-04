use crate::{card::Card, game::GameKind, rank::Rank, rules::GameRules};

/// A grand game. Only jacks are trumps. The objective is for the soloist to win
/// more than half the available points.
#[derive(Clone, Debug)]
pub struct GrandGame {}

impl GrandGame {
    pub fn card_is_trump(&self, card: Card) -> bool {
        card.rank == Rank::Jack
    }

    pub fn card_follows_suit(&self, leading_card: Card, card: Card) -> bool {
        if self.card_is_trump(leading_card) {
            self.card_is_trump(card)
        } else {
            card.suit == leading_card.suit
        }
    }
}

impl GameRules for GrandGame {
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

        let card_follows_suit = self.card_follows_suit(*leading_card, card);
        let card_is_trump = self.card_is_trump(card);

        // If the card does not follow suit and is not trump, then it cannot
        // win the trick.
        if !card_follows_suit && !card_is_trump {
            return false;
        }

        // If the card is higher (or trumps) every other card already in the
        // trick, then it is the current winner of the trick.
        trick
            .iter()
            // Exclude all cards which don't follow suit and are not trump, as
            // these cannot win the trick.
            .filter(|card| {
                self.card_is_trump(**card) || self.card_follows_suit(*leading_card, **card)
            })
            .all(
                |trick_card| match (self.card_is_trump(*trick_card), card_is_trump) {
                    // If both cards are trump (Jacks) then the Jack with the
                    // higher base value wins.
                    (true, true) => trick_card.suit.base_value() < card.suit.base_value(),
                    // If the trick card is trump (Jack) and the played card
                    // isn't, then the played card looses.
                    (true, false) => false,
                    // If the trick card is not trump (Jack) but the played card
                    // is, then the played card wins.
                    (false, true) => true,
                    // If neither card are tump, then the one with the higher
                    // rank wins.
                    (false, false) => trick_card.rank.compare(card.rank, GameKind::Grand).is_lt(),
                },
            )
    }
}
