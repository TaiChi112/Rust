struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn default() -> Point {
        Point { x: 0, y: 0 }
    }
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn set_x(&mut self, x: i32) {
        self.x = x;
    }
    fn set_y(&mut self, y: i32) {
        self.y = y;
    }
    fn get_x(&self) -> i32 {
        self.x
    }
    fn get_y(&self) -> i32 {
        self.y
    }

    fn display(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}
struct Address {
    city: String,
    state: String,
    country: String,
}
impl Address {
    fn new(city: String, state: String, country: String) -> Address {
        Address {
            city,
            state,
            country,
        }
    }
    fn display(&self) {
        println!(
            "City: {}, State: {}, Country: {}",
            self.city, self.state, self.country
        );
    }
    fn set_city(&mut self, city: String) {
        self.city = city;
    }
}
struct IdentityCard {
    name: String,
    age: i32,
    address: Address,
}
impl IdentityCard {
    fn new(name: String, age: i32, address: Address) -> IdentityCard {
        IdentityCard { name, age, address }
    }
    fn display(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
        self.address.display();
    }
}
struct Person {
    id_card: IdentityCard,
}
impl Person {
    fn new(name: String, age: i32, address: Address) -> Person {
        Person {
            id_card: IdentityCard::new(name, age, address),
        }
    }
    fn display(&self) {
        self.id_card.display();
    }
}
struct Stack {
    stack: Vec<i32>,
    capacity: usize,
    top: usize,
}
impl Stack {
    fn new(capacity: usize) -> Stack {
        Stack {
            stack: Vec::with_capacity(capacity),
            capacity,
            top: 0,
        }
    }
    fn push(&mut self, value: i32) {
        if self.top < self.capacity {
            self.stack.push(value);
            self.top += 1;
        } else {
            println!("Stack is full");
        }
    }
    fn pop(&mut self) -> Option<i32> {
        if self.top > 0 {
            self.top -= 1;
            self.stack.pop()
        } else {
            println!("Stack is empty");
            None
        }
    }
    fn display(&self) {
        println!("Stack: {:?}", self.stack);
    }
    fn is_empty(&self) -> bool {
        self.top == 0
    }
    fn is_full(&self) -> bool {
        self.top == self.capacity
    }
    fn size(&self) -> usize {
        self.top
    }
    fn capacity(&self) -> usize {
        self.capacity
    }
}
fn main() {
    let p1 = Point::new(10, 20);
    p1.display();
    println!("x: {}, y: {}", p1.get_x(), p1.get_y());

    let mut p2 = Point::default();
    p2.display();
    p2.set_x(30);
    p2.set_y(40);
    p2.display();

    let mut ps1 = Person::new(
        "John".to_string(),
        30,
        Address::new("New York".to_string(), "NY".to_string(), "USA".to_string()),
    );
    ps1.display();
    ps1.id_card.address.set_city("Los Angeles".to_string());
    ps1.display();

    let mut s1 = Stack::new(5);
    s1.push(10);
    s1.push(20);
    s1.push(30);
    s1.display();
    println!("Stack size: {}", s1.size());
    println!("Stack capacity: {}", s1.capacity());
    println!("Stack is empty: {}", s1.is_empty());
    println!("Stack is full: {}", s1.is_full());
    println!("Stack is pop: {:?}", s1.pop());
    s1.display();
}
