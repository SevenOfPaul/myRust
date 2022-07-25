use std::time::{ Instant};


fn main(){
 let sy_time=Instant::now();
 let mut arr:[i8;7]=[1,2,54,78,8,9,10];
 for _i in 0..arr.len()-1 {
    for j in 0..arr.len()-1{
       if arr.get(j)>arr.get(j+1){
        let temp=arr[j];
         arr[j]=arr[j+1];
         arr[j+1]=temp;
       }
    }
 }
 println!("{:?}",arr);
 println!("{:?}",sy_time.elapsed());
}