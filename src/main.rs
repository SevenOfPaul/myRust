 pub fn sort(nums:Vec<i32>)->Vec<i32>{
        if nums.len()<2{
            return nums
        }
        let mut left=Vec::new();
        left.extend_from_slice(&nums[0..nums.len()>>1]);

        let mut right=Vec::new();
        right.extend_from_slice(&nums[nums.len()>>1..nums.len()]);
        Solution::merge(Solution::sort(left),Solution::sort(right))
    }
    pub fn merge(mut left:Vec<i32>,mut right:Vec<i32>)->Vec<i32>{
        let mut res=vec![];
        while  left.len()!=0&&right.len()!=0 {
            if left.last().unwrap()>right.last().unwrap(){
                res.push(right.pop().unwrap());
            }else{
                res.push(left.pop().unwrap());
            }
        }
        res.extend_from_slice(&left[0..left.len()]);
        res.extend_from_slice(&right[0..left.len()]);
        res
    }
fn main(){

}
