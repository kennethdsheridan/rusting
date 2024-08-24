struct Solution {}

impl Solution {
    pub fn double_values(nums: Vec<i32>) -> Vec<i32> {

        // create an iter and map over the values 
        // doubling each element before printing
        let doubled: Vec<i32> = nums.iter()
            .map(|&x| x * 2 )
            .collect();

        // print the values
        doubled.iter()
            .for_each(|x| println!("the new value is {:?}", x));
        
        // return the doubled vector
        doubled

    }
}

fn main() {
    let nums = vec![1,2,5,6,3,12,45];

   let run = Solution::double_values(nums);

   println!("the doubled values are {:?}", run);

}
