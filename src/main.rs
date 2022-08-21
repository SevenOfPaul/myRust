use std::process::Command;
#[derive(Debug)]
enum test{
  name
}
extern "C"{
  fn abs(input:i32)->i32;
  fn rand()->i32;
}
static mut HELLO_WORLD:&str="hello";
fn change_static(){
  unsafe{
    HELLO_WORLD="hello world!";
  }
}
fn main() {
  let mut num=5;
  unsafe {
    let r1:*const i32=&num ;
    let str="string";
    println!("{}", str.as_ptr() as u8);
    let c1=r1.clone();
    let r2:*const u8=str.as_ptr();
    println!("{}",*r2);
    // Command::new("cmd").arg("/c").arg("").output().expect("cmd exec error!");
   println!("{}__{}",abs(12),rand());
    change_static();
    println!("{:?}__{}",*r1,HELLO_WORLD);
    println!("{:?}",test::name)
}
}