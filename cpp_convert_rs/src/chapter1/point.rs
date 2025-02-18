#[derive(Default)]
pub struct Point {
    x: i8,
    y: i8,
}
impl Point {
    pub fn default() -> Self {
        Point { x: 0, y: 0 }
    }
    pub fn new(x: i8, y: i8) -> Self {
        Point { x, y }
    }
    pub fn get_x(&self) -> i8 {
        self.x
    }
    pub fn get_y(&self) -> i8 {
        self.y
    }
    pub fn set_x(&mut self, x: i8) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i8) {
        self.y = y;
    }
    pub fn show(&self) {
        println!("x:{},y:{}", self.x, self.y);
    }
}
impl Drop for Point {
    fn drop(&mut self) {
        println!("Point is dropped at x:{},y:{}", self.x, self.y);
    }
}
