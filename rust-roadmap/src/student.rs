
pub struct Student {
    id: u8,
    name: String,
}
// impl Student {
//     pub fn new(id: u8, name: String) -> Student {
//         Student { id, name }
//     }
// }
impl Student {
    pub fn new(id: u8, name: String) -> Self {
        Self { id: id, name: name }
    }
    // pub fn hello() {
    //     println!("Hello {}", self.name);
    // }
    pub fn hello(&self) {
        println!("id : {} my name's : {}", self.id, self.name);
    }
}
// impl Student {
//     pub fn new(id: u8, name: String) -> Self {
//         Self { id, name }
//     }
// }
// pub struct Student {
//     pub id: u8,
//     pub name: String,
// }
