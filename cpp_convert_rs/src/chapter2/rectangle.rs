pub struct Rectangle {
    width: i8,
    height: i8,
}
impl Rectangle{
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
}