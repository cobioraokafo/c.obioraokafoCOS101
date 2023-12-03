use std::io;

fn main() {
    // Creating variables and getting values for input a, b and c.
    println!("Input value of a >>");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a: f64 = a.trim().parse().expect("Failed to input");

    println!("Input value of b >>");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b: f64 = b.trim().parse().expect("Failed to input");

    println!("Input the value of c >>");
    let mut c = String::new();
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c: f64 = c.trim().parse().expect("Failed to input");

    let disc = b.powf(2.0) - 4.0 * a * c;

    if disc > 0.0 {
        let root1 = (-b + disc.sqrt()) / (2.0 * a);
        let root2 = (-b - disc.sqrt()) / (2.0 * a);
        println!("There are two distinct roots: {}, {}", root1, root2);
    } else if disc == 0.0 {
        let root = -b / (2.0 * a);
        println!("There is one distinct root: {}", root);
    } else {
        println!("There are no real roots");
    }
}

