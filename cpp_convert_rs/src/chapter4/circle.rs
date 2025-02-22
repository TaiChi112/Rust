
use crate::chapter1::point::Point;

pub struct Circle{
    center: Point,
    radius: f64,
}
impl Circle{
    pub fn default() -> Circle{
        Circle{
            center: Point::default(),
            radius: 0.0,
        }
    }
    pub fn new(center: Point, radius: f64) -> Circle{
        Circle{
            center,
            radius,
        }
    }
    pub fn is_inside(&self, p: Point) -> bool{
        let dx = self.center.get_x() - p.get_x();
        let dy = self.center.get_y() - p.get_y();
        let distance = ((dx * dx + dy * dy) as f64).sqrt();
        distance < self.radius
    }
    pub fn show(&self){
        println!("Center: ({}, {}), Radius: {}", self.center.get_x(), self.center.get_y(), self.radius);
    }
    pub fn get_center(&self) -> &Point{
        &self.center
    }
    pub fn get_radius(&self) -> f64{
        self.radius
    }
    pub fn set_center(&mut self, center: Point){
        self.center = center;
    }
    pub fn set_radius(&mut self, radius: f64){
        self.radius = radius;
    }
}
impl Drop for Circle{
    fn drop(&mut self){
        println!("Dropping Circle");
    }
}