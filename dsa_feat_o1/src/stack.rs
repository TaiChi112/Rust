struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

pub struct Stack<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Self{
        Stack { head: None }
    }
    pub fn push(&mut self,data :T){
        let new_node = Box::new(
            Node{
                data,
                next: self.head.take(),
            }
        );
        self.head = Some(new_node);
    }
    pub fn pop(&mut self)-> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }
    pub fn peek(&self)-> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn update(&mut self,new_data:T){
        if let Some(node) = self.head.as_mut() {
            node.data = new_data;
        }
    }
    pub fn clear(&mut self) {
        self.head = None;
    }

}