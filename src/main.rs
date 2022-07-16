
fn main() {
 let mut v:Vec<i32>=Vec::new();
 let _v2 = vec![1, 2, 3];
 v.push(1);
  if let Some(i) = &_v2.get(0){
    println!("{:?}",i)
  }
  if let None = &_v2.get(4){
    println!("出错啦")
  }

}
