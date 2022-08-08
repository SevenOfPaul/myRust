#[derive(Debug)]
enum List{
    Cons(i32,RefCell<Rc<List>>),
    Nil
}
use std::{rc::Rc, cell::RefCell};
use crate::List::Cons;
fn main() {
 let list=Cons(6, RefCell::new(Rc::new(Cons(7,RefCell::new(Nil)))));
}
