
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::cmp::Ordering;
use std::fmt;
use std::io;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

mod lib {
#[derive(PartialEq, Eq, EnumIter, Copy, Clone)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match each suit to a symbol
        match *self {
            Suit::Hearts => write!(f, "♥"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Clubs => write!(f, "♣"),
            Suit::Spades => write!(f, "♠"),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, EnumIter, Clone, Copy)]
pub enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}
impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match each rank to a symbol or number
        match *self {
            Rank::Ace => write!(f, " A"),
            Rank::Two => write!(f, " 2"),
            Rank::Three => write!(f, " 3"),
            Rank::Four => write!(f, " 4"),
            Rank::Five => write!(f, " 5"),
            Rank::Six => write!(f, " 6"),
            Rank::Seven => write!(f, " 7"),
            Rank::Eight => write!(f, " 8"),
            Rank::Nine => write!(f, " 9"),
            Rank::Ten => write!(f, "10"),
            Rank::Jack => write!(f, " J"),
            Rank::Queen => write!(f, " Q"),
            Rank::King => write!(f, " K"),
        }
    }
}

pub struct Card {
    suit: Suit,
    rank: Rank,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fn red_card(rank: &str, suit: &str) -> String {
            format!("\x1b[31m{}{}\x1b[0m", rank, suit)
        }
        if self.suit == Suit::Hearts || self.suit == Suit::Diamonds {
            write!(
                f,
                "{}",
                red_card(&self.rank.to_string(), &self.suit.to_string())
            )
        } else {
            write!(f, "{}{}", self.rank, self.suit)
        }
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Deck {
        /// Generate a new sorted deck of cards
        let mut cards = Vec::with_capacity(52);
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card { suit, rank });
            }
        }
        Deck { cards }
    }
    fn shuffle(&mut self) {
        /// Shuffle the deck of cards in place
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}

}
