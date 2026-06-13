use crate::card::Card;

pub trait GameRules {
    /// Returns `true` if the card is allowed to be played on the trick.
    fn can_play_card(&self, trick: &[Card], hand: &[Card], card: Card) -> bool;

    /// Returns `true` if the card would be the current winning card, if played.
    /// This does not necessarily guarantee that the card _will_ win the trick.
    fn card_wins_trick(&self, trick: &[Card], card: Card) -> bool;
}
