# Cargo #
1. 创建项目 `cargo new +name`
2. 检查代码 `cargo check`
3. 代码编译 `cargo build`
4. 代码编译执行` cargo run`
# 变量 #
## 数据类型 ##
1. bool
## 变量定义 ##
+ rust声明变量需要使用mut 否则为常量 不可修改
1. 常量
```rust
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