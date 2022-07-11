fn test(a:isize)->isize{
 print!("你好{}",a);
  a+1
}
fn main(){
    let a:isize=16;
    
    let b:isize=test(a);
    let s:String=String::from("字符串");
    let c=&s;
    print!("b={}&a={}",b,c)
}
