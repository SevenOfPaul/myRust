struct Soultion{
    val:Vec<i32>
}
impl Soultion{
    pub fn kth_largest_number(nums: Vec<String>, k: i32) -> String {
        let temp=&mut nums.into_iter().map(|s|{s.parse::<i32>().unwrap()}).collect::<Vec<_>>();
        println!("{:?}",temp);
        // quickSort(arr)[(k as usize)-1].to_string()
        String::new()
    }
}
fn quickSort(arr: &mut Vec<i32>) ->Vec<i32>{

    if arr.len()<=1{
        return arr.clone()
    }
    let  left:&mut Vec<i32>=&mut Vec::new();
    let  right:&mut Vec<i32>=&mut Vec::new();
    let provide=vec![arr[0]];
    for i in 1..arr.len(){
        if arr[i]>=provide[0]{
            left.push(arr[i]);
        }else{
            right.push(arr[i]);
        }
    }
    return vec![quickSort(left),provide,quickSort(right)].concat()
}
fn main(){
let mut arr =vec!["1", "20", "300", "400","12542","50000"];
   println!("{:?}",arr.sort_by(|a,b|a.cmp(b)));
    println!("{:?}",arr);
}