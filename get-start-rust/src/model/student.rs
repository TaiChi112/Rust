pub struct Student {
    id: i32,
    name: String,
    age: i32,
}
impl Student {
    pub fn new(id: i32, name: String, age: i32) -> Self {
        Self {
            id: id,
            name: name,
            age: age,
        }
    }
    pub fn introduce(&self) {
        println!("ID: {}, Name: {}, Age: {}", self.id, self.name, self.age);
    }
}
