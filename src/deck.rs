use crate::card::Card;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards: Vec<Card> = vec![];
        let suits = ["H", "D", "C", "S"];
        let faces = ["A", "K", "Q", "J"];

        for s in suits {
            for f in faces {
                cards.push(Card::new(f, s).unwrap());
            }
            for i in 1..10 {
                cards.push(Card::new(&i.to_string(), s).unwrap());
            }
        }
        Self { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}
