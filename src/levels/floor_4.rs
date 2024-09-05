// Conditionals and Loops

pub fn conditionals_and_loops() {
    let is_even = true;

    if(is_even) {
        println!("The number is even");
    }
    else{
        println!("The number is odd");
    }

    for i in 0..11 {
        print!("{} ", i);
    }
    
    let sentence = String::from("My name is Manoj");
    let first_word = get_first_word(sentence);

    fn get_first_word(sentence: String) -> String {
        let mut ans: String = String::from("");
        for char in sentence.chars() {
            ans.push_str(char.to_string().as_str());
            if(char == ' ') {
                break;
            }
        }
        return ans;
    }

    println!("The first word is: {}", first_word);
}