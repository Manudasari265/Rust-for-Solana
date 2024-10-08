mod levels {
    pub mod floor_1;
    pub mod floor_2;
    pub mod floor_3;
    pub mod floor_4;
    pub mod floor_5;
}

use levels::floor_1::numbers;
use levels::floor_2::booleans;
use levels::floor_3::strings;
use levels::floor_4::conditionals_and_loops;
use levels::floor_5::mutability;

fn main() {
    numbers();
    booleans();
    strings();
    conditionals_and_loops();
    mutability();
}