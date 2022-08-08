# Cargo #
1. 创建项目 `cargo new +name`
2. 检查代码 `cargo check`
3. 代码编译 `cargo build`
4. 代码编译执行` cargo run`
5. 
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
4.可变引用遍历
```rust
 for v in &mut _v2{
    *v+=1;
    println!("{:?}",v)
  }
 ```
5. 使用枚举存储不同类型的数据
```rust
enum conText{
    Text(String),
    Float(f32),
    Int(i8)
}
  let _v3:Vec<conText>=vec![conText::Text("你好".to_string()),
    conText::Int(1),
    conText::Text("很灵活".to_string()),
    conText::Float(0.7)
    ];
 ```
   ### String ###
1. 创建字符串
```rust
let mut s=String::new();
let mut s2=String::from("hello");
let mut s3="world".to_string();
 ```
2. 合并字符串
```rust
//追加字符串
s.push_str("rust")
//追加字符
s.push('q')
//第一个加数为值 其余为引用
let s3=s1+s&2
 ```
 3. 下标打印字符串
    + 必须转换为切片 在利用切片的特性
```rust
  let s4="hello".to_string()+&"rust".to_string()+&s3;
 let str=&s4[0..=2];
  println!("{}",s4);
```
   + chars方法 字符打印
```rust
   for c in s4.chars(){
    println!("c={}",c)
   }
```
  + bytes方法 字节打印
```rust
   for c in s4.bytes(){
    println!("c={}",c)
   }
```
 ### hashMap ###
1. Rust 中哈希类型（哈希映射）为 HashMap<K,V>
2. 声明 hashMap
 ```rust
 let mut scores:HashMap<String,i32>=HashMap::new();
 //插入
  scores.insert("blue".to_string(), 10);
  //取值
  scores.get("blue")
  //确认是否有blue 没有的话再插入
  scores.entry("blue".to_string()).or_insert(3)
  ```
3. 数组遍历声明
```rust
   let keys=vec![String::from("blue"),String::from("red")];
   let values=vec![10,20];
   let scores2:HashMap<_,_>=keys.iter().zip(values.iter()).collect();
   //into_iter 方法将列表转为迭代器，接着通过 collect 进行收集，不过需要注意的是，collect 方法在内部实际上支持生成多种类型的目标集合，
   //因为我们需要通过类型标注 HashMap<_,_> 来告诉编译器：请帮我们收集为 HashMap 集合类型，具体的 KV 类型，麻烦编译器您老人家帮我们推导。
 ```
4. 获取值
```rust
 if let Some(v)=scores2.get(&keys[0]){
    println!("{:?}",v);
   }
let item=String::from("blue")
scores.get(&item);
scores[&item]
 ```
## 包模块管理 ##
1. Package 项目/工程
2. Crate  包/模块
3. rust 模块默认的成员都是私有的
   + 声明包成员
```rust
mod factory{
  // pub(crate) 公开关键字
    pub(crate) mod produce_refrigerator{
        pub(crate) fn produce_re(){
            println!("冰箱");
        }
    }
    pub(crate) mod produce_washing_machine{
        pub(crate) fn produce_re(){
            println!("洗衣机");
        }
    }
}
 ```
### 自定义包 ###
1. `cargo new libName --lib`
2. 在lib下的src也可以创建其他rs文件
3. 但是必须在lib.rs下暴露
### 引入第三方包 ###
1. 在toml中写入需要的第三方包
```toml
rand="0.8"
rust-crypto="*"
```
2. 在编译时编译器会自动下载
### 小技巧 ### 
1.as设置别名 `use std::io::Result as IoResult;`
2.{}引入多个 `use std::{cmp::Ordering, io};`
3.* 全部引入 `use std::collections::*;`
4. self `use self::xxx，表示加载当前模块中的 xxx。此时 self 可省略`
   +  `use xxx::{self, yyy}，表示，加载当前路径下模块 xxx 本身，以及模块 xxx 下的 yyy`
### 限制可见性语法 ###
1. pub 意味着可见性无任何限制
2. pub(crate) 表示在当前包可见
3. pub(self) 在当前模块可见
4. pub(super) 在父模块可见
5. pub(in <path>) 表示在某个路径代表的模块中可见，其中 path 必须是父模块或者祖先模块
## 错误处理 ##
1. panic!("出错了") 相当于throw new Error("出错了")
### 可恢复错误 ###
1. 通常用于从系统全局角度来看可以接受的错误，例如处理用户的访问、操作等错误
2. 这些错误只会影响某个用户自身的操作进程，而不会对系统的全局稳定性产生影响
### 不可恢复错误 ###
1. 该错误通常是全局性或者系统性的错误，例如数组越界访问，系统启动时发生了影响启动流程的错误等等，
2. 这些错误的影响往往对于系统来说是致命的
### 手动处理 ###
1. 分类处理
```rust
 let mut file = File::open("../test.txt");
    let mut content = String::new();
    match file{
        Ok(mut file) => file.read_to_string(&mut content).unwrap(),
        Err(_) =>panic!("出错了")
    };
 ```
2. ?
```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
 ```
3.在不需要处理错误的场景
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
//或
fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
 ```
### 传播错误 ###
1.
```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
 ```
2. main函数中的?处理错误
```rust
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
 ```
 ## 泛型 ## 
### 函数中泛型 ###
```rust
fn largest<T:std::cmp::PartialOrd>(list:&[T])->&T{
    let mut largest = &list[0];
    for item in list.iter() {
        if &item > &largest {
           largest = &item;
        }
    }
    &largest

}
fn main(){
    let a_list=vec![1,2,4,7,8,7,11,4,1];
     println!("{:?}",largest(&a_list));
}
 ```
 ### 结构体中泛型 ###
```rust
  struct Person<T>{
    name:T,
    age:i32,
    addr:T
}
 let p=Person{
    name:String::from("张思"),
    age:12,
    addr:String::from("黑龙江")
   };
  println!("{:?}",p);
 ```
   ### 枚举中泛型 ###
```rust
enum People<T>{
  Name(T),
  Age(i32),
}
 let name=People::Name(("张思舞".to_string()));
   let age=People::Age::<i32>(7);
   if let People::Name(n)=name{
        print!("{}", n);
   }
   if let People::Age(n)=age{
    println!("{}", n);
}
 ```
  ### 方法中泛型 ###
  1. impl指定方法时必须加上泛型
```rust
#[derive(Debug)]
  struct Person<T,G>{
    name:T,
    age:G,
    addr:T
}
impl<T, G> Person<T,G>{

fn getName(&self)->&T{
    &(&self.name)
}
fn getAge(&self)->&G{
  &(&self.age)
}
}
impl<T, G> Person<T,G>{

fn getName(&self)->&T{
    &(&self.name)
}
fn getAge(&self)->&G{
  &(&self.age)
}
fn getNewPerson<H,L>(self,other:Person2<H,L>)->Person<T,L>{
    let person=Person{
      name:self.name,
      age:other.age,
      addr:self.addr
    };
    return person;
}
}
 ```
 2. 针对特定泛型实现方法
```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
 ```
## Trait 特征 ##
1. Trait类似其它语言的接口 可以定以共享的行为
2. Trait只能定义行为(方法)
3. trait_bound相当于extend
## 实现 ##
```rust
trait ShoolName {
    fn get_school_name(&self)->String;
    
}
pub struct Post {
  pub title: String, // 标题
  pub author: String, // 作者
  pub content: String, // 内容
}
impl ShoolName for Weibo {
  fn get_school_name(&self) -> String {
      format!("{}发表了微博{}", self.username, self.content)
  }
}
fn main(){
  let p=Post{
    title:String::from("标题"),
    author:String::from("张思"),
    content:String::from("真好")
  };
  println!("{}",p.get_school_name());
}
```
### Trait_bound ###
1. 相当于ts的extends
2. where写法
```rust
 fn printPerson<T>(person:&T)
 where T:GetName+GetAge
 {
    println!("{}",person.GetName());
    println!("{}",person.GetAge());
   }
```
3. 标准写法
```rust
 fn printPerson<T:GetName+GetAge>(person:&T){
    println!("{}",person.GetName());
    println!("{}",person.GetAge());
   }
``` 
### Trait作为返回值 ###
1. 不要用if来返回两个符合trait的类型 编译器无法通过
```rust
trait ShoolName {
    fn get_school_name(&self)->String;
    
}
#[derive(Debug)]
pub struct Post {
  pub title: String, // 标题
  pub author: String, // 作者
  pub content: String, // 内容
}
 let p=Post{
    title:String::from("标题"),
    author:String::from("张思"),
    content:String::from("真好")
  };
  fn return_school<T:ShoolName>(obj:&T)->&impl ShoolName{
  return obj.clone()
}
let _t=return_school(&p); 
 ```
 #### 有条件的实现Trait_bound ####
 1.只有符合Trait的才会添加方法
```rust
use std::fmt::Display;
struct Pair<T> {
    x: T,
    y: T,
}
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```
```rust
impl<T:GetName> PrintName for T{
 fn print_name(&self){
  println!("{}",self.GetName())
 }
}
```
2. 为不同特征重载
```rust
struct Person{
  master:T,
  student:U
}
impl<T:GetName+GetAge,U:getName> Person<T,U>{
  fn print(&self){
    println!("{}",self.master.get_name())
    println!("{}",self.student.get_name())
  }
}
```
## 生命周期 ##
1. 生命周期的主要作用是避免悬垂引用，它会导致程序引用了本不该引用的数据
```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
//let r; 的声明方式貌似存在使用 null 的风险，实际上，当我们不初始化它就使用时，编译器会给予报错
//r 引用了内部花括号中的 x 变量，但是 x 会在内部花括号 } 处被释放，因此回到外部花括号后，r 会引用一个无效的 x
//此处 r 就是一个悬垂指针，它引用了提前被释放的变量 x，可以预料到，这段代码会报错：
```
2. 为了保证 Rust 的所有权和借用的正确性，Rust 使用了一个借用检查器(Borrow checker)，来检查我们程序的借用正确性：
```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}      
                // ---------+
``` 
  + 这段代码和之前的一模一样，唯一的区别在于增加了对变量生命周期的注释。这里，r 变量被赋予了生命周期 'a，x 被赋予了生命周期 'b，从图示上可以明显看出生命周期 'b 比 'a 小很多。

  + 在编译期，Rust 会比较两个变量的生命周期，结果发现 r 明明拥有生命周期 'a，但是却引用了一个小得多的生命周期 'b，在这种情况下，编译器会认为我们的程序存在风险，因此拒绝运行。

  + 如果想要编译通过，也很简单，只要 'b 比 'a 大就好。总之，x 变量只要比 r 活得久，那么 r 就能随意引用 x 且不会存在危险：
```rust
{
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```
  + 根据之前的结论，我们重新实现了代码，现在 x 的生命周期 'b 大于 r 的生命周期 'a，因此 r 对 x 的引用是安全的。
### 静态生命周期 ###
1. 'static，拥有该生命周期的引用可以和整个程序活得一样久。
2. 字符字面值都具有static生命周期
```rust
#![allow(unused)]
fn main() {
let s: &'static str = "我没啥优点，就是活得久，嘿嘿";
}
```
### 函数生命周期 ###
1. 为了避免 x y 的生命周期小于返回的生命周期
2. 
```rust
fn longest <'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
//也就是 较短生命周期的变量被释放时 'a结束
```
## 生命周期消除 ##
+ 消除规则不是万能的，若编译器不能确定某件事是正确时，会直接判为不正确，那么你还是需要手动标注生命周期
+ 函数或者方法中，参数的生命周期被称为 输入生命周期，返回值的生命周期被称为 输出生命周期
1. 每一个引用参数都会获得独自的生命周期
   + 例如一个引用参数的函数就有一个生命周期标注: fn foo<'a>(x: &'a i32)，两个引用参数的有两个生命周期标注:fn foo<'a, 'b>(x: &'a i32, y: &'b i32), 依此类推。
2. 若只有一个输入生命周期(函数参数中只有一个引用类型)，那么该生命周期会被赋给所有的输出生命周期，也就是所有返回值的生命周期都等于该输入生命周期
   + 例如函数 fn foo(x: &i32) -> &i32，x 参数的生命周期会被自动赋给返回值 &i32，因此该函数等同于 fn foo<'a>(x: &'a i32) -> &'a i32
3. 若存在多个输入生命周期，且其中一个是 &self 或 &mut self，则 &self 的生命周期被赋给所有的输出生命周期
   + 拥有 &self 形式的参数，说明该函数是一个 方法，该规则让方法的使用便利度大幅提升。
## &'static 和 T: 'static ##
1. &'static 对于生命周期有着非常强的要求：一个引用必须要活得跟剩下的程序一样久，才能被标注为 &'static。
2. 但是，&'static 生命周期针对的仅仅是引用，而不是持有该引用的变量，对于变量来说，还是要遵循相应的作用域规则 :
## 函数式编程 ##
### 闭包 ###
1. 标准函数方式
```rust
fn add(x:u32)->u32{
   x+1
}
 ```
 2. 闭包写法
```rust
let use_closure=|n:i32|->i32{
    println!("this is closure");
    return n
};
let closure2=|n:i32|{n+1};
let closure3=|n|n+1;
let v=closure3(7);
 ```
1. 针对closure3闭包，如果你只进行了声明，但是没有使用，编译器会提示你为 x, y 添加类型标注，因为它缺乏必要的上下文
2. 也就是必须赋值 且只能附加一次类型
#### 捕获作用域中的值 ####
1. 闭包可以获取作用域内的值
```rust
let closure3=|n|n+1;
let v=closure3(7);
let as_x=|n:i32|->bool{
    n==v
};
//下面函数例子会报错
fn as_x(n:i32)->bool{
    n==v
}
```
2. 闭包的特征
   + 闭包捕获变量有三种途径，恰好对应函数参数的三种传入方式：转移所有权、可变借用、不可变借用，因此相应的 Fn 特征也有三种：
   + FnOnce
   + 该类型的闭包会拿走被捕获变量的所有权。Once 顾名思义，说明该闭包只能运行一次
 ```rust
 fn fn_once<F>(func:F)
where
    F: FnOnce(usize) -> bool+Copy,
{
    println!("{}？", func(3));
    println!("{}？", func(4));
}
fn_once(|z|z==h.len());
```
   + FnMut
   + 它以可变借用的方式捕获了环境中的值，因此可以修改该值：
```rust
fn fn_mut<F>(mut func2: F)
where
    F: FnMut(i32) -> i32,
{
    println!("闭包={}？", func2(10));
    println!("闭包={}？", func2(4));
}
fn_mut(|mut z|{
    let index=h.len() as i32;
     z=index;
     z
});
 ```
   + Fn
   + Fn 特征，它以不可变借用的方式捕获环境中的值
```rust
fn fn_no_mut<'a,F: Fn(&'a str)->&str>(func3: F)
{
    println!("闭包={}？", func3("你好"));
    println!("闭包={}？", func3("我不好"));
}
fn_no_mut(|mut z|{
   z="号";
   z
});
```
  ## 迭代器 ##
+ 迭代器允许我们迭代一个连续的集合，例如数组、动态数组 Vec、HashMap 等，
  ### 创建迭代器 ###
```rust
struct Counter {
    count: i32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
 for i in (0..6)  {
        if let Some(v)=counter.next(){
            println!("{:?}",v);
        }else{
            println!("结束了")
        }
    }
 ```
## 智能指针 ##
1. 智能指针虽然也号称指针，但是它是一个复杂的家伙：通过比引用更复杂的数据结构，包含比引用更多的信息，例如元数据，当前长度，最大可用长度等
2. 智能指针往往是基于结构体实现，它与我们自定义的结构体最大的区别在于它实现了 Deref 和 Drop 特征：
3. Deref 可以让智能指针像引用那样工作，这样你就可以写出同时支持智能指针和引用的代码，例如 *T
4. Drop 允许你指定智能指针超出作用域后自动执行的代码，例如做一些数据清除等收尾工作
### Box指针 ###
1. 特意的将数据分配在堆上
2. 数据较大时，又不想在转移所有权时进行数据拷贝
3. 类型的大小在编译期无法确定，但是我们又需要固定大小的类型时
4. 特征对象，用于说明对象实现了一个特征，而不是某个特定的类型
   ```rust
       let b = Box::new(12);
     println!("{:?}&",b);
   ```
5. b存储在栈上 5存储在堆上 b指向5
```rust
#[derive(Debug)]
enum List<T>{
    Cons(T,Box<List<T>>),
    Nil,
}
use crate::List::{Cons,Nil};
fn main() {
 let a=Cons(2,Box::new(Cons(1,Box::new(Nil))));
}

 ```
### 解引用 ###
1. 为 MyBox 实现 Deref 特征，以支持 * 解引用操作符
2. 在 Deref 特征中声明了关联类型 Target，在之前章节中介绍过，关联类型主要是为了提升代码可读性
3. deref 返回的是一个常规引用，可以被 * 进行解引用
### 解引用重载 ###
```rust
struct MyBox<T>{
    value:T
}
impl<T> MyBox<T>{
    fn new(value:T)->MyBox<T>{
        MyBox { value }
    }
}
//解引用重载
impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self)->&Self::Target{
        &&self.value
    }
}
```
### Drop ###
1. 析构函数
```rust
struct MyBox<T>{
    value:T
}
impl<T> MyBox<T>{
    fn new(value:T)->MyBox<T>{
        MyBox { value }
    }
}
impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self)->&Self::Target{
        &&self.value
    }
}
impl<T> Drop for MyBox<T>{
    fn drop(&mut self) {
        println!("对像销毁");
    }
}
```
### RC智能指针 ###
1. 在一份引用需要被多种数据所引用的时候
2. 当我们希望在堆上分配一个对象供程序的多个部分使用且无法确定哪个部分最后一个结束时，就可以使用 Rc 成为数据值的所有者，
```rust
#[derive(Debug)]
enum List<T>{
    Cons(T,Rc<List<T>>),
    Nil,
}
use crate::List::{Cons,Nil};
use std::rc::Rc;
fn main() {

 let a=Rc::new(Cons(2,Rc::new(Cons(1,Rc::new(Nil)))));
 let b=Cons(2,Rc::clone(&a));
 let c=Cons(2,Rc::clone(&a));
 println!("{:?}",Rc::strong_count(&a));
}

```
```rust
//创建rc智能指针
Rc::new()
//复制一份数据创建Rc
Rc::clone()
//计算rc引用次数
Rc::strong_count()
```
### RefCell与Cell  ###
1.RefCell与Cell可以用来修改不可变借用内部的值
```rust
let a=RefCell::new(5);
*a.borrow_mut()+=10;
println!("{:?}",&a);//15
```
2. Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy 的情况
3. Cell 只适用于 Copy 类型，用于提供值，而 RefCell 用于提供引用
4. Cell 不会 panic，而 RefCell 会
```rust
fn main() {
    let value=Rc::new(RefCell::new(5));
    let a=Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b=Cons(Rc::new(RefCell::new(5)),Rc::clone(&a));
    let c=Cons(Rc::new(RefCell::new(6)),Rc::clone(&a));
    *value.borrow_mut()+=10;
    println!("{:?}**{:?}",&value,&b);
}
```