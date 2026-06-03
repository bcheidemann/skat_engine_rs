use crate::{
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
    soloist_tricks: Vec<WonTrick>,
    defender_tricks: Vec<WonTrick>,
    current_trick: Trick,
    current_player: PlayerId,
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
            soloist_tricks: Vec::with_capacity(10),
            defender_tricks: Vec::with_capacity(10),
            current_trick: Trick::empty(),
            current_player: forehand,
            soloist,
        }
    }

    pub fn player(&self, id: PlayerId) -> &PlayerState {
        &self.players[id.into_inner()]
    }

    pub fn current_player(&self) -> &PlayerState {
        &self.players[self.current_player.into_inner()]
    }

    fn current_player_mut(&mut self) -> &mut PlayerState {
        &mut self.players[self.current_player.into_inner()]
    }

    // TODO: Return sensible result
    #[allow(clippy::result_unit_err)]
    pub fn play_card(&mut self, card_idx: usize) -> Result<(), ()> {
        let played_card = self.current_player().hand.cards[card_idx];

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
        let (trick, outcome) = trick.try_play_card(&self.game, self.current_player, played_card);

        // SAFETY: See above.
        unsafe { std::ptr::write(&mut self.current_trick, trick) };

        match outcome {
            PlayCardOutcome::CardPlayed => {}
            PlayCardOutcome::InvalidCard => return Err(()),
            PlayCardOutcome::TrickComplete(won_trick) => {
                if won_trick.winning_player == self.soloist {
                    self.soloist_tricks.push(won_trick);
                } else {
                    self.defender_tricks.push(won_trick);
                }
            }
        }

        self.current_player_mut().hand.cards.remove(card_idx);

        self.current_player = self.current_player.next();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Card, game::grand::GrandGame};

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
            Card!(9 of Clubs),
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

        assert_eq!(game_state.current_trick.cards().len(), 0);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 10);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 10);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 10);

        game_state.play_card(0).unwrap();
        assert_eq!(game_state.current_trick.cards().len(), 1);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 10);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 10);

        game_state.play_card(0).unwrap();
        assert_eq!(game_state.current_trick.cards().len(), 2);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 10);

        game_state.play_card(0).unwrap();
        assert_eq!(game_state.current_trick.cards().len(), 0);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 9);

        game_state.play_card(0).unwrap();
        assert_eq!(game_state.current_trick.cards().len(), 1);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 8);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 9);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 9);

        game_state.play_card(0).unwrap();
        assert_eq!(game_state.current_trick.cards().len(), 2);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 8);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 8);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 9);

        game_state.play_card(0).unwrap();
        assert_eq!(game_state.current_trick.cards().len(), 0);
        assert_eq!(game_state.player(PlayerId::FIRST).hand.cards.len(), 8);
        assert_eq!(game_state.player(PlayerId::SECOND).hand.cards.len(), 8);
        assert_eq!(game_state.player(PlayerId::THIRD).hand.cards.len(), 8);

        panic!("{game_state:#?}");
    }
}
