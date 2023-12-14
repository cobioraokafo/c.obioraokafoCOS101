use std::io::Write;

fn main() {

	let announce = "Week 9 - Rust File Input & Outout\n";
	let dept = "Departtment of Computer Science";

	let mut file = std::fs::File::create("date.txt").expect("create failed");
	file.write_all("Welcome to Rust programming\n"
		.as_bytes()).expect("write failed");
	file.write_all(announce.as_bytes()).expect("write failed");
	file.write_all(dept.as_bytes()).expect("write failed");
	println!("\nData written to file");

}