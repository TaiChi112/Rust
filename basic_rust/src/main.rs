fn main() {
    let person1 = Person1 {
        id: 1,
        name: "Alice".to_string(),
    };
    println!("My id {} & My name's {}!", person1.id, person1.name);

    let person2 = basic_rust::Person2 {
        id: 2,
        name: "Bob".to_string(),
    };
    println!("My id {} & My name's {}!", person2.id, person2.name);
}

struct Person1 {
    id: u8,
    name: String,
}
