use std::time::SystemTime;

// #[derive(Debug)]
// enum List<T>{
//     Cons(T,Rc<List<T>>),
//     Nil,
// }
// use crate::List::{Cons,Nil};
// use std::rc::Rc;
fn func(v:i32,oldvalue:i32)->i32{
if v==0 {
    return oldvalue
}else{
    func(v-1,oldvalue+v)
}
}
fn main() {

//  let a=Rc::new(Cons(2,Rc::new(Cons(1,Rc::new(Nil)))));
//  let b=Cons(2,Rc::clone(&a));
//  let c=Cons(2,Rc::clone(&a));
//  println!("{:?}",Rc::strong_count(&a));
let sy_time = SystemTime::now();
let sum=func(15000,0);
println!("{:?}ms,{}",SystemTime::now().duration_since(sy_time).unwrap(),sum)
}
