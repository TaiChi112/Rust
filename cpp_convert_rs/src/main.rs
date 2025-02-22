use cpp_convert_rs::chapter1::point::Point;
use cpp_convert_rs::chapter2::rectangle::Rectangle;
use cpp_convert_rs::chapter3::my_rect::MyRect;
use cpp_convert_rs::chapter4::circle::Circle;

fn main() {
    let (x, y) = (1, 2);
    let name = "x";
    let p1 = Point::new(&name, x, y);
    // p1.show();
    println!("get point name: {} ", p1.get_name());
    println!("get point x: {} ", p1.get_x());
    println!("get point y: {} ", p1.get_y());

    let p3 = Point::default();
    // p3.show();
    println!("get point name: {} ", p3.get_name());
    println!("get point x: {} ", p3.get_x());
    println!("get point y: {} ", p3.get_y());

    println!(" is {}", p3.dot_product(&p3));

    let r1 = Rectangle::new(112, 118);
    // r1.show();
    println!("get rectangle width: {} ", r1.get_width());
    println!("get rectangle height: {} ", r1.get_height());

    let mut my_rect = MyRect::with_size(3, 3);
    my_rect.set_rect_at(0, 0, 1, 2);
    my_rect.set_rect_at(0, 1, 3, 4);
    my_rect.set_rect_at(1, 0, 5, 6);
    my_rect.set_rect_at(1, 1, 7, 8);
    my_rect.set_rect_at_object(2, 1, Rectangle::new(11, 12));
    my_rect.show();

    let c1 = Circle::new(p1, 5.2);
    c1.show();

    println!();
}
