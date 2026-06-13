use crate::{card::Card, game::Game, rules::GameRules, state::player::PlayerId};

#[derive(Clone, Debug)]
pub struct Trick {
    cards: Vec<Card>,
    winning_player: PlayerId,
}

impl Trick {
    pub fn empty() -> Self {
        Self {
            cards: Vec::with_capacity(3),
            // SAFETY: `winning_player` is assigned by `try_play_card` and is
            //         not otherwise accessed before assignment.
            winning_player: unsafe { PlayerId::none() },
        }
    }

    pub fn leading_card(&self) -> Option<Card> {
        self.cards().first().cloned()
    }

    pub fn top_card(&self) -> Option<Card> {
        self.cards().last().cloned()
    }

    pub fn cards(&self) -> &Vec<Card> {
        &self.cards
    }

    pub fn currently_winning_player(&self) -> Option<PlayerId> {
        self.winning_player.into_option()
    }

    pub fn try_play_card(
        mut self,
        game: &Game,
        player: PlayerId,
        hand: &[Card],
        card: Card,
    ) -> (Self, PlayCardOutcome) {
        debug_assert!(self.cards.len() < 3, "overflowed the capacity of the trick");

        if self.cards.is_empty() {
            self.cards.push(card);
            self.winning_player = player;
            return (self, PlayCardOutcome::CardPlayed);
        }

        debug_assert!(!self.winning_player.is_none());

        if !game.can_play_card(&self.cards, hand, card) {
            return (self, PlayCardOutcome::InvalidCard);
        }

        if game.card_wins_trick(&self.cards, card) {
            self.winning_player = player;
        }

        self.cards.push(card);

        if self.cards.len() == 3 {
            return (
                Self::empty(),
                PlayCardOutcome::TrickComplete(WonTrick {
                    winning_player: self.winning_player,
                    cards: {
                        // This use of unsafe code is very unnecessary, but this is
                        // a toy project and it felt like a good oppotunity to
                        // practice! =)

                        // Set the length of the vector to 0 to ensure that its
                        // elements will not be dropped when the vector is dropped.
                        //
                        // SAFETY: A length of 0 is always less than or equal to the
                        //         capacity of the vector.
                        // SAFETY: 0 is always less than or equal to the previous
                        //         length, so no new elements need to be
                        //         initialised.
                        unsafe { self.cards.set_len(0) };

                        // SAFETY: A `Vec`'s pointer is always aligned properly, and
                        //         the alignment the array needs is the same as the
                        //         items.
                        // SAFETY: The length of the vector was checked above so we
                        //         know there are sufficient items in the vector to
                        //         fill the array.
                        // SAFETY: The items will not double-drop as the above
                        //         `set_len` call tells the vector not to drop the
                        //         items when it is dropped, and we are dropping
                        //         self immediately, so there is no risk of the
                        //         vector being extended again.
                        unsafe { std::ptr::read(self.cards.as_ptr() as *const [Card; 3]) }
                    },
                }),
            );
        }

        (self, PlayCardOutcome::CardPlayed)
    }
}

pub enum PlayCardOutcome {
    CardPlayed,
    InvalidCard,
    TrickComplete(WonTrick),
}

#[derive(Clone, Copy, Debug)]
pub struct WonTrick {
    pub winning_player: PlayerId,
    pub cards: [Card; 3],
}
