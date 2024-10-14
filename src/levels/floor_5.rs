// * Mutability

pub fn mutability() {
    let mut x: i8 = 1;
    x = 3;
    println!("{}", x);

    let mut y: String = String::from("Hi there");
    y.push_str(" Rust");
    println!("{}", y);

    //* 'const' keyword */
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; //^ always use uppercase with underscores between words

    //* shadowing */
    let i = 5;
    let i = i + 1;
    println!("The value above the scope is: {}", i);
    {
        let i = i * 2;
        println!("The value of i in the inner scope is {}", i);
    }
    println!("The value of i in outer scope is: {}", i);

    let spaces = "      ";
    let spaces = spaces.len();
    println!("The length of spaces is: {}", spaces);

    let mut spaces1 = "      "; //^ this variable doesn't need to be mutable
    // spaces1 = spaces1.len(); //^ this will give us error
    println!("The spaces1 length is: {}", spaces1);
}
