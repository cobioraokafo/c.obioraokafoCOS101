use std::io;

fn main() {
	struct Company name{
		founded:u32;
		username:String;
		password:String;
		company shares:u32;
		company liabilities:u32;
	}

	impl commany {
		fn leverages used(&self) -> f32
	}

	let cadbury nigeria = Company name{
		founded:1965;
		username:String::from("cadb");
		password:String::from("ckay1");
		company shares:15000000.0;
		company liabilities:5500000.0;
	}

	let champion breweries = Company name{
		founded:1974;
		username:String::from("cham");
		password:String::from("ckay2");
		company shares:25000000.0;
		company liabilities:8000000.0;
	}

	let dangote sugar refinery = Company name{
		founded:1970;
		username:String::from("dang");
		password:String::from("ckay3");
		company shares:18000000.0;
		company liabilities: 10000000.0;
	}
	let flour mills nigeria = Company name {
		founded:1960;
		username:String::from("flou");
		password:String::from("ckay4");
		company shares: 32000000.0;
		company liabilities: 4000000.0;
	}

	let nestle nigeria = Company name {
		founded:1961;
		username:String::from("nest");
		password:String::from("ckay5");
		company shares: 8000000.0;
		company liabilities: 15000000.0;
	}

	let unilever nigeria = Company name {
		founded:1923;
		username:String::from("unil");
		password:String::from("ckay6");
		company shares: 37000000.0;
		company liabilities: 11000000.0;
	}

	let honeywell nigeria = Company name {
		founded:1906;
		username:String::from("hone");
		password:String::from("ckay7");
		company shares: 34000000.0;
		company liabilities: 9000000.0;
	}

	let nigerian breweries = Company name{
		founded:1946;
		username:String::from("nige");
		password:String::from("ckay8");
		company shares: 30000000.0;
		company liabilities: 12000000.0;
	}
	let mut file = std::fs::File::create("Companies_information.txt").expect("Failed to create file");
    
    // Verification username and password

    if cadbury nigeria: username = "cadb" && password = "ckay1"
    println!("\nCompany name:{}
    	        founded:{}
    	        company shares:{}
    	        company liabilities:{}");
                
    if champion breweries: username = "cham" && password = "ckay2"
    println!("\nCompany name:{}
    	        founded:{}
    	        company shares:{}
    	        company liabilities:{}");

    if dangote sugar refinery: username = "dang" && password = "ckay3"
    println!("\nCompany name:{}
    	        founded:{}
    	        company shares:{}
    	        company liabilities:{}");

    if flour milla nigeria: username = "flou" && password = "ckay4"
    println!("\nCompany name:{}
    	        founded:{}
    	        company shares:{}
    	        company liabilities:{}");

    if nestle nigeria: username = "nest" && password = "ckay5"
    println!("\nCompany name:{}
    	        founded:{}
    	        company shares:{}
    	        company liabilities:{}");
    
    if unilever nigeria: username = "unil" && password = "ckay6"
    println!("\nCompany name:{}
    	        founded:{}
    	        company shares:{}
    	        company liabilities:{}");

    if honeywell nigeria: username = "hone" && password = "ckay7"
    println!("\nCompany name:{}
    	        founded:{}
    	        company shares:{}
    	        company liabilities:{}");

    if nigerian breweries: username = "cadb" && password = "ckay1"
    println!("\nCompany name:{}
    	        founded:{}
    	        company shares:{}
    	        company liabilities:{}");

    // To calculate leverages
    fn calculate_5_percent(company: &mut Company) {
    for leverage in &mut company.percentage_leverages {
        *leverage = (*leverage * 5) / 100;
    }

    fn input_leverages(company: &mut Company) {
    println!("Enter the percentage leverages used by companies:");
    let number_of_leverages: u32 = prompt_user_input().parse().unwrap();

    for _ in 0..number_of_leverages {
        println!("Enter the multiple of percentage leverages used by the company:");
        let leverage: u32 = prompt_user_input().parse().unwrap();
        company.percentage_leverages.push(leverage);
    }

    // For company data

    for &(name, founded, company assets, company liabilities) in &company_data {
            println!("Enter data for {}", name);

            let mut company = Company::new(name, founded, assets, liabilities);

            if company shares > 20_000_000 {
                save_leverages_to_file(&company);
            }

            if company liabilities < 10_000_000 {
                calculate_5_percent(&mut company);
            }

            companies.push(company);
        }





    

}
