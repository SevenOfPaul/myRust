struct Counter {
    count: i32,
}
impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    match v1_iter.next() {
        Some(v) => println!("{}", v),
        None => todo!(),
    }
    match v1_iter.next() {
        Some(v) => println!("{}", v),
        None => todo!(),
    }
    match v1_iter.next() {
        Some(v) => println!("{}", v),
        None => todo!(),
    }
    let total: i32 = v1.iter().sum();
    let v2: Vec<_> = v1.iter().map(|x| x).collect();
    let mut counter=Counter::new();
    for i in (0..6)  {
        if let Some(v)=counter.next(){
            println!("{:?}",v);
        }else{
            println!("结束了")
        }
    }
    println!("{:?}", total);
    println!("{:?}", v2);
    println!("hello world");
}
