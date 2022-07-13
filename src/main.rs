fn test(number:&mut isize)->isize{
  *number+=1;
  return *number
}
fn main(){
 let mut two:isize=2;
 let three=test(&mut two);
 let s=String::from("hello world!");
  let s2=&s[0..s.len()];
  //在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置，
  //也就是 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节,下面的代码就会崩溃:
  print!("two={}\nthree={}\ns2={}",two,three,s2);
}
