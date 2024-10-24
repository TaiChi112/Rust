mod utils;

use utils::{
    data::arr::max,
    math::{
        algebra::{add, subtract},
        matrix::matrix1,
    },
    other::test_other,
};

mod func;
use func::test::{test1, test2};

fn main() {
    let x = 10;
    let y = 20;
    println!("add {} : ", add(x, y));

    println!("substract : {}  ", subtract(x, y));

    println!("test {} ", utils::math::algebra::add(x, y));
    test1();
    test2();

    test_other();
    matrix1();

    let my_array = [1, 2, 3, 4, 5];

    println!("The maximum value in the array is: {}", max(&my_array));
}
