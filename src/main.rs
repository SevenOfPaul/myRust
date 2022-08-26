use std::io::BufReader;
use std::time;
use std::{fs::File, io::{Write, Read}};
use std::fs::OpenOptions;
fn test()->!{
    loop{}
}
fn main() {
    let now=time::Instant::now();
    let mut file=File::open("C:/Users/Paul/OneDrive/桌面/nest/rust/src/test.js").expect("no error");
     let mut s2=String::new();
     file.read_to_string(& mut s2);
    // let mut buffer=BufReader::new(&file);
    // buffer.read_to_string( &mut s2);
     println!("{}",&s2);
      println!("{:?}",now.elapsed());
    // let s="now";
    // let mut file = OpenOptions::new()
    // .read(true)
    // .append(true)
    // .create(true)
    // .open("./test.js")
    // .unwrap();
  
    // file.write_all(s.as_bytes());
    
    // println!("{:?}",s2)
}