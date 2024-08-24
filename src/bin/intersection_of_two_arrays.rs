use std::collections::HashSet;

fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {

    // Convert the Sets: place both arrays into sets to remove the duplicates allowing 
    // for easy intersection operations
    let set1: HashSet<_> = nums1.into_iter().collect();

    let set2: HashSet<_> = nums2.into_iter().collect();



    // Intersection: 
    

    // Convert the Vector: 
    set1.intersection(&set2).cloned().collect()

}

fn main() {}
