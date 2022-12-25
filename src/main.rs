
fn main(){
 let a=0;
    let mut arr:Vec<i32>=Vec::with_capacity(10);
    //假定想要获取第178个bit的状态
    let numIndex=178/32;
    let bitIndex=178%32;
    //取得i位的状态
    let s=(arr[numIndex]>>bitIndex)&1;
   //修改状态为1
    arr[numIndex]=arr[numIndex]|(1<<bitIndex);
    //修改状态为0
    arr[numIndex]=arr[numIndex]&(!(1<<bitIndex));
}