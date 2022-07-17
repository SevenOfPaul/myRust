#[derive(Debug)]
pub struct Person{
    name:String,
    age:i8,
}
impl Person{
    pub fn new_person(name:String,age:i8)->Person{
        Person{
           name,
           age,
        }
    } 
}