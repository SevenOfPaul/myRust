#[macro_export]
macro_rules! trino{
  ($a:expr,$b:expr,$c:expr)=>{
    {
    if $a{
       $b
    }else{
       $c
    }
      }
  };
}