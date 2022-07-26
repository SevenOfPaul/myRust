use std::{time::Instant, string};
fn longest <'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x  
    } else {
        y
    }
}
#[derive(Debug)]
struct A<'a>{
    name:&'a String
}
impl<'a> A<'a>{
 fn do_some(&self,s:&'a str)->&'a str{
      s
 }
}
fn main(){
    let now = Instant::now();
    let s: &'static str = "我没啥优点，就是活得久，嘿嘿";
let r;
{
    r=s;
};
let n=String::from("hello");
let a=A{name:&n};
println!("{:?}",longest("你好啊","不好"));
 println!("{:?},{:?}", now.elapsed(),a);
}