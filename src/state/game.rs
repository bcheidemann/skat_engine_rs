use crate::{
    bot::BotContext,
    card::Card,
    game::Game,
    state::player::{PlayerId, PlayerState},
    trick::{PlayCardOutcome, Trick, WonTrick},
};

#[derive(Clone, Debug)]
pub struct GameState {
    game: Game,
    _skat: [Card; 2],
    players: [PlayerState; 3],
    tricks_won: Vec<WonTrick>,
    current_trick: Trick,
    current_player_id: PlayerId,
    soloist: PlayerId,
}

impl GameState {
    pub fn new(
        game: Game,
        skat: [Card; 2],
        players: [PlayerState; 3],
        forehand: PlayerId,
        soloist: PlayerId,
    ) -> Self {
        Self {
            game,
            _skat: skat,
            players,
            tricks_won: Vec::with_capacity(10),
            current_trick: Trick::empty(),
            current_player_id: forehand,
            soloist,
        }
    }

    pub fn last_won_trick(&self) -> Option<WonTrick> {
        self.tricks_won.last().cloned()
    }

    pub fn current_trick(&self) -> &Trick {
        &self.current_trick
    }

    pub fn player(&self, id: PlayerId) -> &PlayerState {
        &self.players[id.into_inner()]
    }

    pub fn current_player_id(&self) -> PlayerId {
        self.current_player_id
    }

    pub fn current_player(&self) -> &PlayerState {
        &self.players[self.current_player_id.into_inner()]
    }

    fn current_player_mut(&mut self) -> &mut PlayerState {
        &mut self.players[self.current_player_id.into_inner()]
    }

    pub fn get_bot_context(&self) -> BotContext<'_> {
        BotContext {
            game: &self.game,
            player_id: self.current_player_id,
            player_state: self.current_player(),
            current_trick: &self.current_trick,
            tricks_won: &self.tricks_won,
            soloist: self.soloist,
            skat: if self.current_player_id() == self.soloist {
                Some(self._skat)
            } else {
                None
            },
        }
    }

    // TODO: Return sensible result
    #[allow(clippy::result_unit_err)]
    pub fn play_card(&mut self, card: Card) -> Result<(), ()> {
        let current_player_hand = &self.current_player().hand.cards;
        let Some((played_card_idx, _)) = current_player_hand
            .iter()
            .enumerate()
            .find(|(_, card_in_hand)| **card_in_hand == card)
        else {
            return Ok(());
        };

        // SAFETY: `&self.current_trick` is valid for reads as it is non-null,
        //         it's memory range is entirely contained within the bounds of
        //         `self`, and it is only accessed from a single thread.
        // SAFETY: The compiler guarantees that `&self.current_trick` is
        //         properly aligned.
        // SAFETY: `&self.current_trick` is a properly initialised reference to
        //         a [Trick].
        let trick = unsafe { std::ptr::read(&self.current_trick as *const Trick) };

        // SAFETY: It is safe fine to move `trick` out of `&self.current_trick`
        //         temporarily, as the field is guaranteed to be reassigned
        //         before any further accesses to `self.current_trick` occur.
        let (trick, outcome) = trick.try_play_card(
            &self.game,
            self.current_player_id,
            current_player_hand,
            card,
        );

        // SAFETY: See above.
        unsafe { std::ptr::write(&mut self.current_trick, trick) };

        match outcome {
            PlayCardOutcome::CardPlayed => {
                self.current_player_mut().hand.cards.remove(played_card_idx);
                self.current_player_id = self.current_player_id.next();

                Ok(())
            }
            PlayCardOutcome::InvalidCard => Err(()),
            PlayCardOutcome::TrickComplete(won_trick) => {
                self.current_player_mut().hand.cards.remove(played_card_idx);
                self.current_player_id = won_trick.winning_player;
                self.tricks_won.push(won_trick);

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Card,
        game::{grand::GrandGame, suit::SuitGame},
        suit::Suit,
    };

    use super::*;

    #[test]
    pub fn test_grand() {
        let skat = [Card!(7 of Spades), Card!(King of Spades)];
        let hand1 = [
            Card!(Jack of Clubs),
            Card!(Ace of Clubs),
            Card!(King of Clubs),
            Card!(8 of Clubs),
            Card!(Ace of Hearts),
            Card!(King of Hearts),
            Card!(9 of Hearts),
            Card!(Queen of Spades),
            Card!(9 of Spades),
            Card!(9 of Diamonds),
        ];
        let hand2 = [
            Card!(Jack of Hearts),
            Card!(Jack of Diamonds),
            Card!(9 of Clubs),
            Card!(9 of Hearts),
            Card!(8 of Hearts),
            Card!(Ace of Spades),
            Card!(8 of Spades),
            Card!(King of Diamonds),
            Card!(Queen of Diamonds),
            Card!(8 of Diamonds),
        ];
        let hand3 = [
            Card!(Jack of Spades),
            Card!(Queen of Clubs),
            Card!(10 of Clubs),
            Card!(7 of Clubs),
            Card!(Queen of Hearts),
            Card!(7 of Hearts),
            Card!(9 of Spades),
            Card!(Ace of Diamonds),
            Card!(9 of Diamonds),
            Card!(7 of Diamonds),
        ];
        let mut game_state = GameState::new(
            Game::Grand(GrandGame {}),
            skat,
            [
                PlayerState::new(hand1.into()),
                PlayerState::new(hand2.into()),
                PlayerState::new(hand3.into()),
            ],
            PlayerId::FIRST,
            PlayerId::FIRST,
        );

        assert_eq!(game_state.current_player_id(), PlayerId::FIRST);
        assert_eq!(game_state.current_trick.cards().len(), 0);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 10);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 10);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 10);

        game_state.play_card(Card!(Jack of Clubs)).unwrap();
        assert_eq!(game_state.current_player_id(), PlayerId::SECOND);
        assert_eq!(game_state.current_trick.cards().len(), 1);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 10);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 10);

        game_state.play_card(Card!(Jack of Diamonds)).unwrap();
        assert_eq!(game_state.current_player_id(), PlayerId::THIRD);
        assert_eq!(game_state.current_trick.cards().len(), 2);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 10);

        game_state.play_card(Card!(Jack of Spades)).unwrap();
        assert_eq!(game_state.current_player_id(), PlayerId::FIRST);
        assert_eq!(game_state.current_trick.cards().len(), 0);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 9);

        game_state.play_card(Card!(8 of Clubs)).unwrap();
        assert_eq!(game_state.current_player_id(), PlayerId::SECOND);
        assert_eq!(game_state.current_trick.cards().len(), 1);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 8);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 9);

        game_state.play_card(Card!(9 of Clubs)).unwrap();
        assert_eq!(game_state.current_player_id(), PlayerId::THIRD);
        assert_eq!(game_state.current_trick.cards().len(), 2);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 8);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 8);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 9);

        game_state.play_card(Card!(10 of Clubs)).unwrap();
        assert_eq!(game_state.current_player_id(), PlayerId::THIRD); // Player 3 won the trick
        assert_eq!(game_state.current_trick.cards().len(), 0);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 8);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 8);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 8);
    }

    /// See https://github.com/bcheidemann/skat_engine_rs/issues/1
    #[test]
    pub fn regression_1_grand() {
        let skat = [Card!(Jack of Clubs), Card!(7 of Clubs)];
        let hand1 = [
            Card!(Jack of Hearts),
            Card!(Ace of Diamonds),
            Card!(10 of Diamonds),
            Card!(King of Diamonds),
            Card!(Queen of Diamonds),
            Card!(9 of Diamonds),
            Card!(8 of Diamonds),
            Card!(7 of Diamonds),
            Card!(Ace of Clubs),
            Card!(10 of Clubs),
        ];
        let hand2 = [
            Card!(Jack of Diamonds),
            Card!(Ace of Hearts),
            Card!(10 of Hearts),
            Card!(King of Hearts),
            Card!(Queen of Hearts),
            Card!(9 of Hearts),
            Card!(8 of Hearts),
            Card!(7 of Hearts),
            Card!(King of Clubs),
            Card!(Queen of Clubs),
        ];
        let hand3 = [
            Card!(Jack of Spades),
            Card!(Ace of Spades),
            Card!(10 of Spades),
            Card!(King of Spades),
            Card!(Queen of Spades),
            Card!(9 of Spades),
            Card!(8 of Spades),
            Card!(7 of Spades),
            Card!(9 of Clubs),
            Card!(8 of Clubs),
        ];
        let mut game_state = GameState::new(
            Game::Grand(GrandGame {}),
            skat,
            [
                PlayerState::new(hand1.into()),
                PlayerState::new(hand2.into()),
                PlayerState::new(hand3.into()),
            ],
            PlayerId::FIRST,
            PlayerId::FIRST,
        );

        game_state.play_card(Card!(Ace of Diamonds)).unwrap();
        game_state
            .play_card(Card!(Ace of Hearts))
            .expect("should be allowed to break suit, since they only have a Jack in Diamonds");
    }

    /// See https://github.com/bcheidemann/skat_engine_rs/issues/1
    #[test]
    pub fn regression_1_suit() {
        let skat = [Card!(Jack of Clubs), Card!(7 of Clubs)];
        let hand1 = [
            Card!(Jack of Hearts),
            Card!(Ace of Diamonds),
            Card!(10 of Diamonds),
            Card!(King of Diamonds),
            Card!(Queen of Diamonds),
            Card!(9 of Diamonds),
            Card!(8 of Diamonds),
            Card!(7 of Diamonds),
            Card!(Ace of Clubs),
            Card!(10 of Clubs),
        ];
        let hand2 = [
            Card!(Jack of Diamonds),
            Card!(Ace of Hearts),
            Card!(10 of Hearts),
            Card!(King of Hearts),
            Card!(Queen of Hearts),
            Card!(9 of Hearts),
            Card!(8 of Hearts),
            Card!(7 of Hearts),
            Card!(King of Clubs),
            Card!(Queen of Clubs),
        ];
        let hand3 = [
            Card!(Jack of Spades),
            Card!(Ace of Spades),
            Card!(10 of Spades),
            Card!(King of Spades),
            Card!(Queen of Spades),
            Card!(9 of Spades),
            Card!(8 of Spades),
            Card!(7 of Spades),
            Card!(9 of Clubs),
            Card!(8 of Clubs),
        ];
        let mut game_state = GameState::new(
            Game::Suit(SuitGame {
                trump_suit: Suit::Clubs,
            }),
            skat,
            [
                PlayerState::new(hand1.into()),
                PlayerState::new(hand2.into()),
                PlayerState::new(hand3.into()),
            ],
            PlayerId::FIRST,
            PlayerId::FIRST,
        );

        game_state.play_card(Card!(Ace of Diamonds)).unwrap();
        game_state
            .play_card(Card!(Ace of Hearts))
            .expect("should be allowed to break suit, since they only have a Jack in Diamonds");
    }

    /// See https://github.com/bcheidemann/skat_engine_rs/issues/2
    #[test]
    pub fn regression_2_grand() {
        let skat = [Card!(Jack of Hearts), Card!(7 of Clubs)];
        let hand1 = [
            Card!(Jack of Clubs),
            Card!(Ace of Spades),
            Card!(Jack of Diamonds),
            Card!(10 of Diamonds),
            Card!(King of Diamonds),
            Card!(Queen of Diamonds),
            Card!(9 of Diamonds),
            Card!(8 of Diamonds),
            Card!(7 of Diamonds),
            Card!(10 of Clubs),
        ];
        let hand2 = [
            Card!(Jack of Spades),
            Card!(King of Clubs),
            Card!(Queen of Clubs),
            Card!(Ace of Hearts),
            Card!(10 of Hearts),
            Card!(King of Hearts),
            Card!(Queen of Hearts),
            Card!(9 of Hearts),
            Card!(8 of Hearts),
            Card!(7 of Hearts),
        ];
        let hand3 = [
            Card!(Ace of Diamonds),
            Card!(10 of Spades),
            Card!(King of Spades),
            Card!(Queen of Spades),
            Card!(9 of Spades),
            Card!(8 of Spades),
            Card!(7 of Spades),
            Card!(Ace of Clubs),
            Card!(9 of Clubs),
            Card!(8 of Clubs),
        ];
        let mut game_state = GameState::new(
            Game::Grand(GrandGame {}),
            skat,
            [
                PlayerState::new(hand1.into()),
                PlayerState::new(hand2.into()),
                PlayerState::new(hand3.into()),
            ],
            PlayerId::FIRST,
            PlayerId::FIRST,
        );

        game_state.play_card(Card!(Jack of Clubs)).unwrap();
        assert!(
            game_state.play_card(Card!(King of Clubs)).is_err(),
            "should not be allowed to play King of Clubs because player has a Jack of Spades"
        );
        game_state
            .play_card(Card!(Jack of Spades))
            .expect("should be allowed to play Jack of Spades as leading card is also a Jack");
        game_state
            .play_card(Card!(Ace of Diamonds))
            .expect("should be allowed to any card as the player does not have a Jack");
    }

    /// See https://github.com/bcheidemann/skat_engine_rs/issues/2
    #[test]
    pub fn regression_2_suit() {
        let skat = [Card!(Jack of Hearts), Card!(7 of Clubs)];
        let hand1 = [
            Card!(Jack of Clubs),
            Card!(Ace of Spades),
            Card!(Jack of Diamonds),
            Card!(10 of Diamonds),
            Card!(King of Diamonds),
            Card!(Queen of Diamonds),
            Card!(9 of Diamonds),
            Card!(8 of Diamonds),
            Card!(7 of Diamonds),
            Card!(10 of Clubs),
        ];
        let hand2 = [
            Card!(Jack of Spades),
            Card!(King of Clubs),
            Card!(Queen of Clubs),
            Card!(Ace of Hearts),
            Card!(10 of Hearts),
            Card!(King of Hearts),
            Card!(Queen of Hearts),
            Card!(9 of Hearts),
            Card!(8 of Hearts),
            Card!(7 of Hearts),
        ];
        let hand3 = [
            Card!(Ace of Diamonds),
            Card!(10 of Spades),
            Card!(King of Spades),
            Card!(Queen of Spades),
            Card!(9 of Spades),
            Card!(8 of Spades),
            Card!(7 of Spades),
            Card!(Ace of Clubs),
            Card!(9 of Clubs),
            Card!(8 of Clubs),
        ];
        let mut game_state = GameState::new(
            Game::Suit(SuitGame {
                trump_suit: Suit::Hearts,
            }),
            skat,
            [
                PlayerState::new(hand1.into()),
                PlayerState::new(hand2.into()),
                PlayerState::new(hand3.into()),
            ],
            PlayerId::FIRST,
            PlayerId::FIRST,
        );

        game_state.play_card(Card!(Jack of Clubs)).unwrap();
        assert!(
            game_state.play_card(Card!(King of Clubs)).is_err(),
            "should not be allowed to play King of Clubs because player has a Jack of Spades"
        );
        game_state
            .play_card(Card!(Jack of Spades))
            .expect("should be allowed to play Jack of Spades as leading card is also a Jack");
        game_state.play_card(Card!(Ace of Diamonds)).expect(
            "should be allowed to any card as the player does not have a Jack or any other trumps",
        );
    }
}
