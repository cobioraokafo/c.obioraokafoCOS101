use std::io;

fn main() {
	struct Company{
		username:String;
		password:String;
		year:u32;
		company shares:f32;
		company liabilities:f32;
	}

	impl Company {
		fn percentageleverage(&self)->f32 (
		   return self.liabilities / self.shares * 100;
    }

	let cadbury nigeria = Company {
		username:String::from("cadb");
		password:String::from("ckay1");
		year:1965;
		company shares:15000000.0;
		company liabilities:5500000.0;
	}

	let champion breweries = Company {
		username:String::from("cham");
		password:String::from("ckay2");
		year:1974;
		company shares:25000000.0;
		company liabilities:8000000.0;
	}

	let dangote sugar refinery = Company {
		username:String::from("dang");
		password:String::from("ckay3");
		year:1970;
		company shares:18000000.0;
		company liabilities: 10000000.0;
	}
	let flour mills nigeria = Company {
		username:String::from("flou");
		password:String::from("ckay4");
		year:1960;
		company shares: 32000000.0;
		company liabilities: 4000000.0;
	}

	let nestle nigeria = Company {
		username:String::from("nest");
		password:String::from("ckay5");
		year:1961;
		company shares: 8000000.0;
		company liabilities: 15000000.0;
	}

	let unilever nigeria = Company {
		username:String::from("unil");
		password:String::from("ckay6");
		year:1923;
		company shares: 37000000.0;
		company liabilities: 11000000.0;
	}

	let honeywell nigeria = Company {
		username:String::from("hone");
		password:String::from("ckay6");
		year:1906;
		company shares: 34000000.0;
		company liabilities: 9000000.0;
	}

	let nigerian breweries = Company {
		username:String::from("nige");
		password:String::from("ckay7");
		year:1946;
		company shares: 30000000.0;
		company liabilities: 12000000.0;
	}
	let mut file = std::fs::File::create("Companies_information.txt").expect("Failed to create file")






    

}