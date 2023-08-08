use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;

use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Colour {
    Red,
    Black,
}

#[derive(PartialEq, Eq, EnumIter, Copy, Clone, Debug, EnumCountMacro)]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}
impl Suit {
    pub fn colour(&self) -> Colour {
        match self {
            Suit::Hearts => Colour::Red,
            Suit::Diamonds => Colour::Red,
            Suit::Clubs => Colour::Black,
            Suit::Spades => Colour::Black,
        }
    }
}

#[derive(PartialEq, Eq, EnumIter, Clone, Copy, Debug)]
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
impl Rank {
    pub fn next(&self) -> Option<Rank> {
        match self {
            Rank::Ace => Some(Rank::Two),
            Rank::Two => Some(Rank::Three),
            Rank::Three => Some(Rank::Four),
            Rank::Four => Some(Rank::Five),
            Rank::Five => Some(Rank::Six),
            Rank::Six => Some(Rank::Seven),
            Rank::Seven => Some(Rank::Eight),
            Rank::Eight => Some(Rank::Nine),
            Rank::Nine => Some(Rank::Ten),
            Rank::Ten => Some(Rank::Jack),
            Rank::Jack => Some(Rank::Queen),
            Rank::Queen => Some(Rank::King),
            Rank::King => None,
        }
    }
    pub fn prev(&self) -> Option<Rank> {
        match self {
            Rank::Ace => None,
            Rank::Two => Some(Rank::Ace),
            Rank::Three => Some(Rank::Two),
            Rank::Four => Some(Rank::Three),
            Rank::Five => Some(Rank::Four),
            Rank::Six => Some(Rank::Five),
            Rank::Seven => Some(Rank::Six),
            Rank::Eight => Some(Rank::Seven),
            Rank::Nine => Some(Rank::Eight),
            Rank::Ten => Some(Rank::Nine),
            Rank::Jack => Some(Rank::Ten),
            Rank::Queen => Some(Rank::Jack),
            Rank::King => Some(Rank::Queen),
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Card {
    suit: Suit,
    rank: Rank,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        // Generate a new sorted deck of cards
        let mut cards = Vec::with_capacity(52);
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card { suit, rank });
            }
        }
        Deck { cards }
    }
    pub fn shuffle(&mut self) {
        // Shuffle the deck of cards in place
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Foundation {
    pub top: Option<Card>,
}
impl Foundation {
    pub fn new() -> Foundation {
        Foundation { top: None }
    }
    pub fn legal(&self, card: &Card) -> bool {
        match &self.top {
            None => card.rank == Rank::Ace,
            Some(top) => {
                card.suit == top.suit && match top.rank.next() {
                    None => false,
                    Some(next) => card.rank == next,
                }
            }
        }
    }
    pub fn push(&mut self, card: Card) -> Result<(), Card> {
        if self.legal(&card) {
            self.top = Some(card);
            Ok(())
        } else {
            Err(card)
        }
    }

    pub fn pop(&mut self) -> Result<Card, ()> {
        match self.top {
            None => Err(()),
            Some(card) => {
                self.top = None;
                Ok(card)
            }
        }
    }
}

#[derive(Debug)]
pub struct Tableau {
    pub cards: Vec<Card>,
}
impl Tableau {
    pub fn new() -> Tableau {
        Tableau { cards: Vec::new() }
    }
    pub fn legal(&self, card: &Card) -> bool {
        match self.cards.last() {
            None => true,
            Some(top) => {
                card.suit.colour() != top.suit.colour() && match top.rank.prev() {
                    None => false,
                    Some(prev) => card.rank == prev,
                }
            }
        }
    }
    pub fn push(&mut self, card: Card) -> Result<(), Card> {
        if self.legal(&card) {
            self.cards.push(card);
            Ok(())
        } else {
            Err(card)
        }
    }
    pub fn illegal_push(&mut self, card: Card) {
        self.cards.push(card);
    }
    pub fn pop(&mut self) -> Result<Card, ()> {
        match self.cards.pop() {
            None => Err(()),
            Some(card) => Ok(card),
        }
    }
}

const FREECELL_NUM: usize = 4;
const TABLEAU_NUM: usize = 8;

#[derive(Debug)]
pub struct Game {
    pub freecells: [Option<Card>; FREECELL_NUM],
    pub foundations: [Foundation; Suit::COUNT],
    pub tableau: Vec<Tableau>,
}
impl Game {
    pub fn new() -> Game {
        let mut tableaus: Vec<Tableau> = Vec::with_capacity(TABLEAU_NUM);
        for _ in 0..TABLEAU_NUM {
            tableaus.push(Tableau::new());
        }

        let random_deck = {
            let mut deck = Deck::new();
            deck.shuffle();
            deck
        };

        for (i, card) in random_deck.cards.into_iter().enumerate() {
            let tableau = &mut tableaus[i % TABLEAU_NUM];
            tableau.illegal_push(card);
        }

        Game {
            freecells: [None; FREECELL_NUM],
            foundations: [Foundation::new(); Suit::COUNT],
            tableau: tableaus,
        }
    }
}
