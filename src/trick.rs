use crate::card::Card;

#[derive(Clone, Debug)]
pub struct Trick {
    pub cards: [Option<Card>; 3],
}

#[derive(Clone, Debug)]
pub struct WonTrick {
    pub cards: [Card; 3],
}
