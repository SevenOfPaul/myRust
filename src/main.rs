use std::path::Components;

use gui::{Screen,Button,selectBox};
fn main() {
   let mut s=Screen{
      components:vec![]
   };
   let b1=Button{width:200,height:200,label:String::from("确认")};
   let b2=Button{width:200,height:200,label:String::from("取消")};
   s.components.push(Box::new(b1));
   s.components.push(Box::new(b2));
   s.run();
}