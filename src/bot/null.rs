use crate::{
    bot::{Bot, BotContext},
    card::Card,
    game::GameKind,
};

pub struct NullBot;

impl Bot for NullBot {
    fn play_card(&mut self, ctx: BotContext<'_>) -> Card {
        let playable_cards = ctx.playable_cards().collect::<Box<[_]>>();
        debug_assert!(!playable_cards.is_empty());

        let can_follow_suit = if let Some(leading_card) = ctx.current_trick.leading_card() {
            // In a null game, if any playable card follows suit, then all
            // playable cards will follow suit.
            playable_cards.first().unwrap().suit == leading_card.suit
        } else {
            true
        };

        if can_follow_suit {
            playable_cards
                .iter()
                .cloned()
                .min_by(|a, b| a.rank.compare(b.rank, GameKind::Null))
                .unwrap()
        } else {
            playable_cards
                .iter()
                .cloned()
                .max_by(|a, b| a.rank.compare(b.rank, GameKind::Null))
                .unwrap()
        }
    }
}
