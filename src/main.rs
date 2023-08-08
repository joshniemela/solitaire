mod lib;
use lib::Game;


fn main() {
    // make a shuffled deck and print it
    let mut Game = Game::new();
    println!("{:?}", Game);
}
