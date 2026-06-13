use crate::{
    card::Card,
    game::{grand::GrandGame, null::NullGame, suit::SuitGame},
    rules::GameRules,
    state::game::GameState,
};

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

impl GameRules for Game {
    fn can_play_card(&self, trick: &[Card], hand: &[Card], card: Card) -> bool {
        match self {
            Game::Suit(suit_game) => suit_game.can_play_card(trick, hand, card),
            Game::Grand(grand_game) => grand_game.can_play_card(trick, hand, card),
            Game::Null(null_game) => null_game.can_play_card(trick, hand, card),
        }
    }

    fn card_wins_trick(&self, trick: &[Card], card: Card) -> bool {
        match self {
            Game::Suit(suit_game) => suit_game.card_wins_trick(trick, card),
            Game::Grand(grand_game) => grand_game.card_wins_trick(trick, card),
            Game::Null(null_game) => null_game.card_wins_trick(trick, card),
        }
    }

    fn is_game_over(&self, game_state: &GameState) -> bool {
        match self {
            Game::Suit(suit_game) => suit_game.is_game_over(game_state),
            Game::Grand(grand_game) => grand_game.is_game_over(game_state),
            Game::Null(null_game) => null_game.is_game_over(game_state),
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
