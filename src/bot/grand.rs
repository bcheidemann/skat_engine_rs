use crate::{
    bot::{Bot, BotContext},
    card::Card,
    rules::GameRules,
};

pub struct GrandBot;

#[derive(Default)]
pub struct GrandBotContext;

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
