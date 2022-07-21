fn largest<T:std::cmp::PartialOrd>(list:&[T])->&T{
    let mut largest = &list[0];
    for item in list.iter() {
        if &item > &largest {
           largest = &item;
        }
    }
    &largest

}
#[derive(Debug)]
  struct Person<T>{
    name:T,
    age:i32,
    addr:T
}
#[derive(Debug)]
  struct Point<T>{
   x:T,
   y:T
}
fn main(){
    let a_list=vec![1,2,4,7,8,7,11,4,1];
    let b_list=vec!['a','c','d','r','f','k'];
   let p=Person{
    name:String::from("张思"),
    age:12,
    addr:String::from("黑龙江")
   };
 let number=Point{x:1,y:2};
     println!("{:?}",largest(&a_list));
     println!("{:?}",largest(&b_list));
     println!("{:?}",p);
     println!("{:?}",&number);
}