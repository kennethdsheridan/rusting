


fn main() {
    let dyn_array = [0, 4, 8, 16, 2, 9, 3];  
    println!("Original array: {:?}", dyn_array);

    // Find index
    if let Some(index) = dyn_array.iter().position(|&x| x == 8) {
        println!("Index of 8: {:?}", index);
    }

    // Filtering
    let even_numbers: Vec<&i32> = dyn_array.iter().filter(|&x| x % 2 == 0).collect();
    println!("Even numbers: {:?}", even_numbers);

    // Mapping
    let squared: Vec<i32> = dyn_array.iter().map(|&x| x * x).collect();
    println!("Squared values {:?}", squared);
    

    // Sum and Product
    let sum: i32 = dyn_array.iter().sum(); 
    let product: i32 = dyn_array.iter().product();
    println!("The sum is {} and the product is {}", sum, product);

    // Min and Max
    // Sorting
    // Enumerate
 }

