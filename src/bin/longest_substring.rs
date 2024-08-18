use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        s.chars() 
            .enumerate() // give each an index
            .fold(

                // Initial state: (character map, start index, max length)
                (HashMap::new(), 0, 0),

                // Closure: (current state, current char) -> new state
                |(mut char_indices, mut start, max_len), (end, ch)| {
                   
                    // Update start index:
                    // 1. insert current char and its next index into map
                    // 2. If char exists, get its previous index, else get 0
                    // 3. Take max of current start and the value from step 2
                    start = start.max(char_indices.insert(ch, end + 1).unwrap_or(0));

                    // Return new state:
                    // 1. Updated char_indices map
                    // 2. New start index
                    // 3. Max of current max_len and length of current substring
                    (char_indices, start, max_len.max(end + 1 - start))
                },
                ) // end of fold()
            .2 as i32 // extract max_len (3rd element of tuple) and convert to i32
    }
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_longest_substring() {
        assert_eq!(Solution::length_of_longest_substring("abcabcbbb".to_string()), 3);
    }

}

fn main() {
   // test cases: (input string, expected string)
   let test_cases = vec![
       ("abcabcbb".to_string(), 3),
       ("bbbbb".to_string(), 1),
       ("pwwkew".to_string(), 3),
       ("".to_string(), 0),
   ];

   // iterate through test cases
   for (s, expected) in test_cases {
       // run the solution
       let result = Solution::length_of_longest_substring(s.clone());
       
       // print input, ouput, and expected result
       println!("Input: {}, Output: {}, Expected: {}", s, result, expected);

       // assert taht the output mathes the exected result
       assert_eq!(result, expected);
   }
}





