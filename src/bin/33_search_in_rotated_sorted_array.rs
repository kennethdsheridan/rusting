struct Solution {}

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {

        let pivot = nums.iter()
           .enumerate() // index the values 
           .min_by_key(|&(_,&x)| x) // find the smallest element
           .map(|(i, _)| i) // extract just the index
           .unwrap_or(0); // if the array is empty, use 0 as a pivot 
        
        match nums[pivot..].binary_search(&target) {
            Ok(index) => (pivot + index) as i32,
            Err(_) => match nums[..pivot].binary_search(&target) {
                Ok(index) => index as i32,
                Err(_) => -1,
            },
        };

        -1
    }
}

fn main() {
    let nums = vec![4,5,6,7,8,9];

    let target = 6;

    let result = Solution::search(nums, target);

    println!("Index of the target of {target} is {}", result);
}
