const MAX_POINT:u32=10000;
fn main() {
    let b:bool=true;
    let mut a:u32=1; 
    let c:char='你';
    let d:char='好';
    let e:[char;3]=['你','好','吗'];
    if b{
        println!("Hello, rust!{}&{}",a,MAX_POINT);
    }
    a=3;
    print!("Hello, rust!{},{}{}&{}\n{}{}{}?",a,c,d,isize::max_value(),e[0],e[1],e[2]);
}
