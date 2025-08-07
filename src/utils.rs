use base64::prelude::*;
use hyper::Uri;

pub fn base64_into_str(base64str: &str) -> String {
    let bytes = BASE64_STANDARD.decode(base64str).unwrap();
    String::from_utf8(bytes).expect("Invalid UTF-8 byte sequence")
}

pub fn into_uri(base64str: &str) -> Uri {
    let bytes = BASE64_STANDARD.decode(base64str).unwrap();
    String::from_utf8(bytes)
        .expect("Invalid UTF-8 byte sequence")
        .parse()
        .unwrap()
}
