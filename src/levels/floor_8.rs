fn main() {
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("short");
        result = longest(string1.as_str(), string2.as_str()); // usually error occurs here
    }
    println!("The longest string is: {}", result);

    // Result in error handling
    fn divide(a: f64, b: f64) -> Result<f64, String> {
        if b == 0.0 {
            Err(String::from("Division by zero"))
        } else {
            Ok(a / b)
        }
    }

    // Options in error handling
    fn find_char(s: &str, c: char) -> Option<usize> {
        for (i, ch) in s.chars().enumerate() {
            if ch == c {
                return Some(i);
            }
        }
        None
    }
}

main();