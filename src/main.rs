use std::mem;

struct Nothing; // 无字段意味着没有大小
// 所有字段都无大小意味着整个结构体无大小
struct LotsOfNothing {
    foo: Nothing,
    qux: (),      // 空元组无大小
    baz: [u8; 0], // 空数组无大小
}
struct A<'a>{
    _a:i32,
    _b:&'a [u8],
    _c:&'a u32
}
trait AB{
    fn test(){}
}
fn main(){
let a=A{_a:1,_b:&[1,8,9,10],_c:&1};
    let slice=&a._b[0..4];
    println!("{}-{}",mem::size_of_val(&a._b),mem::size_of_val(&a._c));
     println!("{}--{}",mem::size_of_val(slice),mem::size_of::<&[u8]>());
}