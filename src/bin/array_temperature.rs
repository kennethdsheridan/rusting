

fn main() {
    
    let temperatures = [ 72, 75, 68, 80, 73 ];

    match temperatures {
        [spring, summer, fall, winter, average] => {
            println!("Average temperature: {}", average);
        }

        _ => println!("Unexpected data format"),
    }

    println!("Temperatures are: {}", &temperatures);
}
