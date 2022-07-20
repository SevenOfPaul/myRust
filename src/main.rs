use std::fs::File;
use std::io::{prelude::*, Error};
fn read_file(path:String)->Result<String,Error>{
    let mut file = File::open(path)?;
    let mut s=String::new();
     file.read_to_string(&mut s)?;
     Ok(s)
}
fn main(){
    // let mut file = File::open("../test.txt");
    // let mut content = String::new();
    // match file{
    //     Ok(mut file) => file.read_to_string(&mut content).unwrap(),
    //     Err(_) =>panic!("出错了")
    // };
    let mut content = read_file("../test.txt".to_string());
    match &content {
     Ok(content)=>  println!("{:?}",content),
     Err(e) => panic!("{}", e)
    }
}