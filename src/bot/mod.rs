pub mod grand;
pub mod null;

use std::collections::HashSet;

use crate::{
    ALL_CARDS_SET,
    card::Card,
    game::Game,
    rules::GameRules as _,
    state::player::{PlayerId, PlayerState},
    trick::{Trick, WonTrick},
};

pub trait Bot {
    /// Called on the bots turn when it is required to play a card.
    ///
    /// # Implementation
    ///
    /// Implementors must ensure that the returned card is legally playable.
    fn play_card(&mut self, ctx: BotContext<'_>) -> Card;
}

pub struct BotContext<'a> {
    /// The details of the game being played.
    pub game: &'a Game,
    /// The ID of the current player.
    pub player_id: PlayerId,
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

impl BotContext<'_> {
    /// Return `true` if the player the bot is representing is the soloist.
    pub fn is_soloist(&self) -> bool {
        self.player_id == self.soloist
    }

    /// Return an iterator over all the cards on the players current hand which
    /// are legally playable.
    pub fn playable_cards(&self) -> impl Iterator<Item = Card> {
        let hand = &self.player_state.hand;
        hand.cards
            .iter()
            .filter(|card| {
                self.game
                    .can_play_card(self.current_trick.cards(), &hand.cards, **card)
            })
            .cloned()
    }

    /// Returns an iterator over all cards which are known to be out of play.
    /// This includes all cards already played, and the cards in the skat, if
    /// this is known to the current player (i.e. they are the soloist and are
    /// not playing a hand game).
    pub fn all_known_out_of_play_cards(&self) -> impl Iterator<Item = Card> + use<'_> {
        let played = self.tricks_won.iter().flat_map(|trick| trick.cards);
        let skat = self.skat.iter().flat_map(|skat| *skat);

        skat.chain(played)
    }

    /// Returns an iterator over all cards who's state is currently unknown.
    /// This includes all unplayed cards on other players hands, and the skat,
    /// if the player does not know the contents of the skat (i.e. they are a
    /// gegenspieler/defender or they are the soloist playing a hand game).
    pub fn all_unknown_cards(&self) -> impl Iterator<Item = Card> {
        let known: HashSet<Card> = self
            .all_known_out_of_play_cards()
            .chain(self.player_state.hand.cards.iter().cloned())
            .collect();

        ALL_CARDS_SET
            .iter()
            .cloned()
            .filter(move |card| !known.contains(card))
    }

    /// Searches for a card guaranteed to win the trick, based on the bots
    /// knowledge of the cards on the opponents hand.
    pub fn find_card_guaranteed_to_win_trick(&self) -> Box<[Card]> {
        let all_unknown_cards: Box<[_]> = self.all_unknown_cards().collect();

        self.playable_cards()
            .filter(|card: &Card| {
                let trick_if_played = self
                    .current_trick
                    .cards()
                    .iter()
                    .cloned()
                    .chain([*card])
                    .collect::<Box<_>>();

                all_unknown_cards.iter().any(|possible_card| {
                    !self.game.card_wins_trick(&trick_if_played, *possible_card)
                })
            })
            .collect()
    }
}
