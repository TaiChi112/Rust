pub mod stack;
use stack::Stack;
fn main() {
    let mut stack: Stack<i32> = Stack::new();
    stack.push(112);
    stack.push(113);
    stack.push(114);
    stack.push(115);

    println!("Stack after pushing elements:");
    while let Some(value) = stack.pop() {
        println!("{}", value);
    }
    println!("Stack is empty: {}", stack.is_empty());
    stack.push(116);
    println!("Stack after pushing 116:");
    if let Some(top) = stack.peek() {
        println!("Top element: {}", top);
    } else {
        println!("Stack is empty, no top element.");
    }
    stack.update(117);
    println!("Stack after updating top element to 117:");
    if let Some(top) = stack.peek() {
        println!("Top element: {}", top);
    } else {
        println!("Stack is empty, no top element.");
        return;
    }
    stack.clear();
    println!("Stack cleared.");
    println!("Stack is empty: {}", stack.is_empty());
    if let Some(top) = stack.peek() {
        println!("Top element after clearing: {}", top);
    } else {
        println!("Stack is empty, no top element.");
        return;
    }
}