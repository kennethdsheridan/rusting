use std::collections::HashMap;


fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    // - fld the strings into a HashMap, grouping by sorted characters
    let anagrams_map = strs.into_iter().fold(HashMap::new(), |mut map, s| {
        let mut sorted_chars: Vec<char> = s.chars().collect();

        sorted_chars.sort_unstable();
        let key = sorted_chars.into_iter().collect::<String>();

        // insert the string into the corresponding vector in the map
        map.entry(key).or_insert_with(Vec::new).push(s);
        map
    });

    anagrams_map.into_values().collect()
}

fn anagram(strs: Vec<String>) -> Vec<Vec<String>> {
    let anagram_map = strs.into_iter().fold(HashMap::new(), |mut map, s| {
        // convert the string 's' into a vector of characters
        let mut sorted_chars: Vec<char> = s.chars().collect();

        // sort the vector of characters into lexicographical order, 
        // ensuring that all anagrams will have the same sorted key.
        sorted_chars.sort_unstable();
        
        // convert the sorted vector of characters back into a string to 
        // use as the key
        let key = sorted_chars.into_iter().collect::<String>();

        // if the key exists, returns a mutable reference to the corresponding 
        // value (a vector of strings). 
        //
        // if the key does not exist, it inserts the key with a new empty vector
        map.entry(key).or_insert_with(Vec::new).push(s);
        
        // return the updated 'map' to be used as the acc in the next 
        // iteration
        map
    });
    anagram_map.into_values().collect()
}
fn main() {}
