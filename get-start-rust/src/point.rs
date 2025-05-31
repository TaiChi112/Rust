pub struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Self { x: x, y: y }
    }
    pub fn distance(&self, other: &Point) -> f64 {
        let x_squared = (self.x - other.x) * (self.x - other.x);
        let y_squared = (self.y - other.y) * (self.y - other.y);
        ((x_squared + y_squared) as f64).sqrt()
    }
}
