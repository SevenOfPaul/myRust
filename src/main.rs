fn add(x:i32)->i32{
    x+1
}
fn do_twice(f:fn(i32)->i32,val:i32)->i32{
   return f(val);
}
fn wapper_func<T>(t:T,v:i32)->i32
    where T:Fn(i32)->i32{
   return t(v);
}
fn return_clo()->fn(i32)->i32{
    |n|n
}
#[derive(Debug)]
struct MyType{

}
trait A{
 fn print(&self);
}

impl A for MyType {
    fn print(&self){
        println!("{:?} of A",self);
    }
}
impl MyType{
    fn print(&self){
        println!("{:?}of Self",self);
    }
}
fn main() {
    let func=|val:i32|->i32{
        val+1
    };
    let f=&return_clo() as *const fn(i32) -> i32;
    unsafe {
        let h= *f;
        println!("{}", h(7));
    }
    let m=MyType{};
    m.print();
    //完全限定语法
    <MyType as A>::print(&m);
    println!("{}", do_twice(add,12));
    println!("{}",0.1+0.2);
   println!("{}",wapper_func(func,17));
    // println!("{:?}",return_clo());
    println!("hello world");
}