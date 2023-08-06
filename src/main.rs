mod lib;

fn main() {
    // make a shuffled deckk and print it
    let mut deck = lib::Deck::new();
    deck.shuffle();
    for card in deck.cards.iter() {
        println!("{}", card);
    }
}
