struct Person1 {
    id: u8,
    name: String,
}
impl Person1 {
    fn new(id: u8, name: String) -> Person1 {
        Person1 { id, name }
    }
}
struct Person2 {
    id: u8,
    name: String,
}
impl Person2 {
    fn new(id: Option<u8>, name: Option<String>) -> Person2 {
        Person2 {
            id: id.unwrap_or(0),
            name: name.unwrap_or("".to_string()),
        }
    }
}
struct Person3;
fn main() {
    let p1 = Person1::new(1, "Alice".to_string());
    println!("id: {}, name: {}", p1.id, p1.name);

    let p2 = Person2 {
        id: 2,
        name: "Bob".to_string(),
    };
    println!("id: {}, name: {}", p2.id, p2.name);

    let p21 = Person2::new(Some(3), Some("Charlie".to_string()));
    println!("id: {}, name: {}", p21.id, p21.name);

    let p3 = Person3;
}
