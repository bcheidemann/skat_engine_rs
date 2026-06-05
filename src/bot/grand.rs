use std::collections::HashSet;

use crate::{
    DECK,
    bot::{Bot, BotContext},
    card::Card,
    game::Game,
    rules::GameRules,
};

pub struct GrandBot;

#[derive(Default)]
pub struct GrandBotContext;

impl GrandBot {
    /// Returns an iterator over all cards which are known to be out of play.
    /// This includes all cards already played, and the cards in the skat, if
    /// this is known to the current player (i.e. they are the soloist and are
    /// not playing a hand game).
    fn all_known_out_of_play_cards(&self, ctx: &BotContext<'_>) -> impl Iterator<Item = Card> {
        let played = ctx.tricks_won.iter().flat_map(|trick| trick.cards);
        let skat = ctx.skat.iter().flat_map(|skat| *skat);

        skat.chain(played)
    }

    /// Returns an iterator over all cards who's state is currently unknown.
    /// This includes all unplayed cards on other players hands, and the skat,
    /// if the player does not know the contents of the skat (i.e. they are a
    /// gegenspieler/defender or they are the soloist playing a hand game).
    fn all_unknown_cards(&self, ctx: &BotContext<'_>) -> Box<[Card]> {
        let out_of_play = self.all_known_out_of_play_cards(ctx);
        let all_known_cards = out_of_play.chain(ctx.player_state.hand.cards.iter().cloned());
        let all_cards = HashSet::from(DECK);

        all_cards
            .difference(&all_known_cards.collect())
            .cloned()
            .collect()
    }

    /// Searches for a card guaranteed to win the trick, based on the bots
    /// knowledge of the cards on the opponents hand.
    fn find_card_guaranteed_to_win_trick(
        &self,
        ctx: &BotContext<'_>,
        playable_cards: impl AsRef<[Card]>,
    ) -> Option<Card> {
        let all_unknown_cards = self.all_unknown_cards(ctx);
        let card_guaranteed_to_win_filter =
            make_guaranteed_win_filter(ctx.game, ctx.current_trick.cards(), &all_unknown_cards);

        playable_cards
            .as_ref()
            .iter()
            .cloned()
            .find(card_guaranteed_to_win_filter)
    }
}

impl Bot for GrandBot {
    fn play_card(&mut self, ctx: BotContext<'_>) -> Card {
        let hand = &ctx.player_state.hand;

        debug_assert!(!hand.cards.is_empty());

        let playable_cards = hand
            .cards
            .iter()
            .filter(|card| {
                ctx.game
                    .can_play_card(ctx.current_trick.cards(), &hand.cards, **card)
            })
            .cloned()
            .collect::<Box<[_]>>();

        if let Some(card) = self.find_card_guaranteed_to_win_trick(&ctx, &playable_cards) {
            return card;
        }

        if let Some(card) = playable_cards
            .iter()
            .find(|card| ctx.game.card_wins_trick(ctx.current_trick.cards(), **card))
        {
            return *card;
        }

        playable_cards
            .first()
            .cloned()
            .unwrap_or_else(|| hand.cards[0])
    }
}

fn make_guaranteed_win_filter(
    game: &Game,
    current_trick: &[Card],
    all_unknown_cards: &[Card],
) -> impl Fn(&Card) -> bool {
    move |card: &Card| {
        let trick_if_played = current_trick
            .as_ref()
            .iter()
            .cloned()
            .chain([*card])
            .collect::<Box<_>>();

        all_unknown_cards
            .iter()
            .any(|possible_card| !game.card_wins_trick(&trick_if_played, *possible_card))
    }
}
