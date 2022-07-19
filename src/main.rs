use newlib::factory as F;
use newlib::A;
use rand::prelude::*;
//这个可以忽略
// extern crate crypto;
use crypto::digest::Digest;
use crypto::sha3::Sha3;
fn main() {
    let x: u8 = random();
    F::produce_refrigerator::produce_re();
    F::produce_washing_machine::produce_re();
    let p=A::Person::new_person(String::from("张三"),16);
    let mut hahser=Sha3::sha3_256();
    hahser.input_str("hello world!");
    let result=hahser.result_str();
    println!("{:?}\n{:?}",p,result);
   println!("main{}",x);
   }