#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
fn main() {
    let mut a1 = 1;
    // variable
    let x = 10;
    let mut y = 20;
    println!("{}", x);
    println!("{}", y);

    y = 112;
    println!("{}", y);
    y = 118;
    println!("{}", y);

    let (a, b) = (99, 87);
    println!("{}", a);
    println!("{}", b);

    // constant
    const PI: f32 = 3.14;

    // Tuple
    let mut m = (112, 3.14, "Alice");
    println!("{}", m.0);
    println!("{}", m.1);
    println!("{}", m.2);
    m.0 = 113;
    println!("{}", m.0);

    let m: (u8, f64, i32) = (112, 3.14, 113); // memory in stack
    println!("{}", m.0);
    println!("{}", m.1);
    println!("{}", m.2);

    let a1 = m.0;
    println!("{}", a1);

    //Array
    let x: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{}", x[0]);
    println!("{}", x[1]);
    println!("{}", x[2]);
    println!("{}", x[3]);
    println!("{}", x[4]);
    let x = [1, 2, 3, 4, 5];

    let x = [112; 5]; // duplicate data of array
    println!("{}", x[0]);
    println!("{}", x[1]);
    println!("{}", x[2]);
    println!("{}", x[3]);
    println!("{}", x[4]);

    // condition statement if else
    let score = 50;
    let mut grade = "";
    if score >= 80 {
        grade = "A";
    } else if score >= 70 {
        grade = "B";
    } else if score >= 60 {
        grade = "C";
    } else if score >= 50 {
        grade = "D";
    } else {
        grade = "F";
    }
    println!("you are grade : {}", grade);
    // let grade = if score >= 50 { "pass" } else { "fail" };
    // println!("{}", grade);
}

fn get_number() -> i32 {
    let x = 10;
    let y = 20;
    x + y
}
// mod func;
// use func::array;
// fn main(){
//     array::array();
// }
// fn main() {
//     let mut items: Vec<String> = Vec::new();

//     create(&mut items, " 1 ".to_string());
//     create(&mut items, " 2 ".to_string());
//     create(&mut items, " 3 ".to_string());

//     read(&items);

//     update(&mut items, 1, "112".to_string());
//     read(&items);

//     delete(&mut items, 2);
//     read(&items);

// }

// fn create(items: &mut Vec<String>, new_item: String) {
//     items.push(new_item);
// }

// fn read(items: &Vec<String>) {
//     for (index, item) in items.iter().enumerate() {
//         println!("Index: {}, item: {}", index, item);
//     }
//     println!("Length: {}", items.len());
// }
// fn update(items: &mut Vec<String>, index: usize, new_value: String) {
//     if let Some(item) = items.get_mut(index) {
//         *item = new_value;
//     } else {
//         println!("No item found at index {}", index);
//     }
// }
// fn delete(items: &mut Vec<String>, index: usize) {
//     if let Some(_) = items.get(index) {
//         items.remove(index);
//     } else {
//         println!("Index {} out of bounds", index);
//     }
// }
// mod utils;

// use utils::{
//     data::arr::max,
//     math::{
//         algebra::{add, subtract},
//         matrix::matrix1,
//     },
//     other::test_other,
// };

// mod func;
// use func::test::{test1, test2};

// fn main() {
//     let x = 10;
//     let y = 20;
//     println!("add {} : ", add(x, y));

//     println!("substract : {}  ", subtract(x, y));

//     println!("test {} ", utils::math::algebra::add(x, y));
//     test1();
//     test2();

//     test_other();
//     matrix1();

//     let my_array = [1, 2, 3, 4, 5];

//     println!("The maximum value in the array is: {}", max(&my_array));
// }
