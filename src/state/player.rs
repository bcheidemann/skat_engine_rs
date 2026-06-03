use crate::hand::Hand;

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub hand: Hand,
}

impl PlayerState {
    pub fn new(hand: Hand) -> Self {
        Self { hand }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct PlayerId(usize);

impl PlayerId {
    /// Can be used as a placeholder when the player ID will definetly be
    /// assigned.
    const NONE: Self = Self(usize::MAX);

    pub const FIRST: Self = Self(0);
    pub const SECOND: Self = Self(1);
    pub const THIRD: Self = Self(2);

    /// Returns the [PlayerId::NONE] value. This can be used as a placeholder when
    /// the player ID will definetly be assigned but is not yet known.
    ///
    /// # SAFETY
    ///
    /// The player ID must be reassigned before it is used.
    #[inline(always)]
    pub unsafe fn none() -> Self {
        Self::NONE
    }

    #[inline(always)]
    pub fn is_none(self) -> bool {
        self == Self::NONE
    }

    #[inline(always)]
    pub fn next(self) -> Self {
        debug_assert!(self != Self::NONE, "Cannot call next on PlayerId::NONE");
        Self((self.0 + 1) % 3)
    }

    #[inline(always)]
    pub fn into_inner(self) -> usize {
        self.0
    }
}

impl From<PlayerId> for usize {
    #[inline(always)]
    fn from(value: PlayerId) -> Self {
        value.0
    }
}

impl From<usize> for PlayerId {
    #[inline(always)]
    fn from(value: usize) -> Self {
        debug_assert!(value < 3, "Player ID must be less than 3");
        Self(value)
    }
}
