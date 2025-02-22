#[derive(Clone)]
pub struct Rectangle {
    width: i32,
    height: i32,
}
impl Rectangle{
    pub fn default() -> Self{
        Rectangle{
            width: 0,
            height: 0,
        }
    }
    pub fn new(width:i32 , height: i32) -> Self{
        Rectangle{
            width,
            height,
        }
    }
    pub fn area(&self) -> i32{
        self.width * self.height
    }
    pub fn perimeter(&self) -> i32{
        2 * (self.width + self.height)
    }
    pub fn show(&self) {
        println!("Rectangle: {} x {} (Area: {}, Perimeter: {})", self.width, self.height, self.area(), self.perimeter());
    }
    pub fn set_width(&mut self, width: i32){
        self.width = width;
    }
    pub fn set_height(&mut self, height: i32){
        self.height = height;
    }
    pub fn get_width(&self) -> i32{
        self.width
    }
    pub fn get_height(&self) -> i32{
        self.height
    }
}
impl Drop for Rectangle {
    fn drop(&mut self) {
        println!("Dropping Rectangle: {} x {}", self.width, self.height);
    }
    
}