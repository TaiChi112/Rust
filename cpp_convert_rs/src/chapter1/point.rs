pub struct Point {
    name: String,
    x: i8,
    y: i8,
}
impl Point {
    pub fn default() -> Self {
        Point {
            name: "Origin".to_string(),
            x: 0,
            y: 0,
        }
    }
    pub fn new(name: &str, x: i8, y: i8) -> Self {
        Point {
            name: name.to_string(),
            x,
            y,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_x(&self) -> i8 {
        self.x
    }
    pub fn get_y(&self) -> i8 {
        self.y
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn set_x(&mut self, x: i8) {
        self.x = x;
    }
    pub fn set_y(&mut self, y: i8) {
        self.y = y;
    }
    pub fn show(&self) {
        println!("Point: {} ({}, {})", self.name, self.x, self.y);
    }
}
impl Drop for Point {
    fn drop(&mut self) {
        println!("Dropping Point: {}", self.name);
    }
}
