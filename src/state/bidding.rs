use crate::state::player::PlayerId;

#[derive(Clone, Debug)]
pub struct BiddingState {
    /// The player who dealt the hand.
    pub dealer_id: PlayerId,
    /// The current bidding step.
    current_bidding_step: usize,
}

impl BiddingState {
    pub fn current_bidding_value(&self) -> u16 {
        ALL_POSSIBLE_BIDDING_VALUES[self.current_bidding_step]
    }
}

pub static ALL_POSSIBLE_BIDDING_VALUES: &[u16] = &[
    18, 20, 22, 23, 24, 27, 30, 33, 35, 36, 40, 44, 45, 46, 48, 50, 54, 55, 59, 60, 63, 66, 70, 72,
    77, 80, 81, 84, 88, 90, 96, 99, 100, 108, 110, 117, 120, 121, 126, 130, 132, 135, 140, 143,
    144, 150, 153, 154, 156, 160, 162, 165, 168, 170, 176, 180, 187, 192, 198, 204, 216, 240, 264,
];

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::game::value::GameValue;

    use super::*;

    #[test]
    fn test_all_possible_bidding_values() {
        // game level:
        //     0
        //   + 1 (for playing)
        //   + 1-4[Grand] / 1-11[Suit] (for matadors)
        //   + 1 (for hand)
        //   + 1 (for schneider achieved)
        //   + 1 (for schneider announced)
        //   + 1 (for schwartz achieved)
        //   + 1 (for schwartz announced)
        //   + 1 (for overt)
        //   = 2-11[Grand] / 2-18[Suit]

        let values = {
            let mut values = HashSet::new();

            for game_level in 2u16..=18 {
                for game_value in [
                    GameValue::DIAMONDS,
                    GameValue::HEARTS,
                    GameValue::SPADES,
                    GameValue::CLUBS,
                ] {
                    values.insert(*game_value * game_level);
                }
            }

            for game_level in 2u16..=11 {
                values.insert(*GameValue::GRAND * game_level);
            }

            values.insert(*GameValue::NULL);
            values.insert(*GameValue::NULL_HAND);
            values.insert(*GameValue::NULL_OVERT);
            values.insert(*GameValue::NULL_OVERT_HAND);

            let mut values: Box<_> = values.iter().cloned().collect();

            values.sort();

            values
        };

        assert_eq!(*values, *ALL_POSSIBLE_BIDDING_VALUES);
    }
}
