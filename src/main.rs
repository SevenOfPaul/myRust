fn print(n:i32){
   let mut i=31;
   while i>=0 {
      if n&(1<<i)==0{
         print!("{}",'0');
      }else{
         print!("{}",'1');
      }
      i-=1;
   }
   println!("")
}
fn add(sum:i32,carry:i32)->i32{
   if carry==0{
      return sum
   }
   return add(sum^carry,(sum&carry)<<1)
}
fn pow(mut a:i32,mut b:i32)->i32{
   let mut res=1;
   while b!=0 {
      if b&1!=0{
         res=res*a;
      }
      a*=a;
      b>>=1;
   }
   return res
}
    fn count_primes(n: i32) -> i32 {
        let mut primes = i32::MAX;
        let mut res = 0;
        primes&=!(1<<0);
        primes&=!(1<<1);
        for idx in 2..n{
         if primes&(1<<idx)!=0{
             let mut j=idx*idx;
             while j <n{
                 primes&=!(1<<j);
                 j+=idx;
             }
         }
        }
        for idx in 0..n {
            if  primes& (1 << idx) != 0 {
                res+=1;
            }
        }
        res
    }
fn main(){
println!("{}", Solution::count_primes(10));
}
