use std::io;

fn main() {
	let mut input_1 = String::new();

	 println!("Enter price:");
	 stdin().read_line(&mut input_1).expect("Invalid string");
	 let total order:f32 = input_1.trim().parse().expect("Invalid number");

	 // P = Poundo Yam/Edinakaiko
	 // F = Fried Rice & Chicken
	 // A = Amala & Ewedu
	 // E = Eba & Egusi
	 // W = White Rice & Stew

	 let P:f32 = 3200;
	 let F:f32 = 3000;
	 let A:f32 = 2500;
	 let E:f32 = 2000;
	 let W:f32 = 2500;


}
