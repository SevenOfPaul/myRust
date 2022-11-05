use std::collections::HashMap;

pub fn single_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut m:HashMap<i32,i32>=HashMap::new();
for v in nums{
    if None!=m.get(&v){
        m.remove(&v);
    }else{
        m.insert(v,1);
    }

    }
    let mut result:Vec<i32>=vec![];
    for i in  m.keys(){
        result.push(*i);
    }
    result
}

fn main() {
println!("{:?}", single_numbers(vec![1,2,10,4,1,4,3,3]));
}
