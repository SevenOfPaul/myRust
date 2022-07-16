enum conText{
    Text(String),
    Float(f32),
    Int(i8)
}
use std::collections::HashMap;
fn main() {
    let mut v:Vec<i32>=Vec::new();
    let mut _v2 = vec![1, 2, 3];
    let _v3:Vec<conText>=vec![
    conText::Text("你好".to_string()),
    conText::Int(1),
    conText::Text("很灵活".to_string()),
    conText::Float(0.7)
    ];
    v.push(1);
     if let Some(i) = &_v2.get(0){
       println!("{:?}",i)
     }
     if let None = &_v2.get(4){
       println!("出错啦")
     }
     for v in &mut _v2{
       *v+=1;
       println!("{:?}",v)
     }
     let v3=&_v3[0];
  if let  conText::Text(v)=&_v3[0]{
    println!("{:?}",v)
  }
//    let frist=&_v2[0];
//    _v2.push(7);
//     println!("{}",frist)    
//空字符串
let mut s=String::new();
let mut s2=String::from("hello");
let mut s3="world".to_string();
let sArray:[&String;3]=[&s,&s2,&s3];
let  mut index=0;
loop {
    if index>=sArray.len(){
        break;
    }
     println!("{}",sArray[index]);
     index+=1;
 }
   s.push_str("你好");
   let s4="hello".to_string()+&"rust".to_string()+&s3;
   println!("{}",s4);
   let str=&s4[0..=2];
   //打印指针
   let point=&str as *const &str as usize;
   println!("{:X}",point);
   for c in s4.chars(){
    println!("c={}",c)
   }
   let mut scores:HashMap<String,i32>=HashMap::new();
   scores.insert("blue".to_string(), 10);
   scores.insert("green".to_string(), 12);
//迭代器创建
   }
   