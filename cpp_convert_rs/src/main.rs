use cpp_convert_rs::chapter1::point::Point;
use cpp_convert_rs::chapter2::rectangle::Rectangle;

fn main() {
    let (x, y) = (1, 2);
    let name = "x";
    let p1 = Point::new(&name,x, y);
    p1.show();

    let p3 = Point::default();
    p3.show();

    let r1 = Rectangle::new(112,118);
    r1.show();
}
