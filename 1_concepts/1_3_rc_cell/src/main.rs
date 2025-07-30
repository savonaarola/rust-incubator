use std::{cell::RefCell, rc::Rc};


#[derive(Debug)]
struct  GlobalStack<T>{
    stack: Rc<RefCell<Vec<T>>>
}

impl<T> GlobalStack<T>{
    fn new() -> Self{
        GlobalStack { stack: Rc::new(RefCell::new(Vec::new())) }
    }

    fn push(&self, value: T){
        self.stack.borrow_mut().push(value);
    }
    fn pop(&self) -> Option<T>{
        self.stack.borrow_mut().pop()
    }
    fn len(&self) -> usize{
        self.stack.borrow().len()
    }
}
impl<T> Clone for GlobalStack<T>{
    fn clone(&self) -> Self {
        GlobalStack { stack: Rc::clone(&self.stack) }
    }
}
fn main() {
    let gs1: GlobalStack<i32> = GlobalStack::new();
    gs1.push(43);
    let gs2 = gs1.clone();
    gs2.push(32);
    gs2.push(21);
    gs1.push(13);
    println!("{}",gs2.pop().unwrap());
    println!("{}",gs1.pop().unwrap());
    println!("{}",gs1.len());
    println!("{:?}",gs1);
}
