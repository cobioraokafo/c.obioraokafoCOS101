use std::io;

fn main() {
    // Create an empty vector "city"
    let mut city: Vec<String> = Vec::new();
    // Print City Vector
    println!("The City vector has elements {:?}", city);
    // Push new elements into the vector
    let mut input1 = String::new();
    println!("How many cities do you want to enter?");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num: i32 = input1.trim().parse().expect("Invalid Input");

    for count in 0..city_num {
        let mut input2 = String::new();
        print!("Enter City {}: ", count + 1);
        io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_city: String = input2.trim().parse().expect("Invalid input");
        city.push(new_city);
    }
    print!("Your preferred cities are:\n");
    let mut count = 1;
    // loop to iterate elements in the vector
    for i in &city {
        // iterating through i on the vector
        println!("City {}: {}", count, i);
        count += 1;
    }
}