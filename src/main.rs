trait ShoolName {
    fn get_school_name(&self)->String;
    
}
#[derive(Debug)]
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
fn return_school<T:ShoolName>(obj:&T)->&impl ShoolName{
  return obj.clone()
}
trait  GetName {
    fn GetName(&self)->&String;
}
trait  GetAge {
  fn GetAge(&self)->&i32;
}
struct  Person{
  name:String,
  age:i32
}
impl GetName for Person{
  fn GetName(&self)->&String {
      return &self.name;
  }
}
impl GetAge for Person{
  fn GetAge(&self)->&i32 {
      return &self.age;
  }
}
impl Person{
  fn printPerson<T:GetName+GetAge>(person:&T){
    println!("{}",person.GetName());
    println!("{}",person.GetAge());
   }
}
fn main(){
  let p=Post{
    title:String::from("标题"),
    author:String::from("张思"),
    content:String::from("真好")
  };
  
 let _t=return_school(&p); 
let person=Person{name:"找i".to_string(),age:12};
    Person::printPerson(&person);
}