use rand::{Rng, seq::SliceRandom};
use skat_engine::{DECK, card::Card, hand::Hand};

pub type Skat = [Card; 2];
pub type DealtHands = (Skat, Hand, Hand, Hand);

pub fn deal<R>(rng: &mut R) -> DealtHands
where
    R: Rng + ?Sized,
{
    let mut cards: Vec<_> = DECK.into();
    cards.shuffle(rng);
    let mut cards = cards.into_iter();

    let skat = [cards.next().unwrap(), cards.next().unwrap()];

    let mut hand1 = Hand::empty();
    let mut hand2 = Hand::empty();
    let mut hand3 = Hand::empty();

    while let Some([card_1, card_2, card_3]) = next_3_cards(&mut cards) {
        hand1.cards.push(card_1);
        hand2.cards.push(card_2);
        hand3.cards.push(card_3);
    }

    (skat, hand1, hand2, hand3)
}

fn next_3_cards(cards: &mut impl Iterator<Item = Card>) -> Option<[Card; 3]> {
    Some([cards.next()?, cards.next()?, cards.next()?])
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_invariants() {
        for _ in 0..100 {
            let (skat, hand1, hand2, hand3) = deal(&mut rand::rng());

            let mut seen_cards = HashSet::new();

            for card in skat {
                assert!(seen_cards.insert(card));
            }

            for card in hand1.cards {
                assert!(seen_cards.insert(card));
            }

            for card in hand2.cards {
                assert!(seen_cards.insert(card));
            }

            for card in hand3.cards {
                assert!(seen_cards.insert(card));
            }

            for card in DECK {
                assert!(seen_cards.contains(&card))
            }
        }
    }
}
