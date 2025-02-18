use cpp_convert_rs::chapter1::point::Point;
use cpp_convert_rs::chapter2::rectangle::Rectangle;

fn main() {
    let (x, y) = (1, 2);
    let name = "x";
    let p1 = Point::new(&name,x, y);
    // p1.show();
    println!("get point name: {} ", p1.get_name());
    println!("get point x: {} ", p1.get_x());
    println!("get point y: {} ", p1.get_y());

    let p3 = Point::default();
    // p3.show();
    println!("get point name: {} ", p3.get_name());
    println!("get point x: {} ", p3.get_x());
    println!("get point y: {} ", p3.get_y());

    let r1 = Rectangle::new(112,118);
    // r1.show();
    println!("get rectangle width: {} ", r1.get_width());
    println!("get rectangle height: {} ", r1.get_height());
}
