type MyI32=i8;
use std::result;
use std::io::Error;
type Result<T>=result::Result<T,Error>;
fn main() {
let  a=16;
let b:MyI32=17;
// if let i=b{
//    println!("{}",a+b);
// }
println!("{}",a+b);
}