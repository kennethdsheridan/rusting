use std::collections::HashSet;

fn find_pairs(nums: Vec<i32>, target: i32) -> Vec<(i32, i32)> {
    
    // create an iterator over the vector nums. Then=
    // apply the 'fold' method to iterate over the elements
    // while maintaining an accumulator. 
    //
    // The accumulator is a tuple consisting of: 
    //  - A 'HashSet' to track the numbes we've seen so far
    //  - A 'Vec<(i32, i32)> to store the resulting pairs 
    //  that sum up to the target
    nums.iter()
        .fold((HashSet::new(), Vec::new()), |(mut seen, mut result), &num| {
            // calculate the complement of the current number
            let complement = target - num;

            // check if the complement is already in the 'seen'
            // set. If it is, then we have fold the pair that 
            // sums to the target.
            if seen.contains(&complement) {
                result.push((complement, num));
            }

            seen.insert(num);


            (seen, result)
        })
    .1

    

    
}


fn main() {
    
    let input: Vec<i32> = vec![2,3,4,5,6,7,1,9];

    let target = 10;

    let result = find_pairs(input, target);

    println!("the result is {:?}", result);

}
