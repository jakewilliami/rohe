extern crate base64;
use hyper::Uri;

extern crate serde_json;

pub fn base64_into_str(base64str: &str) -> String {
    let bytes = base64::decode(base64str).unwrap();
    return String::from_utf8(bytes).expect("Invalid UTF-8 byte sequence");
}

pub fn into_uri(base64str: &str) -> Uri {
    let bytes = base64::decode(base64str).unwrap();
    let uri: Uri = String::from_utf8(bytes)
        .expect("Invalid UTF-8 byte sequence")
        .parse()
        .unwrap();
    return uri;
}

