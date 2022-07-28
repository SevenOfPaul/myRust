// struct Cacher<T:Fn(u32)->u32>{
// calcuation:T,
// value:u32,
// }
// impl<T> Cacher<T:Fn(u32)->u32>{
//     fn new(calcuation:T)->Cacher<T>{
        
//     }
// }
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool+Copy,
{
    println!("{}？", func(3));
    println!("{}？", func(4));
}
fn fn_mut<F>(func: F)
where
    F: FnMut(usize) -> bool,
{
    println!("{}？", func(3));
    println!("{}？", func(4));
}

fn main(){
let use_closure=|n:i32|->i32{
    // println!("this is closure");
    return n
};

let closure2=|n:i32|{n+1};
let closure3=|n|n+1;
let v=closure3(7);
fn add(n:u8)->u8{
    n+1
}
let h = vec![1, 2, 3];
fn_once(|z|z==h.len());
println!("闭包{}",use_closure(6));
println!("{}",add(6));
}