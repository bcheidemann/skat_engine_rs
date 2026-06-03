use crate::game::{grand::GrandGame, null::NullGame, suit::SuitGame};

pub mod grand;
pub mod null;
pub mod suit;

/// The game being played.
#[derive(Clone, Debug)]
pub enum Game {
    /// A regular suit game. All jacks and each card of the chosen suit are
    /// trumps. The objective is for the soloist to win more than half the
    /// available points.
    Suit(SuitGame),
    /// A grand game. Only jacks are trumps. The objective is for the soloist to
    /// win more than half the available points.
    Grand(GrandGame),
    /// A null game. There are no trumps. The objective is for the soloist to
    /// win zero tricks.
    Null(NullGame),
}

impl Game {
    pub fn kind(&self) -> GameKind {
        match self {
            Game::Suit(_) => GameKind::Suit,
            Game::Grand(_) => GameKind::Grand,
            Game::Null(_) => GameKind::Null,
        }
    }
}

/// The kind of game being played.
#[derive(Clone, Copy, Debug)]
pub enum GameKind {
    Suit,
    Grand,
    Null,
}
