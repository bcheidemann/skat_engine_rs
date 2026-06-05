pub mod grand;

use crate::{
    card::Card,
    game::Game,
    state::player::{PlayerId, PlayerState},
    trick::{Trick, WonTrick},
};

pub struct BotContext<'a> {
    /// The details of the game being played.
    pub game: &'a Game,
    /// The state of the player, including their hand.
    pub player_state: &'a PlayerState,
    /// All cards which have been played in the current round.
    pub current_trick: &'a Trick,
    /// All tricks which have been won during the game.
    pub tricks_won: &'a Vec<WonTrick>,
    /// The player who is playing as a soloist.
    pub soloist: PlayerId,
    /// The contents of the skat, if it is known to the player. This may not be
    /// be known, for example if the bot is a gegenspieler/defender, or is
    /// playing a hand game.
    pub skat: Option<[Card; 2]>,
}

pub trait Bot {
    fn play_card(&mut self, ctx: BotContext<'_>) -> Card;
}
