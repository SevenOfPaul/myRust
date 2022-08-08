#[derive(Debug)]
enum List{
    Cons(i32,RefCell<Rc<List>>),
    Nil
}
use std::{rc::Rc, cell::RefCell};
use crate::List::Cons;
use crate::List::Nil;
impl List{
    fn tail(&self)->Option<&RefCell<Rc<List>>>{
               match self {
                Cons(number, item) => Some(item),
                Nil => None,
            }
    }
    fn tail_number(&self)->Option<i32>{
        match self {
         Cons(number, item) => Some(*number),
         Nil => None,
     }
}
}
fn main() {
 let list=Cons(6,RefCell::new(Rc::new(Nil)));
 if let Some(i)=  list.tail_number()  {
    println!("{:?}",i);
}
 let list2=Cons(2,RefCell::new(Rc::clone(&Rc::new(list))));
 if let Some(i)=  list.tail()  {
  *i.borrow_mut()=Rc::clone(&Rc::new(list2));
}
 println!("{:?}&&{:?},{:?}",&list.tail(),&list.tail_number(),list);
}
