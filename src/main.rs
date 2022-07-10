const MAX_POINT:u32=10000;
fn main() {
    let b:bool=true;
    let mut a:u32=1; 
    let c:char='你';
    let d:char='好';
    let e:[char;3]=['你','好','吗'];
    let t:(u32,i32,char)=(12,-12,'t');
    let condition=true;
    if b{
        println!("Hello, rust!{}&{}",a,MAX_POINT);
    }
    a=3;
    let number=if condition {
        5
    } else {
        6
    };
    println!("Hello, rust!{},{}{}&{}",a,c,d,isize::max_value());
    for i in &e{
          print!("{}",i)
       
    }
    for i in 0..10{
        println!("{}",i);
    }
    print!("?{}{}{},{}",t.0,t.1,t.2,number);
    let f=loop{
        a+=1;
        print!("{}",a);
        if a>=100{
            break a;
        }
       };
       print!("{}",f);
}
