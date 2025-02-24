pub struct Book {
    title: String,
    author: String,
    pages: u32,
    price: f32,
}
impl Book{
    pub fn new(title: String, author: String, pages: u32, price: f32) -> Book {
        Book {
            title,
            author,
            pages,
            price,
        }
    }
    pub fn display(&self) {
        println!(
            "Title: {}, Author: {}, Pages: {}, Price: {}",
            self.title, self.author, self.pages, self.price
        );
    }
}