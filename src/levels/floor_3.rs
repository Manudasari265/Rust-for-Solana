// Strings

pub fn strings() {
    let greeting = String::from("Hello world");
    println!("{}", greeting);

    let char1 = greeting.chars().nth(1);

    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at index 1"),
    }

    println!("{:?}", char1);

    if let Some(c) = char1 {
        println!("{}", c);
    }
}