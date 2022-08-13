
fn main() {
   let mut a=1;
   match a {
       0=>println!("0&"),
       1=> println!("1"),
       _ =>println!("no")
   }
   let mut color:Option<&str>=None;
   if let Some(c)=color{
      println!("{}",c);
   }else if a==1{
      println!("是1吗？");
   }
   while let None = color {
       println!("不存在");
       a+=1;
       if a==7{
         color=Some("你好");
         println!("{:?}",&color);
       }
   }
   while let None = color {
      println!("不存在");
      a+=1;
      if a==7{
        color=Some("你好");
        println!("{:?}",&color);
      }
  }
  let v=vec![1,2,3,16,5,7,10];
  for (index,value) in v.iter().enumerate(){
   println!("{}(){}",index,value);
  }
  let (x,_,z)=(1,2,3);
  println!("{},{}",x,z);
}