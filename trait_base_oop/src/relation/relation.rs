pub struct Point {
    x: i32,
    y: i32,
}
pub struct Book {
    name: String,
    author: String,
}
pub struct Door {
    width: i32,
    height: i32,
    color: String,
}

trait Vehicle {
    fn name(&self) -> &str;
    fn describe(&self) {
        println!("{} is a vehicle", self.name());
    }
}
trait Drive {
    fn drive(&self, distance: u32);
}
trait Fuel {
    fn fuel_type(&self) -> &str;
}
pub struct Car {
    name: String,
    fuel: String,
}
pub struct Bicycle {
    name: String,
}
impl Vehicle for Car {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Drive for Car {
    fn drive(&self, distance: u32) {
        println!("The car has driven {} miles", distance);
    }
}
impl Fuel for Car {
    fn fuel_type(&self) -> &str {
        &self.fuel
    }
}
impl Vehicle for Bicycle {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Drive for Bicycle {
    fn drive(&self, distance: u32) {
        println!("The bicycle has driven {} miles", distance);
    }
}
fn move_vehicle(vehicle: &dyn Drive, distance: u32) {
    vehicle.drive(distance);
}
