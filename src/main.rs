use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

trait Stackable {
    fn legal_push(&self, card: Card) -> bool;
    fn push(&mut self, card: Card);
    fn pop(&mut self) -> Option<Card>;
    fn top(&self) -> Option<Card>;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, EnumIter)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Clubs => write!(f, "♣"),
            Suit::Diamonds => write!(f, "♦"),
            Suit::Hearts => write!(f, "♥"),
            Suit::Spades => write!(f, "♠"),
        }
    }
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
    fn legal_push(&self, card: Card) -> bool {
        match self.cards.last() {
            None => true,
            Some(Card { suit, rank }) => card.suit != *suit && card.rank == rank - 1,
        }
    }
    fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }
    fn top(&self) -> Option<Card> {
        self.cards.last().copied()
    }
}

#[derive(Clone, Copy, Debug)]
struct Freecell {
    card: Option<Card>,
}
impl Stackable for Freecell {
    fn legal_push(&self, card: Card) -> bool {
        self.card.is_none()
    }
    fn push(&mut self, card: Card) {
        self.card = Some(card);
    }
    fn pop(&mut self) -> Option<Card> {
        let card = self.card;
        self.card = None;
        card
    }
    fn top(&self) -> Option<Card> {
        self.card
    }
}

#[derive(Clone, Copy, Debug)]
struct Foundation {
    card: Option<Card>,
}
impl Stackable for Foundation {
    fn legal_push(&self, card: Card) -> bool {
        match self.card {
            None => card.rank == 0,
            Some(Card { suit, rank }) => card.suit == suit && card.rank == rank + 1,
        }
    }
    fn push(&mut self, card: Card) {
        self.card = Some(card);
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
    fn top(&self) -> Option<Card> {
        self.card
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

// ACUTAL LOOP
use crossterm::{
    cursor::{DisableBlinking, EnableBlinking, Hide, MoveTo, RestorePosition, SavePosition, Show},
    execute, queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{
        disable_raw_mode, enable_raw_mode, Clear, ClearType::All, EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use std::io;
use std::io::Read;
use std::io::Write;

// this sets the terminal to be ready to play the game
fn enter_alt_screen() {
    enable_raw_mode().unwrap();
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        Clear(All),
        SavePosition,
        MoveTo(10, 10),
        Hide,
    );
}

fn draw_card_frame(stdout: &mut io::Stdout, x: u16, y: u16) {
    // draw a pretty card with frame and all
    queue!(
        stdout,
        MoveTo(x, y),
        EnableBlinking,
        Print("┌────┐"),
        MoveTo(x, y + 1),
        Print("│    │"),
        MoveTo(x, y + 2),
        Print("│    │"),
        MoveTo(x, y + 3),
        Print("│    │"),
        MoveTo(x, y + 4),
        Print("└────┘"),
        DisableBlinking,
    );
}

fn rank_to_char(rank: u8) -> char {
    match rank {
        1 => 'A',
        10 => 'T',
        11 => 'J',
        12 => 'Q',
        13 => 'K',
        _ => (rank + 48) as char,
    }
}

fn draw_card(stdout: &mut io::Stdout, card: Option<Card>, x: u16, y: u16) {
    // draw a pretty card with frame and all
    let mut stdout = io::stdout();
    // if the card is red, change the color to red
    draw_card_frame(&mut stdout, x, y);
    match card {
        None => {}
        Some(Card { suit, rank }) => {
            if suit == Suit::Diamonds || suit == Suit::Hearts {
                queue!(stdout, crossterm::style::SetForegroundColor(Color::Red));
            };
            queue!(
                stdout,
                MoveTo(x + 1, y + 1),
                Print(rank_to_char(rank)),
                MoveTo(x + 1, y + 3),
                Print(suit),
                MoveTo(x + 4, y + 1),
                Print(suit),
                MoveTo(x + 4, y + 3),
                Print(rank_to_char(rank)),
                ResetColor,
            );
        }
    }
}

fn draw_game(game: &Game) {
    // gives some margin
    let origin = (2, 0);
    let foundation_origin = (origin.0 + 6 * FREECELL_NUM as u16 + 2, origin.1);
    let pile_origin = (origin.0, origin.1 + 6);
    let mut stdout = io::stdout();
    queue!(stdout, Clear(All));
    // drawing the freecells
    for (i, freecell) in game.freecells.iter().enumerate() {
        draw_card(
            &mut stdout,
            freecell.card,
            origin.0 + 6 * i as u16,
            origin.1,
        );
    }
    // draw the foundations
    for (i, foundation) in game.foundations.iter().enumerate() {
        draw_card(
            &mut stdout,
            foundation.card,
            foundation_origin.0 + 6 * i as u16,
            foundation_origin.1,
        );
    }
    // draw the tableau
    for (i, pile) in game.tableau.iter().enumerate() {
        for (j, card) in pile.cards.iter().enumerate() {
            draw_card(
                &mut stdout,
                Some(*card),
                pile_origin.0 + 6 * i as u16,
                pile_origin.1 + 2 * j as u16,
            );
        }
    }
    stdout.flush();
}

// this cleans up the terminal after the game is done
struct CleanUp;
impl Drop for CleanUp {
    fn drop(&mut self) {
        execute!(io::stdout(), LeaveAlternateScreen, Show).unwrap();
        disable_raw_mode().unwrap();
    }
}

// move from one struct that implements stackable to another stackable
fn move_card(game: &mut Game, from: char, to: char) -> Result<(), ()> {
    let from_stack = get_stackable(game, from)?;

    match from_stack.top() {
        None => Err(()),
        Some(card) => {
            let to_stack = get_stackable(game, to)?;
            if to_stack.legal_push(card) {
                let from_stack = get_stackable(game, from)?;
                let card = from_stack.pop().unwrap();
                let to_stack = get_stackable(game, to)?;
                to_stack.push(card);
                Ok(())
            } else {
                Err(())
            }
        }
    }
}

const FOUNDATION_KEYS: [char; 4] = ['t', 'y', 'u', 'i'];
const FREECELL_KEYS: [char; 4] = ['q', 'w', 'e', 'r'];
const PILE_KEYS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

// using a char and the game, get the corresponding stackable
fn get_stackable(game: &mut Game, key: char) -> Result<&mut dyn Stackable, ()> {
    if FOUNDATION_KEYS.contains(&key) {
        Ok(&mut game.foundations[FOUNDATION_KEYS.iter().position(|&x| x == key).unwrap()])
    } else if FREECELL_KEYS.contains(&key) {
        Ok(&mut game.freecells[FREECELL_KEYS.iter().position(|&x| x == key).unwrap()])
    } else if PILE_KEYS.contains(&key) {
        Ok(&mut game.tableau[PILE_KEYS.iter().position(|&x| x == key).unwrap()])
    } else {
        Err(())
    }

fn main() {
    let _clean_up = CleanUp;

    enter_alt_screen();
    let mut game = Game::new();
    draw_game(&game);
    let mut stdin = io::stdin();
    loop {
        let mut buffer = [0; 1];
        stdin.read_exact(&mut buffer).unwrap();

        let key = buffer[0] as char;

        if key == 'Q' {
            break;
        }
        // do a move
        if key == ' ' {
            let mut buffer = [0; 2];
            stdin.read_exact(&mut buffer).unwrap();
            let from = buffer[0] as char;
            let to = buffer[1] as char;
            execute!(
                io::stdout(),
                MoveTo(0, 25),
                Print(format!("Moving card from: {} to: {}", from, to))
            );
            if from == to {
                continue;
            }
            // try to move the card
            if move_card(&mut game, from, to).is_ok() {
                draw_game(&game);
            }
        }
    }
}
