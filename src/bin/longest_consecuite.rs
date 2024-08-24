use std::collections::HashSet;

fn longest_consecutive(nums: Vec<i32>) -> usize {
    
    // - insert all the elements into a HashSet to allow
    // for fast lookups
    let num_set: HashSet<i32> = nums.into_iter().collect();

    // - use the fold method to find the longest consecutive
    // sequence.
    num_set.iter().fold(0, |max_len, &num| {
        // check if 'num' is the start of a sequence
        if !num_set.contains(&(num - 1)) {
           
            // count the length of the sequence starting at 'num'
            let mut current_len = 1;
            let mut current_num = num + 1;
            while num_set.contains(&current_num) {
                current_len += 1; 
                current_num += 1; 
            }

            // - update 'max_len' if the current sequence is the 
            // longest found so far
            max_len.max(current_len)
        } else {
            // if 'num' is not the start of a sequence, keep 'max_len' as is
            max_len
        }

    })
}

fn main() {
    let nums: Vec<i32> = vec![100, 4, 200, 1, 3, 2,];

    let result = longest_consecutive(nums);

    println!("the result is {:?}", result);

}
