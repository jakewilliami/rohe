#[derive(Debug)]
pub struct Postcode(String);

impl Postcode {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub trait PostcodeConstructor {
    fn parse_postcode(&self) -> Postcode;
}

impl PostcodeConstructor for i64 {
    fn parse_postcode(&self) -> Postcode {
        Postcode(format!("{:0>4}", &self))
    }
}

impl PostcodeConstructor for i32 {
    fn parse_postcode(&self) -> Postcode {
        Postcode(format!("{:0>4}", &self))
    }
}

impl PostcodeConstructor for &str {
    fn parse_postcode(&self) -> Postcode {
        Postcode(self.to_string())
    }
}

impl PostcodeConstructor for String {
    fn parse_postcode(&self) -> Postcode {
        Postcode(self.to_string())
    }
}
