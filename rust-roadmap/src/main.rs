fn main() {
    let mut items: Vec<String> = Vec::new();

    create(&mut items, " 1 ".to_string());
    create(&mut items, " 2 ".to_string());
    create(&mut items, " 3 ".to_string());
    
    read(&items);

    update(&mut items, 1, "112".to_string());
    read(&items);

    delete(&mut items, 2);
    read(&items);

}

fn create(items: &mut Vec<String>, new_item: String) {
    items.push(new_item);
}

fn read(items: &Vec<String>) {
    for (index, item) in items.iter().enumerate() {
        println!("Index: {}, item: {}", index, item);
    }
    println!("Length: {}", items.len());
}
fn update(items: &mut Vec<String>, index: usize, new_value: String) {
    if let Some(item) = items.get_mut(index) {
        *item = new_value;
    } else {
        println!("No item found at index {}", index);
    }
}
fn delete(items: &mut Vec<String>, index: usize) {
    if let Some(_) = items.get(index) {
        items.remove(index);
    } else {
        println!("Index {} out of bounds", index);
    }
}
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
