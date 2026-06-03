use owo_colors::OwoColorize;

use crate::{game::Game, rank::Rank, suit::Suit};

#[derive(Clone, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// Whether the card is a trump card in the current game.
    pub fn is_trump(&self, game: &Game) -> bool {
        match game {
            Game::Suit(suit_game) => self.rank == Rank::Jack || self.suit == suit_game.trump_suit,
            Game::Grand(_) => self.rank == Rank::Jack,
            Game::Null(_) => false,
        }
    }

    pub fn display_term(&self) -> String {
        let c = self.char();
        match self.suit {
            Suit::Diamonds | Suit::Hearts => c
                .if_supports_color(owo_colors::Stream::Stdout, |text| text.red())
                .to_string(),
            Suit::Spades | Suit::Clubs => c
                .if_supports_color(owo_colors::Stream::Stdout, |text| text.black())
                .to_string(),
        }
    }

    #[inline(always)]
    pub fn char(&self) -> char {
        match self.suit {
            Suit::Diamonds => match self.rank {
                Rank::N7 => '🃇',
                Rank::N8 => '🃈',
                Rank::N9 => '🃉',
                Rank::N10 => '🃊',
                Rank::Jack => '🃋',
                Rank::Queen => '🃍',
                Rank::King => '🃎',
                Rank::Ace => '🃁',
            },
            Suit::Hearts => match self.rank {
                Rank::N7 => '🂷',
                Rank::N8 => '🂸',
                Rank::N9 => '🂹',
                Rank::N10 => '🂺',
                Rank::Jack => '🂻',
                Rank::Queen => '🂽',
                Rank::King => '🂾',
                Rank::Ace => '🂱',
            },
            Suit::Spades => match self.rank {
                Rank::N7 => '🂧',
                Rank::N8 => '🂨',
                Rank::N9 => '🂩',
                Rank::N10 => '🂪',
                Rank::Jack => '🂫',
                Rank::Queen => '🂭',
                Rank::King => '🂮',
                Rank::Ace => '🂡',
            },
            Suit::Clubs => match self.rank {
                Rank::N7 => '🃗',
                Rank::N8 => '🃘',
                Rank::N9 => '🃙',
                Rank::N10 => '🃚',
                Rank::Jack => '🃛',
                Rank::Queen => '🃝',
                Rank::King => '🃞',
                Rank::Ace => '🃑',
            },
        }
    }
}

#[macro_export]
macro_rules! Card {
    ($rank:tt of $suit:tt) => {
        $crate::card::Card {
            rank: $crate::Rank!($rank),
            suit: $crate::Suit!($suit),
        }
    };
}
