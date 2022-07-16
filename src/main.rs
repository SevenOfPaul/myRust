#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    id: i32,
}
fn create_user(username: String, email: String) -> User {
    User {
        username,
        active: false,
        email,
        id: 3,
    }
}
impl User {
    fn get_name(&self) {
        println!("{:?}", &self.username);
    }
    fn get_id(&self) {
        println!("{:?}", &self.id);
    }
    fn new(username: String, email: String) -> User {
        User {
            username,
            active: false,
            email,
            id: 3,
        }
    }
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
enum IpAddKind {
    Ipv4,
    Ipv6,
}
#[derive(Debug)]
struct IpAdd {
    kind: IpAddKind,
    address: String,
}
enum Message {
    Quit,                       //默认类型
    Move { x: i32, y: i32 },    //结构体类型
    Write(String),              //元组类型
    ChangeColor(i32, i32, i32), //元组类型
}
impl Message {
    fn print(&self) {
        match *self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move x={} y={}", x, y),
            Message::ChangeColor(a, b, c) => println!("ChangeColor a={},b={},c={}", a, b, c),
            _ => print!("Write"),
        }
    }
}
// enum Option<T> {
//     Some(T),
//     None,
// }
fn puls_one(x:Option<i32>)->i32{
    let mut temp=0;
  match x {
       Some(i) => temp = i,
      _=>print!("do some")
  }
   temp
}

fn main() {
    let mut user = User {
        active: false,
        username: String::from("大宝贝"),
        email: String::from("3167385363@qq.com"),
        id: 6578962,
    };
    if user.active {
        user.username = String::from("小宝贝");
    }
    let user2 = User {
        username: String::from("不是宝贝的宝贝"),
        ..user
    };
    let black = Color(0, 0, 0);
    let user3 = create_user(String::from("第三个宝贝"), String::from("3167385363!"));
    let user4 = User::new("user4".to_string(), "1245@qq".to_string());
    println!(
        "姓名:{}\n邮箱:{}\nid:{}",
        user.username, user2.email, user.id
    );
    println!(
        "姓名:{}\n邮箱:{}\nid:{}",
        user3.username, user3.email, user3.id
    );
    user2.get_name();
    let ip = IpAdd {
        kind: IpAddKind::Ipv4,
        address: "127.0.0.1".to_string(),
    };
    println!("{:?}", black);
    println!("{:?}", user4);
    print!("{:?}", ip);
    let some_number = Some(5);
    let some_string = Some(String::from("你好"));
    let absent_number: Option<i32> =Option::None;
    let number: i32 = 10;
    let mut temp = 0;
    //对some_number进行模式匹配
    match some_number {
        Some(i) => temp = i,
        None => print!("do nothing"),
    }
    println!("{}",puls_one(some_number));
    println!("{:?}",number+temp)
}
