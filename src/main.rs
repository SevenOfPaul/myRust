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
#[derive(Debug)]
struct NumberArray<'a> {
    count: &'a [i32],
    index:i32
}
impl NumberArray<'_>{
 fn new(arr:[i32;5])->NumberArray<'static>{
   let mut result=&arr[0..arr.len()-1];
   NumberArray{count:result,index:0}
 }
}
<<<<<<< HEAD
<<<<<<< Updated upstream
impl Iterator for NumberArray<'_>{
    type Item = i32;
    fn next(&mut self)->Option<Self::Item>{
=======
impl<'a> Iterator for NumberArray<'_>{
    type Item = &'a[i32];
    fn next(&mut self)->Option<Self::Item >{
>>>>>>> 2022-8-01
        self.index += 1;
        let index=self.index;
         if self.index<=self.count.len() as i32 {
          return Some(self.count.clone())
         }else{
            return None;
         }
    }
}
=======
// impl<'a> Iterator for NumberArray<'_>{
//     type Item = &'a[i32];
//     fn next(&mut self)->Option<Self::Item >{
//         self.index += 1;
//         let index=self.index;
//          if self.index<=self.count.len() as i32 {
//           return Some(self.count.clone())
//          }else{
//             return None;
//          }
//     }
// }
>>>>>>> Stashed changes
fn main() {
    ///声明动态数组
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
    let na=NumberArray::new([1,2,3,4,5].to_owned());
    println!("数组{:?}",&na);
<<<<<<< Updated upstream
    for n in na{
        println!("数组{:?}",n);
    }
<<<<<<< HEAD
}
=======
    // for n in na{
    //     println!("数组{:?}",n);
    // }
}
>>>>>>> Stashed changes
=======
}
>>>>>>> 2022-8-01
