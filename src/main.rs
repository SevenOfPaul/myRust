fn test(a:isize)->isize{
 print!("你好{}",a);
  a+1
}
fn main(){
    let a:isize=16;
    let b:isize=test(a);
    let mut s:String=String::from("字符串");
    s.push_str("你好吗");
    let s2=s.clone();
    let s = String::from("hello");  // s 进入作用域
    print!("b={}&a={}&s2={}",b,s,s2)
}
