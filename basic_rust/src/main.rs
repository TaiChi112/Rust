//package::module::struct
use basic_rust::{
    customer::Customer,
    person::{Person2, Person3},
    specking::Specking,
};
fn main() {
    let person1 = Person1 {
        id: 1,
        name: "Alice".to_string(),
    };
    println!("My id {} & My name's {}!", person1.id, person1.name);

    let person2 = Person2 {
        id: 2,
        name: "Bob".to_string(),
    };
    println!("My id {} & My name's {}!", person2.id, person2.name);

    let person3 = Person3::new(1, "Charlie".to_string());
    person3.hello2();

    let customer1 = Customer::new(112, "Customer1".to_string());
    customer1.hello();

    let person4 = Person3::new(113, "John".to_string());
    person4.speck();

    // enum
    let x = Colors::Red;
    match x {
        Colors::Red => println!("Red"),
        Colors::Yellow => println!("Yellow"),
        // Colors::Blue => println!("Blue"),
        _ => println!("Unknown"),
    }
    let a1 = Colors::Blue;
    let a2 = Colors::Yellow;
    match (a1, a2) {
        (Colors::Blue, Colors::Yellow) => println!("Blue & Yellow"),
        _ => println!("Not Blue & Yellow"),
    }
    // let y = Colors::Blue;
    // let mut z = "";
    // match y {
    //     Colors::Blue => z = "Blue",
    //     Colors::Red => z = "Green",
    //     Colors::Yellow => z = "Yellow",
    //     _ => println!("Unknown"),
    // }

    let color = match x {
        Colors::Red => "Red",
        Colors::Yellow => "Yellow",
        Colors::Blue => "Blue",
    };
    println!("{}", color);

    let x1 = check_grade(50);
    match x1 {
        GradeResult::Error(e) => println!("{}", e),
        GradeResult::Value(g) => println!("{}", g),
    }

    let x2 = check_grade1(80);
    match x2 {
        None => println!("Invalid score"),
        Some(v) => println!("{}", v),
    }
    let x3 = check_grade3(-112);
    match x3 {
        Err(e) => println!("{}", e),
        Ok(v) => println!("{}", v),
    }
    let x4 = check_grade3(100);
    let abstrcut_score = x4.unwrap();
    println!("{}", abstrcut_score);

    let x5 = check_grade3(100);
    if x5.is_err() {
        return;
    }
    let y1 = x5.unwrap();
    println!("{}", y1);

    let x6 = check_grade3(100);
    if let Ok(v) = x6 {
        println!("{}", v);
    }

    let x7 = check_grade3(100);
    let y2 = match x7 {
        Err(e) => {
            println!("{}", e);
            return;
        }
        Ok(v) => v,
    };
    println!("{}", y2);

    // Closures
    let xx = add(10, 20);
    println!("{}", xx);

    let yy = |a, b| a + b;
    // let yy = |a: i32, b: i32| {a + b};// use case long imple fn

    let zz = yy(10, 50);
    println!("{}", zz);
    // let add = |x: i32, y: i32| x + y;

    let yy1 = cal(10, 40, yy);
    println!("{}", yy1);

    let yy2 = cal(10, 40, |a, b| a - b);
    println!("{}", yy2);

    let yy3 = cal(10, 90, add);
    println!("{}", yy3);

    let yy4 = cal2(10, 90, |a, b| a * b);
    println!("{}", yy4);
}
fn cal<F: Fn(i32, i32) -> i32>(a: i32, b: i32, f: F) -> i32 {
    f(a, b)
}
fn cal2<F>(a: i32, b: i32, f: F) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    f(a, b)
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn check_grade3(score: i32) -> Result<String, String> {
    if score < 0 || score > 100 {
        return Err("Invalid score".to_string());
    }
    return Ok("you are grade A".to_string());
}
//std enum in RS
fn check_grade1(score: i32) -> Option<String> {
    if score < 0 || score > 100 {
        return None;
    }
    Some("you are grade A".to_string())
}
fn check_grade(score: i32) -> GradeResult {
    if score < 0 || score > 100 {
        return GradeResult::Error("Invalid score".to_string());
    }
    return GradeResult::Value("you are grade A".to_string());
}
enum GradeResult {
    Value(String),
    Error(String),
}
struct Person1 {
    id: u8,
    name: String,
}

enum Colors {
    Red,
    Yellow,
    Blue,
}
