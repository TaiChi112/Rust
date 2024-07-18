mod module1;
mod module2;

use module1::my_function;
use module2::test_function;

fn main() {
    my_function();
    test_function();
}