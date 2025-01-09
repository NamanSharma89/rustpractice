mod guessgame;

use rand::Rng;
use crate::guessgame::game;

fn main() {
    println!("Guess the number man!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret number is {}", secret_number);
    game(secret_number)
}