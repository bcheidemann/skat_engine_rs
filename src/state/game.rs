use crate::{game::Game, state::player::PlayerState, trick::WonTrick};

#[derive(Clone, Debug)]
pub struct GameState {
    pub game: Game,
    pub players: [PlayerState; 3],
    pub won_tricks: Vec<WonTrick>,
}
