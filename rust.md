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
7. 
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