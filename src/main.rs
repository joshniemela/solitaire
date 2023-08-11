use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

trait Stackable {
    fn push(&mut self, card: Card) -> Result<(), Card>;
    fn pop(&mut self) -> Option<Card>;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, EnumIter)]
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

#[derive(Debug)]
struct Pile {
    cards: Vec<Card>,
}
impl Pile {
    fn new(cards: Vec<Card>) -> Pile {
        Pile { cards }
    }
}
impl Stackable for Pile {
    fn push(&mut self, card: Card) -> Result<(), Card> {
        match self.cards.last() {
            None => {
                self.cards.push(card);
                Ok(())
            }
            Some(Card { suit, rank }) => {
                if card.suit != *suit && card.rank == rank - 1 {
                    self.cards.push(card);
                    Ok(())
                } else {
                    Err(card)
                }
            }
        }
    }

    fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}

#[derive(Clone, Copy, Debug)]
struct Freecell {
    card: Option<Card>,
}
impl Stackable for Freecell {
    fn push(&mut self, card: Card) -> Result<(), Card> {
        if self.card.is_some() {
            Err(card)
        } else {
            self.card = Some(card);
            Ok(())
        }
    }
    fn pop(&mut self) -> Option<Card> {
        self.card
    }
}

#[derive(Clone, Copy, Debug)]
struct Foundation {
    card: Option<Card>,
}
impl Stackable for Foundation {
    fn push(&mut self, card: Card) -> Result<(), Card> {
        match self.card {
            None => {
                if card.rank == 0 {
                    self.card = Some(card);
                    Ok(())
                } else {
                    Err(card)
                }
            }
            // deconstruct the card to suit and rank
            Some(Card { suit, rank }) => {
                if card.suit == suit && card.rank == rank + 1 {
                    self.card = Some(card);
                    Ok(())
                } else {
                    Err(card)
                }
            }
        }
    }
    fn pop(&mut self) -> Option<Card> {
        // return the card and decrement the rank by 1
        match self.card {
            None => None,
            Some(Card { suit, rank }) => {
                if rank == 1 {
                    self.card = None;
                } else {
                    self.card = Some(Card {
                        suit,
                        rank: rank - 1,
                    });
                }
                self.card
            }
        }
    }
}

const FREECELL_NUM: usize = 4;
const FOUNDATION_NUM: usize = 4; // TODO: make this automatically infered from Suit
const TABLEAU_NUM: usize = 8;

#[derive(Debug)]
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
        let freecells = [Freecell { card: None }; FREECELL_NUM];
        let foundations = [Foundation { card: None }; FOUNDATION_NUM];
        Game {
            tableau,
            freecells,
            foundations,
        }
    }
}

fn make_ascii_card(card: Option<Card>) -> String {
    match card {
        None => String::from(" 0 "),
        Some(Card { suit, rank }) => {
            let suit_char = match suit {
                Suit::Clubs => '♣',
                Suit::Diamonds => '♦',
                Suit::Hearts => '♥',
                Suit::Spades => '♠',
            };
            let rank_str = match rank {
                1 => String::from(" A"),
                10 => String::from("10"),
                11 => String::from(" J"),
                12 => String::from(" Q"),
                13 => String::from(" K"),
                _ => format!("{:2}", rank),
            };
            format!("{}{}", rank_str, suit_char)
        }
    }
}

fn make_game_ascii(game: &Game) -> String {
    // first row is foundations and freecells
    let mut ascii = String::new();
    for foundation in game.foundations.iter() {
        ascii.push_str(&make_ascii_card(foundation.card));
        ascii.push(' ');
    }
    for freecell in game.freecells.iter() {
        ascii.push_str(&make_ascii_card(freecell.card));
        ascii.push(' ');
    }
    ascii.push('\n');

    // line spacer
    ascii.push_str(&String::from_utf8(vec![b'-'; 4 * (FREECELL_NUM + FOUNDATION_NUM)]).unwrap());
    ascii.push('\n');

    // second row is tableau
    let mut max_pile_size = 0;
    for pile in game.tableau.iter() {
        if pile.cards.len() > max_pile_size {
            max_pile_size = pile.cards.len();
        }
    }
    for i in 0..max_pile_size {
        for pile in game.tableau.iter() {
            if pile.cards.len() > i {
                ascii.push_str(&make_ascii_card(Some(pile.cards[i])));
            } else {
                ascii.push_str("   ");
            }
            ascii.push(' ');
        }
        ascii.push('\n');
    }
    ascii
}

fn move_card(game: &mut Game, from: usize, to: usize) -> Result<(), ()> {
    // move the nth card to the mth freecell
    let card = game.tableau[from].pop().ok_or(())?;
    game.freecells[to].push(card).map_err(|_| ())?;
    Ok(())
}

// ACUTAL LOOP
use crossterm::terminal;
use std::io;
use std::io::Read;

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Could not disable raw mode")
    }
}

fn main() {
    let _clean_up = CleanUp;
    terminal::enable_raw_mode().expect("failed to turn on Raw mode");
    let mut buf = [0; 1];
    while io::stdin().read(&mut buf).expect("Failed to read line") == 1 && buf != [b'q'] {}
    panic!(); /* add this line*/
}
