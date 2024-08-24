use std::collections::HashMap;

fn first_unique_char(s: String) -> Option<usize> {
    // check if empty first
    if s.is_empty() {
        return None;
    } 

    let char_count = s.chars().fold(HashMap::new(), |mut acc, c| {
        *acc.entry(c).or_insert(0) += 1; // increment the count for each character
        acc // return the updated accumulator (the HashMap)
    });

    // - use enumerate and find_map to find the first unique character. 
    // .find_map() searhes for the first character with a frequency
    // of 1.
    //
    s.chars().enumerate().find_map(|(i, c)| {
        if char_count[&c] == 1 {
            Some(i) // return the index if the character is unique
        } else {
            None // Otherwise, continue searching
        }


    })

}



fn main() {

    let input = String::from("leetcode");

    let result = first_unique_char(input);

    println!("the result is {:?}", result);

}
