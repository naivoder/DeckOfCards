mod card;
mod deck;

use deck::Deck;
use rand::{thread_rng, Rng};

fn main() {
    let mut deck = Deck::new();
    println!("Number of cards in deck: {}", deck.cards.len());

    for _ in 0..5 {
        let id = thread_rng().gen_range(0..52);
        println!("Card #{} (Before Shuffle): {}", id, deck.cards[id]);
        deck.shuffle();
        println!("Card #{} (After Shuffle): {}", id, deck.cards[id]);
    }
}
