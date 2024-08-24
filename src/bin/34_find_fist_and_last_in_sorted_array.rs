struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target:i32) -> Vec<i32> {
        // check if the target exist in the array
        // using binary search.
        if let Ok(index) = nums.binary_search(&target) {
            // if the target is found, then find the range
            //
            // find the leftmost position of the target
            // partition_point returns the index of the first element
            // that doesn't satisfy the predicate.
            // here it finds the first element that not less than the target
            let left = nums.partition_point(|&i| i < target) as i32;

            // find the rightmost position of the target
            // we find the first element thats greater than the 
            // target and subtract 1, giving us the index of the 
            // last occurrence of the target
            let right = nums.partition_point(|&x| x <= target) as i32 -1;

            // Return the range as a vector
            vec![left, right]
        } else {
            vec![-1, -1]
        }
    }
}



fn main() {
    let nums = vec![1,2,3,4,5,6,7,8];
    let target = 4;

   let result = Solution::search_range(nums, target);

    println!("the result of the target {target} is {:?}", result);
}
