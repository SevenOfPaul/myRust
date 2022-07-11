fn test(number:i32)->i32{
 print!("你好{}",number);
 return number+1
}
fn main(){
    let mut a:i32=16;
    let b:i32=test(a);
    print!("{}",b)
}
