// encapsulation
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn distance(&self, other: &Point) -> f64 {
        let x_squared = (self.x - other.x) * (self.x - other.x);
        let y_squared = (self.y - other.y) * (self.y - other.y);
        ((x_squared + y_squared) as f64).sqrt()
    }
}
struct Line {
    start: Point,
    end: Point,
}
impl Line {
    fn new(start: Point, end: Point) -> Line {
        Line {
            start,
            end,
        }
    }
}
struct Person {
    name: String,
    age: u32,
}
impl Person {
    fn new(name: &str, age: u32) -> Person {
        Self {
            name: name.to_string(),
            age,
        }
    }
    fn introduce(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

// inheritance
trait Shape {
    fn area(&self) -> f64;
}
struct Circle {
    radius: f64,
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
fn print_area(shape: &impl Shape) {
    println!("Area: {}", shape.area());
}
// abstraction
trait Speak {
    fn speak(&self);
}
struct Dog;
struct Cat;
impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
impl Speak for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}
fn make_sound<T: Speak>(animal: &T) {
    animal.speak();
}
trait Animal {
    fn make_sound(&self);
}
impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}
impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow!");
    }
}
 
fn main() {
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    println!("Distance: {}", p1.distance(&p2));

    let l1 = Line::new(p1, p2);
    println!("Distance: {}", l1.start.distance(&l1.end));

    let p = Person::new("John", 25);
    p.introduce();

    let c = Circle { radius: 5.0 };
    let r = Rectangle {
        width: 5.0,
        height: 10.0,
    };
    print!("Circle ");
    print_area(&c);
    print!("Rectangle ");
    print_area(&r);

    let d = Dog;
    let c = Cat;
    make_sound(&d);
    make_sound(&c);

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in animals {
        animal.make_sound();
    }
}
