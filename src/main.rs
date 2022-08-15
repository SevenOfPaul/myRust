struct Point{
   x:i32,
   y:i32
}
fn main() {
 let p= Point{x:1,y:2};
 let Point{x:a,y:b}=p;
 let _x=5;
 assert_eq!(1,a);
 assert_eq!(2,b);
 println!("{}-{}_{}",a,b,_x);
 let numbers=(1,32,4,487,87,87,12);
 match numbers {
   (first,second,third,..,last) if first!=0=> println!("{},{}",first,last),
   _=>println!("出错了")
 }
 match numbers {
   (first,second,third,..,last) if first!=0=> println!("{},{}",first,last),
   _=>println!("出错了")
 }
}