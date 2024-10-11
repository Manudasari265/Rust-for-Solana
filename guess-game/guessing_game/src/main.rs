use std::io;
use rand::Rng;
use std::cmp::Ordering;
/* 
   Current program we are 
   building is a binary crate, 
   which is an executable crate
*/
fn main() {
    println!("GUESS THE NUMBER!!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        /* 
           String::new a function that returns a new instance 
           of a String type, ::new function creates 
           a new, empty string binding to mut guess variable
        */ 
        let mut guess = String::new();
        
        /*
          Calling the stdin() function from the io module, 
          which will allow us to handle user input 
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");
    
        let guess: u32 = match guess.trim().parse() {
           Ok(num) => num,
           Err(_) => continue,
        };
        
        // println!("Please type a number");
    
        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
          Ordering::Less => println!("Too small"),
          Ordering::Greater => println!("Too big!"),
          Ordering::Equal => {
            println!("You win!");
            break;
          }
        } 
    }
}
