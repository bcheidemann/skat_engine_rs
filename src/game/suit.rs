use crate::suit::Suit;

/// A regular suit game. All jacks and each card of the chosen suit are trumps.
/// The objective is for the soloist to win more than half the available points.
#[derive(Clone, Debug)]
pub struct SuitGame {
    /// All jacks and each card of this suit are trups.
    pub trump_suit: Suit,
}
