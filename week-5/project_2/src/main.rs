use std::io;

fn main() {
    let mut input1 = String::new();

    println!("Enter age:");
    io::stdin().read_line(&mut input1).expect("Invalid string");
    let age: f32 = input1.trim().parse().expect("Invalid number");

    if age >= 40.0 {
        println!("\nThe incentive of the experienced employee is 1_560_000");
    
    } else if age >= 30.0 && age < 40.0 {
        println!("\nThe incentive of the experienced employee is 1_300_000 ");
    
    } else if age < 28.0 {
        println!("\nThe incentive of the inexperienced employee is 1_300_000");
    
    } else {
        println!("You're inexperienced and your incentive is 100_000");
    }
}







