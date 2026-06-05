pub mod grand;

use crate::{
    card::Card,
    game::Game,
    state::player::{PlayerId, PlayerState},
    trick::{Trick, WonTrick},
};

pub struct BotContext<'a> {
    pub game: &'a Game,
    pub current_trick: &'a Trick,
    pub player_state: &'a PlayerState,
    pub tricks_won: &'a Vec<WonTrick>,
    pub soloist: PlayerId,
}

pub trait Bot {
    fn play_card(&mut self, ctx: BotContext<'_>) -> Card;
}
