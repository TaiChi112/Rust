pub struct Customer {
    id: u8,
    name: String,
}
impl Customer {
    pub fn new(id: u8, name: String) -> Self {
        Self { id, name }
    }
    pub fn hello(&self) {
        println!("My id {} & My name's {}!", self.id, self.name);
    }
}

