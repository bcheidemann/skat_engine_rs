use std::cmp::Ordering;

use crate::game::GameKind;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rank {
    N7,
    N8,
    N9,
    N10,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    /// The number of points the card contributes to a won trick.
    #[inline(always)]
    pub fn point_value(&self) -> u8 {
        match self {
            Rank::N7 => 0,
            Rank::N8 => 0,
            Rank::N9 => 0,
            Rank::N10 => 10,
            Rank::Jack => 2,
            Rank::Queen => 3,
            Rank::King => 4,
            Rank::Ace => 11,
        }
    }

    /// Compares the rank to another rank, returning their relative ordering for
    /// the current game kind.
    #[inline(always)]
    pub fn compare(&self, other: &Rank, game_kind: GameKind) -> Ordering {
        self.raw_rank(game_kind).cmp(&other.raw_rank(game_kind))
    }

    /// Returns the rank as a number for use in ordering. This varies based on
    /// the game kind played, with null games having a different card order to
    /// suit or grand games.
    #[inline(always)]
    fn raw_rank(&self, game_kind: GameKind) -> u8 {
        match game_kind {
            GameKind::Suit | GameKind::Grand => match self {
                Rank::N7 => 0,
                Rank::N8 => 1,
                Rank::N9 => 2,
                Rank::Queen => 3,
                Rank::King => 4,
                Rank::N10 => 5,
                Rank::Ace => 6,
                Rank::Jack => 7,
            },
            GameKind::Null => match self {
                Rank::N7 => 0,
                Rank::N8 => 1,
                Rank::N9 => 2,
                Rank::N10 => 3,
                Rank::Jack => 4,
                Rank::Queen => 5,
                Rank::King => 6,
                Rank::Ace => 7,
            },
        }
    }
}

#[macro_export]
macro_rules! Rank {
    (7) => {
        $crate::rank::Rank::N7
    };
    (8) => {
        $crate::rank::Rank::N8
    };
    (9) => {
        $crate::rank::Rank::N9
    };
    (10) => {
        $crate::rank::Rank::N9
    };
    (Jack) => {
        $crate::rank::Rank::Jack
    };
    (Queen) => {
        $crate::rank::Rank::Queen
    };
    (King) => {
        $crate::rank::Rank::King
    };
    (Ace) => {
        $crate::rank::Rank::Ace
    };
}
