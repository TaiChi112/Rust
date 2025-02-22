use crate::chapter2::rectangle::Rectangle;

pub struct MyRect {
    rects: Vec<Vec<Rectangle>>,
    rows: usize,
    cols: usize,
}
impl MyRect {
    pub fn new() -> Self {
        MyRect {
            rects: vec![],
            rows: 0,
            cols: 0,
        }
    }
    pub fn with_size(rows: usize, cols: usize) -> Self {
        let default_rect = Rectangle::new(0, 0);
        MyRect {
            rects: vec![vec![default_rect; cols]; rows],
            rows,
            cols,
        }
    }
    pub fn set_rect_at(&mut self, row: usize, col: usize, width: u32, height: u32) {
        if row < self.rows && col < self.cols {
            self.rects[row][col] =
                Rectangle::new(width.try_into().unwrap(), height.try_into().unwrap());
        }
    }
    pub fn set_rect_at_object(&mut self, row: usize, col: usize, rect: Rectangle) {
        if row < self.rows && col < self.cols {
            self.rects[row][col] = rect;
        }
    }
    pub fn show(&self) {
        for row in &self.rects {
            for rect in row {
                print!("[{}x{}]", rect.get_width(), rect.get_height());
            }
            println!();
        }
    }
}
// impl Drop for MyRect {
//     fn drop(&mut self) {
//         println!("MyRect is being dropped");
//     }
// }
