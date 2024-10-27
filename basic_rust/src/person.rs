use crate::specking::Specking;
pub struct Person2 {
    pub id: u8,
    pub name: String,
}
pub struct Person3 {
    id: u8,
    name: String,
}
impl Person3 {
    pub fn new(id: u8, name: String) -> Self {
        Self { id, name }
    }
    pub fn hello1() {} //module of struct that like constructor in C++
    pub fn hello2(&self) {
        println!("My id {} & My name's {}!", self.id, self.name);
    } // behavior it's method Person3
}
// implement trait that like interface in TS
impl Specking for Person3 {
    fn speck(&self) {
        println!("{} specking...", self.name);
    }
}
