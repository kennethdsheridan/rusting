// Using the .find() functional pattern to break out of a 
// loop early like a traditional for loop would. 

fn main() {
    let numbers = vec![1,2,3,4,5,6,7];

    if let Some(&first_even) = numbers.iter().find(|&&x| x % 2 == 0) {
        println!("First even number is: {}", first_even);
    }
}
