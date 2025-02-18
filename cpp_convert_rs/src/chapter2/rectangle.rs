pub struct Rectangle {
    width: i8,
    height: i8,
}
impl Rectangle{
    pub fn default() -> Self{
        Rectangle{
            width: 0,
            height: 0,
        }
    }
    pub fn new(width: i8, height: i8) -> Self{
        Rectangle{
            width,
            height,
        }
    }
    pub fn area(&self) -> i8{
        self.width * self.height
    }
    pub fn perimeter(&self) -> i8{
        2 * (self.width + self.height)
    }
    pub fn show(&self) {
        println!("Rectangle: {} x {} (Area: {}, Perimeter: {})", self.width, self.height, self.area(), self.perimeter());
    }
    pub fn set_width(&mut self, width: i8){
        self.width = width;
    }
    pub fn set_height(&mut self, height: i8){
        self.height = height;
    }
    pub fn get_width(&self) -> i8{
        self.width
    }
    pub fn get_height(&self) -> i8{
        self.height
    }
}
impl Drop for Rectangle {
    fn drop(&mut self) {
        println!("Dropping Rectangle: {} x {}", self.width, self.height);
    }
    
}