use std::io;

fn main() {
    println!("GUESS THE NUMBER!!");

    println!("Please input your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the input");

    println!("You guessed: {}", guess);

    let x = 5;
    let y = 10;

    println!("x = {x + y} and y + 2 = {}", y + 2);
}
