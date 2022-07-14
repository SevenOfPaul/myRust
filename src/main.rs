fn test(number:&mut isize)->isize{
  *number+=1;
  return *number
}
fn main(){
 let mut two:isize=2;
 let three=test(&mut two);
 let s=String::from("hello world!");
  let s2=&s[0..s.len()];
  let hw:[char;5]=['h','e','l','l','o'];
  let  mut wh:&[char]=&hw[0..=hw.len()-1];
  print!("\ntwo={}\nthree={}\ns2={}",two,three,s2);
  println!("{:?}",wh);
}
