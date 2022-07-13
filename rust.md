# Cargo #
1. 创建项目 `cargo new +name`
2. 检查代码 `cargo check`
3. 代码编译 `cargo build`
4. 代码编译执行` cargo run`
# 变量 #
## 数据类型 ##
1. bool
2. char 字符 rust中字符为32位 可以存储汉字
3. i8 i16 i32 i4 有符号整形
4. u8 u15 u32 u64 无符号整型
5. f32 f64 浮点型
6. isize usize自适应类型 针对不同平台分配不同的大小
7. [type;size]=[data] 数组`let e:[char;3]=['你','好','吗'];`
8. 元组 `let t:(u32,i32,char)=(12,-12,'t'); print!(t.0) `
9. 字符串类型 `let s:String=String::from("字符串");`
## 变量定义 ##
+ rust声明变量需要使用mut 否则为常量 不可修改
1. 常量
```rust
const MAX_POINT:U32=10000;
fn main() {
    let a:u32=1;
    println!("Hello, rust!{}",a);
}
```
2. 变量
```rust
fn main() {
    let mut a:u32=1;
    println!("Hello, rust!{}",a);
}
```
3. 元组解构
```rust
let t:(u32,i32,char)=(12,-12,'t')
let (x,y,z)=t;
 print!(t.0)
 print!(x)
 ```
## 流程控制 ##
1. if
```rust
 if true{
   println!("Hello, rust!{}&{}",a,MAX_POINT);
}
```
2.if 语法糖
```rust
let number=if condition {
     5
    } else {
    6
};
//注意不要加分号 且 if和else的返回值必须是同一类型
```
2.loop
```rust
  let f=loop{
        a+=1;
        print!("{}",a);
        if a>=100{
            break a;
        }
       };
       print!("{}",f);
```
3. for
```rust
for i in e{
   print!("{}",i)
}
 ```
4.for 语法糖
```rust
for i in 0..10{
        println!("{}",i);
    }
 ```
 ```rust
 //匿名变量
for _ in 0..10{
        println!("{}","好");
    }
 ```
5. while 循环
```rust
while i!=10{
    i=i+1;
    print!("{}",i)
}
 ```
 ```
 存储在2022-7-8
 ```
 ## 函数 ##
1. 声明函数 `fn name(){}`
```rust
fn test(number:i32)->i32{
 print!("你好{}",number);
}
fn main(){
    let mut a:i32=16;
    let b:i32=test(a);
    print!("{}",b)
}
 ```
 2.在明确返回类型时 rust可以去掉return
```rust
fn test(number:i32)->i32{
 print!("你好{}",number);
  number+1
}
 ```
## rust所有权 ##
1. rust通过所有权机制来控制内存
2. 编译的数据的类型大小是固定的 分配在栈上 不固定则分配在堆上
3. rust和cpp一样 函数参数传入是复制值
4. 字符串类型大小不固定
5. Rust 中每一个值都被一个变量所拥有，该变量被称为值的所有者
6. 一个值同时只能被一个变量所拥有，或者说一个值只能拥有一个所有者
7. 当所有者(变量)离开作用域范围时，这个变量将被丢弃(drop)
```rust
let s1 = String::from("hello");
let s2 = s1;
println!("{}, world!", s1);
 ```
 ```rust
 error[E0382]: use of moved value: `s1`
 --> src/main.rs:5:28
  |
3 |     let s2 = s1;
  |         -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value used here after move
  |
  = note: move occurs because `s1` has type `std::string::String`, which does
  not implement the `Copy` trait
 ```
8. Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何自动的复制都不是深拷贝，可以被认为对运行时性能影响较小。
9.如果我们确实需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的方法。
```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```
10.Rust 有一个叫做 Copy 的特征，可以用在类似整型这样在栈中存储的类型。如果一个类型拥有 Copy 特征，一个旧的变量在被赋值给其他变量后仍然可用。
11.如下是一些 Copy 的类型：
```xml
所有整数类型，比如 u32。
布尔类型，bool，它的值是 true 和 false。
所有浮点数类型，比如 f64。
字符类型，char。
元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
不可变引用 &T ，例如转移所有权中的最后一个例子，但是注意: 可变引用 &mut T 是不可以 Copy的
```
 存储在2022-7-10
 ### 引用 ###
 1.常规引用是一个指针类型，指向了对象存储的内存地址。` let x = 5; let y = &x;`
 2. 可变引用 可以改变原数据的引用
```rust
fn main() {
    let s = String::from("hello");

    change(&s);
}
fn change(some_string: &String) {
    some_string.push_str(", world");
}
 let mut two:isize=2;
 let five=&mut two;
```
 3. 可变引用同时只能存在一个
 4. 可变引用与不可变引用不能同时存在
 5. 悬垂引用也叫做悬垂指针，意思为指针指向某个值后，这个值被释放掉了，而指针仍然存在
```rust
  fn main() {
    let reference_to_nothing = dangle();
}
fn dangle() -> &String {
    let s = String::from("hello");
    &s
}
//编译器会报错
```