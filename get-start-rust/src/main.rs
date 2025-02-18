struct Point{
    x: i32,
    y: i32
}
impl Point{
    fn new(x: i32, y: i32) -> Point{
        Point{x: x, y: y}
    }
    fn distance(&self, other: &Point) -> f64{
        let x_squared = (self.x - other.x) * (self.x - other.x);
        let y_squared = (self.y - other.y) * (self.y - other.y);
        ((x_squared + y_squared) as f64).sqrt()
    }
}
fn main(){
    let p1 = Point::new(1, 2);
    let p2 = Point::new(3, 4);
    println!("Distance: {}", p1.distance(&p2));
}