mod levels {
    pub mod floor_1;
    pub mod floor_2;
    pub mod floor_3;
    pub mod floor_4;
    pub mod floor_5;
    pub mod floor_6;
    mod floor_7;
    mod floor_8;
}

use levels::floor_1::numbers;
use levels::floor_2::booleans;
use levels::floor_3::strings;
use levels::floor_4::conditionals_and_loops;
use levels::floor_5::mutability;
use levels::floor_6::data_types;


fn main() {
    numbers();
    booleans();
    strings();
    conditionals_and_loops();
    mutability();
    data_types();
}