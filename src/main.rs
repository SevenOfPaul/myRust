trait ShoolName {
    fn get_school_name(&self)->String;
    
}
pub struct Post {
  pub title: String, // 标题
  pub author: String, // 作者
  pub content: String, // 内容
}
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
fn main(){
  let p=Post{
    title:String::from("标题"),
    author:String::from("张思"),
    content:String::from("真好")
  };
  println!("{}",p.get_school_name());
}