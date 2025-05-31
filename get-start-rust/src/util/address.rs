pub struct Address {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
}
impl Address {
    pub fn new(street: String, city: String, state: String, zip: String) -> Self {
        Self {
            street,
            city,
            state,
            zip,
        }
    }
    pub fn display(&self) {
        println!(
            "Street: {}, City: {}, State: {}, Zip: {}",
            self.street, self.city, self.state, self.zip
        );
    }
}
