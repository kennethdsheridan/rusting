use std::collections::HashMap; 

fn fun_two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut map = HashMap::new();

    nums.iter().enumerate().find_map(|(i, &num)| {
        // calculate the complement which is the value that when 
        // added to the current number, would equal the target.
        let complement = target - num;
        
        // 'get(&complement) checks if the compelement exists if the HashMap.
        // if the com;lement is found, it returns 'Some(&j) where 'j' is the 
        // index of the complement.
        //
        // 'map(|&j| (j, i)) applies a closure to the rsult of 'get'.
        //
        // if 'get' found the com;plemtn, map transforms the rsult in 'Some((j, i))
        // where j is the index of the complement and i is the index of the current number.
        map.get(&complement).map(|&j| (j, i)).or_else(|| {
            map.insert(num, i);
            None
        })
    })
}

fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
   let mut map = HashMap::new();
    
   nums.iter()
       .enumerate()
       .find_map(|(i, &num)| {
           let complement = target - num;

           if let Some(&j) = map.get(&complement) {
               Some((j, i))
           } else {
               map.insert(num, i);
               None
           }
       })
   
}

fn main () {

    let nums = vec![2,7,11,15];

    let target = 9;

    if let Some((i, j)) = two_sum(nums, target) {
        println!("Indices: {}, {}", i, j);
    } else {
        println!("No solution found");
    }

}


