use std::ops::{Deref, DerefMut};

pub struct GameValue(u16);

impl GameValue {
    // Suit games
    pub const DIAMONDS: Self = Self(9);
    pub const HEARTS: Self = Self(10);
    pub const SPADES: Self = Self(11);
    pub const CLUBS: Self = Self(12);

    // Grand
    pub const GRAND: Self = Self(24);

    // Null
    pub const NULL: Self = Self(23);
    pub const NULL_HAND: Self = Self(35);
    pub const NULL_OVERT: Self = Self(46);
    pub const NULL_OVERT_HAND: Self = Self(59);
}

impl Deref for GameValue {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GameValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
