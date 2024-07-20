fn add(a: i32, b: i32) -> i32 {
    a + b
}
mod calculator {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    pub fn sub(a: i32, b: i32) -> i32 {
        a - b
    }
}
// HOF
fn mutiply_by(factor: i32) -> impl Fn(i32) -> i32 {
    move |number| number * factor
}
fn main() {
    println!("result : {}", add(1, 2));
    println!("result : {}", calculator::add(1, 2));
    println!("result : {}", calculator::sub(1, 2));

    let double = mutiply_by(2);
    let triple = mutiply_by(3);

    println!("double : {}", double(2));
    println!("triple : {}", triple(2));
}
