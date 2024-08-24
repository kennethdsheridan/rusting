pub fn filter_even_numbers(numbers: &[i32]) -> Vec<i32> {
    
    let even_nums  = numbers.iter()
        .filter(|&x| x % 2 == 0)
        .cloned()
        .collect();

    println!("the even numbers are {:?}", even_nums);
    
    even_nums
}

fn main() {

    let input = 45;


}
