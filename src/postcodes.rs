#[derive(Debug)]
pub struct Postcode {
	pub postcode: String,
}

pub trait PostcodeConstructor {
	fn parse_postcode(&self) -> Postcode;
	// fn to_string(&self) -> String;
}

impl PostcodeConstructor for i64 {
	fn parse_postcode(&self) -> Postcode {
		return Postcode {
			postcode: format!("{:0>4}", &self)
		};
	}
}

impl PostcodeConstructor for i32 {
	fn parse_postcode(&self) -> Postcode {
		return Postcode {
			postcode: format!("{:0>4}", &self)
		};
	}
}

impl PostcodeConstructor for &str {
	fn parse_postcode(&self) -> Postcode {
		return Postcode {
			postcode: self.to_string()
		};
	}
}

impl PostcodeConstructor for String {
	fn parse_postcode(&self) -> Postcode {
		return Postcode {
			postcode: self.to_string()
		};
	}
}

// impl PostcodeConstructor for Postcode {
// 	fn to_string(&self) -> String {
// 		return Postcode.postcode;
// 	}
// }
