struct User {
  active: bool,
  username: String,
  email: String,
  id: i32,
}
fn create_user(username:String,email:String)->User{
   User{
    username,
    active:false,
    email,
    id:3,
   }
}
impl User{
  fn get_name(&self){
    println!("{:?}",&self.username);
  }
  fn get_id(&self){
    println!("{:?}",&self.id);
  }
}
#[derive(Debug)]
struct Color(i32, i32, i32);
fn main(){
  let mut user=User{
    active:false,
    username:String::from("大宝贝"),
    email:String::from("3167385363@qq.com"),
    id:6578962,
  };
 if user.active {
  user.username=String::from("小宝贝");
 }
 let user2=User{
  username:String::from("不是宝贝的宝贝"),
  ..user
 };
 let black = Color(0, 0, 0);
 let user3=create_user(String::from("第三个宝贝"), String::from("3167385363!"));
  println!("姓名:{}\n邮箱:{}\nid:{}",user.username,user2.email,user.id);
  println!("姓名:{}\n邮箱:{}\nid:{}",user3.username,user3.email,user3.id);
  user2.get_name();
  println!("{:?}",black);
}
