mod levels {
    pub mod floor_1;
    pub mod floor_2;
    pub mod floor_3;
    pub mod floor_4;
}

use levels::floor_1::numbers;
use levels::floor_2::booleans;
use levels::floor_3::strings;
use levels::floor_4::ConditionalsAndLoops;

fn main() {
    numbers();
    booleans();
    strings();
    ConditionalsAndLoops();
}