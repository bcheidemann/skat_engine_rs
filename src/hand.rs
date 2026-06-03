use std::cmp::Ordering;

use crate::{
    card::Card,
    game::{Game, GameKind, grand::GrandGame, null::NullGame, suit::SuitGame},
    rank::Rank,
    suit::Suit,
};

#[derive(Clone, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn empty() -> Self {
        Hand {
            cards: Vec::with_capacity(10),
        }
    }

    pub fn sorted(mut self, game: &Game) -> Self {
        self.sort(game);
        self
    }

    pub fn sort(&mut self, game: &Game) {
        match game {
            Game::Suit(suit_game) => self.cards.sort_by(make_suit_game_sorter(suit_game)),
            Game::Grand(grand_game) => self.cards.sort_by(make_grand_game_sorter(grand_game)),
            Game::Null(null_game) => self.cards.sort_by(make_null_game_sorter(null_game)),
        };
    }
}

fn make_suit_game_sorter(game: &SuitGame) -> impl Fn(&Card, &Card) -> Ordering {
    |a, b| {
        let a_is_jack = a.rank == Rank::Jack;
        let b_is_jack = b.rank == Rank::Jack;

        match (a_is_jack, b_is_jack) {
            // If both are Jacks, then order based on the value of the suit.
            (true, true) => return b.suit.base_value().cmp(&a.suit.base_value()),
            // If `a` is a Jack and `b` is not, then `a` is before `b`.
            (true, false) => return Ordering::Less,
            // If `b` is a Jack and `a` is not, then `b` is before `a`.
            (false, true) => return Ordering::Greater,
            _ => {}
        }

        let a_is_trump = a.suit == game.trump_suit;
        let b_is_trump = b.suit == game.trump_suit;

        match (a_is_trump, b_is_trump) {
            // If `a` is a trump and `b` is not, then `a` is before `b`.
            (true, false) => return Ordering::Less,
            // If `b` is a trump and `a` is not, then `b` is before `a`.
            (false, true) => return Ordering::Greater,
            _ => {}
        }

        if a.suit == b.suit {
            return b.rank.compare(&a.rank, GameKind::Grand);
        }

        black_red_suit_sorter(a.suit, b.suit)
    }
}

fn make_grand_game_sorter(_: &GrandGame) -> impl Fn(&Card, &Card) -> Ordering {
    |a, b| {
        let a_is_jack = a.rank == Rank::Jack;
        let b_is_jack = b.rank == Rank::Jack;

        match (a_is_jack, b_is_jack) {
            // If both are Jacks, then order based on the value of the suit.
            (true, true) => return b.suit.base_value().cmp(&a.suit.base_value()),
            // If `a` is a Jack and `b` is not, then `a` is before `b`.
            (true, false) => return Ordering::Less,
            // If `b` is a Jack and `a` is not, then `b` is before `a`.
            (false, true) => return Ordering::Greater,
            _ => {}
        }

        if a.suit == b.suit {
            return b.rank.compare(&a.rank, GameKind::Grand);
        }

        black_red_suit_sorter(a.suit, b.suit)
    }
}

fn make_null_game_sorter(_: &NullGame) -> impl Fn(&Card, &Card) -> Ordering {
    |a, b| {
        if a.suit == b.suit {
            return b.rank.compare(&a.rank, GameKind::Null);
        }

        black_red_suit_sorter(a.suit, b.suit)
    }
}

/// Sorts suits in the order clubs (black), then hearts (red), then spades
/// (black), then diamonds (red).
fn black_red_suit_sorter(a: Suit, b: Suit) -> Ordering {
    fn suit_idx(suit: Suit) -> u8 {
        match suit {
            Suit::Clubs => 0,
            Suit::Hearts => 1,
            Suit::Spades => 2,
            Suit::Diamonds => 3,
        }
    }
    suit_idx(a).cmp(&suit_idx(b))
}
