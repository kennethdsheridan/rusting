

fn main() {
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum = numbers.iter()
        .fold(0, |acc, &x| acc + x);

    println!("Sum: {:?}", sum);
}
