// Given an integer array 'nums', return an array 'answer' such that 'answer[1]'
// is equal to the product of all the elements of 'nums' except 'nums[1]
//
// Answer must be given in O(n) time and does not use division

fn product_except_self(nums: Vec<i32>) -> Vec<i32> {

    let n = nums.len();
    
    // - initialize the result vector with 1's. This vector will
    // store the final result
    let mut result = vec![1; n];

    // - calculate the prefix products using 'enumerat() and .fold(). 
    // enumerate() gives us both the index 'i' and the element 'num'.
    // 
    // 'fold() iterates through the array, maintaining a running product 'acc'.
    // For each index 'i' we set 'result[1] to the current 'acc' (product of 
    // elements to the left of 'i').
    //
    // Then, update 'acc' by multiplying it with the current number 'num'
    nums.iter().enumerate().fold(1, |acc,(i, &num)| {
        result[i] = acc; // store the current prefix proudct in result[i]
        acc * num        // update eteh accumulator for the next iteration
    });

    // - calculate the suffix products in reverse and combine with 
    // the prefix products in-place 'rev()' reverses the iterator,
    // allowing us to go from right to left.
    // Again, we use fold to maintain a running product 'acc'. 
    //
    // For each index 'i', we multiply 'result[i] by the current 
    // 'acc' (which represents the suffix product).
    //
    // Then we update 'acc' by multiplying it with the current 'num'.
    nums.iter().enumerate().rev().fold(1, |acc, (i, &num)| {
        result[i] *= acc; // multiply the existing prefix product in 'result[i]'
        acc * num         // update the accumulator for the next iteration
    });


    result
}


fn main() {
    
    let input = vec![1,2,3,4];

    let result = product_except_self(input);
    println!("{:?}", result);
}
