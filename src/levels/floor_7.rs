fn main() {
    let mut string = String::from("Hello");
    string.push_str(", world!");
    
    let slice: &str = &string[0..5];
    
    println!("slice is: {slice}");
    
    let x = 10;
    let y = if x > 5 {10} else {0};
    println!("value of y is: {}", y);
    
    loop {
        println!("infinite loop");
        break;
    }
    
    let mut n = 3;
    while n > 0 {
        println!("{}", n);
        n -= 1;
    }
    
    for i in 1..4 {
        println!("{}", i);
    }
    
    let number = 4;
    
    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }
    
    fn add(a: i32, b: i32) -> i32 {
        return a + b;
    }
    
    fn square(x: i32) -> i32 {
        x * x
    }
    
    let result1 = add(3, 4);
    let result2 = square(5);
    println!("result of add is : {}", result1);
    println!("result of square is : {}", result2);
    
    struct User {
        username: String,
        email: String,
        active: bool,
    }
    
    let user1 = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };
    
    // enums IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    //     Unknown,
    // }
    
    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));
    // let unknown = IpAddr::Unknown;
    
    let s3 = String::from("block string");
    {
        let s4 = s3;
        println!("{}", s4);
    }
    
    // println!("{}", s3);
    
}























