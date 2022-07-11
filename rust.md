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
 return number+1;
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
7. 当所有者(变量)离开作用域范围时，这个值将被丢弃(drop)