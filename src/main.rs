use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

trait Stackable {
    fn push(&mut self, card: Card) -> Result<(), Card>;
    fn pop(&mut self) -> Option<Card>;
}

#[derive(Debug, EnumIter)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Clone, Copy, Debug)]
struct Card {
    suit: Suit,
    rank: u8,
}

fn make_deck(suit: SuitIter, ranks: Vec<u8>) -> Vec<Card> {
    let mut cards = Vec::new();
    for rank in ranks {
        for suit in Suit::iter() {
            cards.push(Card { suit, rank });
        }
    }
    cards
}

// Deals cards from a vector into a vector of vectors
fn deal_cards(cards: Vec<Card>, num_piles: usize) -> Vec<Vec<Card>> {
    let mut piles = Vec::new();
    for _ in 0..num_piles {
        piles.push(Vec::new());
    }
    let mut i = 0;
    for card in cards {
        piles[i].push(card);
        i = (i + 1) % num_piles;
    }
    piles
}

struct Pile {
    cards: Vec<Card>,
}
impl Pile {
    fn new(cards: Vec<Card>) -> Pile {
        Pile { cards }
    }
}

struct Freecell {
    card: Option<Card>,
}
impl Freecell {
    fn new() -> Freecell {
        Freecell { card: None }
    }
}

struct Foundation {
    card: Option<Card>,
}
impl Foundation {
    fn new() -> Foundation {
        Foundation { card: None }
    }
}

const FREECELL_NUM: usize = 4;
const FOUNDATION_NUM: usize = 4; // TODO: make this automatically infered from Suit
const TABLEAU_NUM: usize = 8;
struct Game {
    tableau: Vec<Pile>,
    freecells: [Freecell; FREECELL_NUM],
    foundations: [Foundation; FOUNDATION_NUM],
}
impl Game {
    fn new() -> Game {
        let mut rng = thread_rng();
        let mut cards = make_deck(Suit::iter(), (1..=13).collect());
        cards.shuffle(&mut rng);
        let tableau = deal_cards(cards, TABLEAU_NUM)
            .into_iter()
            .map(|cards| Pile::new(cards))
            .collect();
        let freecells = [Freecell::new(); FREECELL_NUM];
        let foundations = [Foundation::new(); FOUNDATION_NUM];
        Game {
            tableau,
            freecells,
            foundations,
        }
    }
}

fn main() {
    let game = Game::new();
    println!("{:?}", game);
}
