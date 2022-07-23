trait ShoolName {
    fn get_school_name(&self)->String;
    
}
pub struct Post {
  pub title: String, // 标题
  pub author: String, // 作者
  pub content: String, // 内容
}
#[derive(Debug)]
pub struct Weibo {
  pub username: String,
  pub content: String
}

impl ShoolName for Weibo {
  fn get_school_name(&self) -> String {
      format!("{}发表了微博{}", self.username, self.content)
  }
}
impl ShoolName for Post {
  fn get_school_name(&self) -> String {
      format!("文章{}, 作者是{}", self.title, self.author)
  }
}
fn return_school()->impl ShoolName{
  return Weibo{
    username:String::from("高"),
    content:String::from("")
  }
}
fn main(){
  let p=Post{
    title:String::from("标题"),
    author:String::from("张思"),
    content:String::from("真好")
  };
//  let t=return_school() as String;
// if let Weibo(t)=t{
//  println!("{}",username)
// }
  println!("{}",p.get_school_name());
}