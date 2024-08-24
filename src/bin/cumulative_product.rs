
fn cumulative_product_fold(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().fold((Vec::with_capacity(nums.len()), 1), |(mut result, product), num| {
        let new_product = product * num; // Calculate the new product
        result.push(new_product); // Store the new product in the result vector
        (result, new_product) // Return the updated result vector and product for the next iteration
    }).0 // Extract and return the result vector from the fold's final tuple
}


fn main() {

    let input = vec![1,2,3,4];

    let result = cumulative_product_fold(input);

    println!("result is {:?}", result);

    
}
