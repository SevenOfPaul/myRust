#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use std::ops::Deref;
struct D{
    num:i8
}
impl D{
   fn new(num:i8)->D{
     D{num}
   }
}
impl Deref for D{
    type Target=i8;
    fn deref(&self) -> &Self::Target {
       &self.num
    }
}
struct MyBox<T>{
    value:T
}
impl<T> MyBox<T>{
    fn new(value:T)->MyBox<T>{
        MyBox { value }
    }
}
impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self)->&Self::Target{
        &&self.value
    }
}
impl<T> Drop for MyBox<T>{
    fn drop(&mut self) {
        println!("对像销毁");
    }
}
fn main() {
    //b存储在栈上 5存储在堆上 b指向5
    let b = Box::new(12);
    use List::Cons;
    use List::Nil;
    let d=D::new(12);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let c=*d;
    let e = MyBox::new(10);
    let f=*e;
    println!("{:?}&{:?}", b, list);
    println!("{:?}&{:?}", b,c);
    println!("{:?}",f)
}
