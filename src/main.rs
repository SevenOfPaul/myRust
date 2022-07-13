fn test(number:&mut isize)->isize{
  *number+=1;
  return *number
}
fn main(){
 let mut two:isize=2;
 let three=test(&mut two);
  print!("two={}\nthree={}",two,three);
}
