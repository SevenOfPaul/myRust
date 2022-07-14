struct User {
  active: bool,
  username: String,
  email: String,
  sign_in_count: i32,
}

fn main(){
  let mut user=User{
    active:false,
    username:String::from("大宝贝"),
    email:String::from("3167385363@qq.com"),
    sign_in_count:6578962,
  };
 if user.active {
  user.username=String::from("小宝贝");
 }
  println!("姓名:{}\n邮箱:{}\nid:{}",user.username,user.email,user.sign_in_count);
}
