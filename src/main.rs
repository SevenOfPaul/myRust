#[derive(Debug)]
enum List{
    //Cons(i32,RefCell<Rc<List>>),
    Cons(i32,RefCell<Weak<List>>),
    Nil
}
use std::rc::Weak;
use std::{rc::Rc, cell::RefCell};
use crate::List::Cons;
use crate::List::Nil;
impl List{
    fn tail(&self)->Option<&RefCell<Weak<List>>>{
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
 let list=Rc::new(Cons(6,RefCell::new(Weak::new())));
 if let Some(i)=  list.tail_number()  {
    println!("{:?}",i);
}
 let list2=Rc::new(Cons(2,RefCell::new(Weak::new())));
 if let Some(i)=  list.tail()  {
  *i.borrow_mut()=Rc::downgrade(&list2);
}
if let Some(i)=  list2.tail()  {
    *i.borrow_mut()=Rc::downgrade(&list);
  }
 println!("{:?}&&{:?},{:?}",&list.tail(),&list.tail_number(),list);
}