use std::{thread,sync::mpsc,time::{Duration, Instant}, rc::Rc, collections::VecDeque};

fn insert(arr:&mut Vec<i32>)->&Vec<i32>{
    for a in 0..arr.len(){
        let mut min:usize=a;
     for b in a+1..arr.len(){
        if arr[b] < arr[min]{
         min=b; 
        }
     }
     {
        let temp=arr[min];
        arr[min]=arr[a];
        arr[a]=temp;
     }
    }
    return arr;
}
fn bubble(arr:&mut Vec<i32>)->&Vec<i32>{
    for a in 0..arr.len(){
        for b in a..arr.len(){
        if arr[a]>arr[b]{
           arr.swap(a, b);
        }
    }
    }
    return arr;
    }
fn main() {
    let (tx,rx)=mpsc::channel();
    let tx3=mpsc::Sender::clone(&tx);
    let mut v=vec![10,2,17,4,20,7,12,32,5,70,54];
    let handle=thread::spawn(move ||{
        let start = Instant::now();
        let v2=bubble( &mut v);
        tx.send(v2.clone()).unwrap();
        tx3.clone().send(vec![2]);
        println!("bubble!{:?}\n时间:{:?}",v2,start.elapsed());
    });
    let start = Instant::now();
    let mut v4=vec![10,2,17,4,20,7,12,32,5,70,54];
    let v3=insert(&mut v4);
    let v5=rx.recv();
   for rec in rx{
    println!("{:?}",rec);
   }
    println!("insert!{:?}\n时间:{:?}",v3,start.elapsed());
    handle.join();
}