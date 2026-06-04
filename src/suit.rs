#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Suit {
    Diamonds,
    Hearts,
    Spades,
    Clubs,
}

impl Suit {
    /// The base value of the game when the suit is chosen as trumps.
    pub fn base_value(&self) -> u8 {
        match self {
            Suit::Diamonds => 9,
            Suit::Hearts => 10,
            Suit::Spades => 11,
            Suit::Clubs => 12,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Suit::Diamonds => "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Spades => "Spades",
            Suit::Clubs => "Clubs",
        }
    }
}

#[macro_export]
macro_rules! Suit {
    (Diamonds) => {
        $crate::suit::Suit::Diamonds
    };
    (Hearts) => {
        $crate::suit::Suit::Hearts
    };
    (Spades) => {
        $crate::suit::Suit::Spades
    };
    (Clubs) => {
        $crate::suit::Suit::Clubs
    };
}
