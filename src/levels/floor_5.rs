// * Mutability

pub fn mutability() {
    let mut x: i8 = 1;
    x = 3;
    println!("{}", x);

    let mut y: String = String::from("Hi there");
    y.push_str(" Rust");
    println!("{}", y);
}