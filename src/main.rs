const MAX_POINT:u32=10000;
fn main() {
    let b:bool=true;
    let mut a:u32=1; 
    let c:char='你';
    let d:char='好';
    let e:[char;3]=['你','好','吗'];
    let t:(u32,i32,char)=(12,-12,'t');
    if b{
        println!("Hello, rust!{}&{}",a,MAX_POINT);
    }
    a=3;
    println!("Hello, rust!{},{}{}&{}",a,c,d,isize::max_value());
    for i in e{
          print!("{}",i)
       
    }
    print!("?{}{}{}",t.0,t.1,t.2)
}
