fn test(a:&mut isize)->isize{
 print!("你好{}",a);
  *a+=1;
  return *a;
}
fn main(){
    let mut a:isize=16;
    let b:isize=test(&mut a);
    let mut s:String=String::from("字符串");
    s.push_str("你好吗");
    let s2=s.clone();
    let s = String::from("hello");  // s 进入作用域
    let s3=&s;
    print!("b={}&a={}&s2={}s3={}",b,a,s2,*s3)
}
