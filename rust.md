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
10. 切片 &str ` let s=String::from("hello world!");  let s2=&s[0..5];`
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
### 切片 ###
1. 切片就是对 复杂 类型中某一部分的引用，它看起来像这样
```rust
let s = String::from("hello");
let slice = &s[0..2];//he
let slice = &s[..2];//he
let slice = &s[..=1];
```
2. 在对字符串使用切片语法时需要格外小心，切片的索引必须落在字符之间的边界位置
3. 例如 UTF-8 字符的边界，例如中文在 UTF-8 中占用三个字节,下面的代码就会崩溃
#### 切片方法 ####
1.追加 在字符串尾部可以使用 push() 方法追加字符 char，也可以使用 push_str() 方法追加字符串字面量。
```rust
fn main() {
    let mut s = String::from("Hello ");
    s.push('r');
    println!("追加字符 push() -> {}", s);
    s.push_str("ust!");
    println!("追加字符串 push_str() -> {}", s);
}
 ```
2. 插入 可以使用 insert() 方法插入单个字符 char，也可以使用 insert_str() 方法插入字符串字面量
```rust
fn main() {
    let mut s = String::from("Hello rust!");
    s.insert(5, ',');
    println!("插入字符 insert() -> {}", s);
    s.insert_str(6, " I like");
    println!("插入字符串 insert_str() -> {}", s);
}
```
以上存储在2022-8-13
## 结构体 ##
1.创建结构体与访问结构体
```rust
struct User {
  active: bool,
  username: String,
  email: String,
  id: i32,
}
 let mut user=User{
    active:false,
    username:String::from("大宝贝"),
    email:String::from("3167385363@qq.com"),
    id:6578962,
  };
   println!("姓名:{}\n邮箱:{}\nid:{}",user.username,user.email,user.id);
 ```
2.结构体快速更新语法
   + user2 仅仅在 email 上与 user1 不同，因此我们只需要对 email 进行赋值，剩下的通过结构体更新语法 ..user1 即可完成。
   + .. 语法表明凡是我们没有显示声明的字段，全部从 user1 中自动获取。需要注意的是 ..user1 必须在结构体的尾部使用。
```rust
 let mut user=User{
    active:false,
    username:String::from("大宝贝"),
    email:String::from("3167385363@qq.com"),
    id:6578962,
  };
let user2=User{
  username:String::from("不是宝贝的宝贝"),
  ..user
 };
 //结构体更新语法跟赋值语句 = 非常相像，在上面代码中，user1 的部分字段所有权被转移到 user2 中：
 //username 字段发生了所有权转移，作为结果，user1 无法再被使用。
 println!("{}", user1.active);
// 下面这行会报错
println!("{:?}", user1);
 ```
3. 构建函数 它接收两个字符串参数： email 和 username，然后使用它们来创建一个 User 结构体，并且返回。
```rust
fn create_user(username:String,email:String)->User{
   User{
    username,
    active:false,
    email,
    id:3,
   }
}
let user3=create_user(String::from("第三个宝贝"), String::from("3167385363!"));
println!("姓名:{}\n邮箱:{}\nid:{}",user3.username,user3.email,user3.id);
```
4. 元组结构体
  + 字段结构体
  + 圆括号
  + 结构体必须要有名称，但是结构体的字段可以没有名称，这种结构体长得很像元组，因此被称为元组结构体
```rust
struct Color(i32, i32, i32);
 let black = Color(0, 0, 0);
 println!()
```
 5. 单元结构体
  + 定义一个类型，但是不关心该类型的内容, 只关心它的行为时，就可以使用 单元结构体 
```rust
struct A{
  
}
 ```
 6. 打印结构体
```rust
#[derive(Debug)]
struct Color(i32, i32, i32);
  println!("{:?}",black);
```
### 方法 ###
1. 在方法中，我们使用 &self 替代this 
2. self 依然有所有权的概念：
   + self 表示 Rectangle 的所有权转移到该方法中，这种形式用的较少
   + &self 表示该方法对 Rectangle 的不可变借用
   + &mut self 表示可变借用
```rust
struct User {
  active: bool,
  username: String,
  email: String,
  id: i32,
}
impl User{
  fn get_name(&self){
    println!("{:?}",&self.username);
  }
  fn get_id(&self){
    println!("{:?}",&self.id);
  }
}
user2.get_name();
user2.get_id();
```
3. 构造函数
```rust
impl User{
  fn new(username:String,email:String)->User{
    User{
      username,
      active:false,
      email,
      id:3,
     }
  }
}
 let user4=User::new("user4".to_string(),"1245@qq".to_string());
 ```
### 枚举类型与模式匹配 ###
1. c语言方式声明
```rust
#[derive(Debug)]
enum IpAddKind {
  Ipv4,
  Ipv6
}
#[derive(Debug)]
struct IpAdd{
  kind:IpAddKind,
  address:String
}
 let ip=IpAdd{
      kind:IpAddKind::Ipv4,
      address:"127.0.0.1".to_string()
    };
   print!("{:?}",ip)
```
2. rust方式
```rust
enum IpAddKind {
  Ipv4(String),
  Ipv6(String)
}
let ip=IpAddKind::Ipv4(String::from("127.0.0.1"))
```
3. 可以是不同类型
```rust
enum IpAdd{
    v4(i8,i8,i8),
    v6(String)
}
let ip=IpAddKind::Ipv4(127,0,0,1)
```
4. 不同类型
```rust
enum Message {
    Quit,//默认类型
    Move { x: i32, y: i32 },//结构体类型
    Write(String),//元组类型
    ChangeColor(i32, i32, i32),//元组类型
}
fn main() {
    let m1 = Message::Quit;
    let m2 = Message::Move{x:1,y:1};
    let m3 = Message::ChangeColor(255,255,0);
}
```
5. 枚举方法与match匹配
```rust
enum Message {
    Quit,//默认类型
    Move { x: i32, y: i32 },//结构体类型
    Write(String),//元组类型
    ChangeColor(i32, i32, i32),//元组类型
}
impl Message {
  fn print(){
  match *self{
    Message::Quit=>println("Quit"),
    Message::Move(x,y)=>println("Move x={} y={}",x,y),
    Message::ChangeColor(a,b,c)=>println("ChangeColor a={},b={},c={}",a,b,c),
     _::Write(&s)=>print("Write={}",)
  }
  }
}
 ```
6. Option泛型表示空值
   + 为了使用 Option<T> 值，需要使用mathc编写处理每个成员的代码。你想要一些代码只当拥有 Some(T) 值时运行，允许这些代码使用其中的 T
  ```rust
   enum Option<T>{
    Some(T),
    None,
   }
    let some_number = Some(5);
    let absent_number: Option<i32> =Option::None;
      match some_number {
        Some(i) => temp = i,
        None => print!("do nothing"),
    }
  ```
7. 模式匹配
   + match 的匹配必须要穷举出所有可能，因此这里用 _ 来代表未列出的所有可能性
   + match 的每一个分支都必须是一个表达式，且所有分支的表达式最终返回值的类型必须相同
```rust
match target {
    模式1 => 表达式1,
    模式2 => {
        语句1;
        语句2;
        表达式2
    },
    _ => 表达式3
}
```
   + if let
   + 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match
```rust
    let _v2 = vec![1, 2, 3];
   if let Some(i) = &_v2.get(0){
    println!("{:?}",i)
  }
}
```
 ##  vector与String和HashMap ##
  ### vector ###
 1. vector是动态数组
```rust
 let mut v:Vec<i32>=Vec::new();
 //vector宏
 let v2 = vec![1, 2, 3];
```
2. 读取方法-下标读取
```rust
 let v2 = vec![1, 2, 3];
 print!("{}",&v2[0])
 ```
3.读取方法-get读取
  + 下标读取到的是Some(T) 若出现数组越界 则为None
```rust
 let v2 = vec![1, 2, 3];
  if let Some(i) = &_v2.get(0){
    println!("{:?}",i)
  }
  if let None = &_v2.get(4){
    println!("出错啦")
  }
 ```