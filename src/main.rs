mod card;
mod deck;

use card::Card;

fn main() {
    let a_card = Card::new("1", "H").unwrap();
    println!("Card: {}", a_card);
}
