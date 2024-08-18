
fn main() {
    
    let v: Vec<_> = (1..5).collect();
    let u = v.iter().chain([6,7,8,9,10].iter().map(|i| i ));
    u.for_each(|i| println!("{i}"));


    let arr1 = [1,2,3,4,5];
    let arr2 = [78,45,34,56,67];

    let new_vec: Vec<_> = arr1.iter().chain(arr2.iter()).collect(); 
    

    new_vec.iter().for_each(|i| println!("{i}"));
}

