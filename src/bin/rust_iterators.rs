pub fn test_rust_iterators() {
    let fruit_list = vec!["strawberry", "cherry", "orange", "mango"];

    let nut_list = vec!["almonds", "peanut", "macadamia", "pecans"];
    
    let mut fruit_iter = fruit_list.iter();

    let item = fruit_iter.next();

    println!("the first item is {}", item.unwrap());
    
    // Testing using the chain() method
    let aggregate_foods =  fruit_list.iter().chain(&nut_list);

    let all_foods: Vec<&&str> = aggregate_foods.clone().collect();

    for food in aggregate_foods {
        println!("eating {}", food);
        println!("eating {:?}", all_foods);
    }

    let fruit_list_strings = fruit_list.iter().map(|e| String::from(*e));


    fruit_list_strings.peekable().next();

    let food = fruit_list_strings.peekable().peek();
    

}

fn main() {
    test_rust_iterators();
}

