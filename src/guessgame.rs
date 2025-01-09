use std::cmp::Ordering;
use std::io;

pub fn game(secret_number: u32) {
    loop {

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("TODO: panic message");
        print!("Please type a number!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Equal => { println!("Got it!"); break; },
            Ordering::Greater => println!("too big!"),
        }

    }
}