use std::{sync::{Mutex, Arc}, thread, rc::Rc};


fn main() {
 let mut m=Arc::new(Mutex::new(5));
 let mut handles=Vec::new();
 for n in 0..10{
    let mut counter=Arc::clone(&m);
  let handle=thread::spawn(move ||{
        let mut num=counter.lock().unwrap();
         *num+=1;
    });
    handles.push(handle);
 }
 for handle in handles{
    handle.join().unwrap();
 }

 println!("{:?}",m);
}