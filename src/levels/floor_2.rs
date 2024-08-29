// Booleans in Rust

pub fn booleans() {
    let is_male: bool = true;
    let is_above_18:bool = true;

    if is_male {
        print!(" You are a male ");
    }
    else{
        print!("You are not a male");
    }

    if is_above_18 && is_male {
        print!(" You are above 18 ");
    }
}