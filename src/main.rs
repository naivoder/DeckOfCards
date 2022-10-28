mod card;
mod deck;

use deck::Deck;
use rand::{thread_rng, Rng};

fn main() {
    println!("BUILDING DECK OF CARDS");

    let mut deck = Deck::new();
    println!("Number of cards in deck: {}\n", deck.cards.len());

    println!("Testing shuffle...");
    for _ in 0..3 {
        let id = thread_rng().gen_range(0..52);
        println!("Card #{} (Before): {}", id, deck.cards[id]);
        deck.shuffle();
        println!("Card #{} (After): {}\n", id, deck.cards[id]);
    }

    println!("Testing draw...");
    for _ in 0..3 {
        let a_card = deck.draw_card();
        println!("Drew card: {}", a_card);
        println!("Number of cards in deck: {}\n", deck.cards.len());
    }
}
