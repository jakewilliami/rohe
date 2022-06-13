extern crate base64;
use hyper::Uri;
// use std::str;
// extern crate bytes;
// use bytes::Bytes;

extern crate serde_json;

// #[repr(C)]
// union PostCode {
//     I: isize,
//     S: String
// }

///// FUNCTIONS BEGIN HERE

// fn str2base64(str: String) {
//     return base64::encode(str.as_bytes());
//     // return str.as_bytes().to_base64(base64::STANDARD);
// }

pub fn base64_into_str(base64str: &str) -> String {
    // let bytes = base64::decode(Bytes::from(base64str)).unwrap();
    let bytes = base64::decode(base64str).unwrap();
    return String::from_utf8(bytes).expect("Invalid UTF-8 byte sequence");
}

// fn into_uri(base64str: &str) -> Uri {
//     let bytes = base64::decode(base64str).unwrap();
//     return String::from_utf8(bytes)
//         .expect("Invalid UTF-8 byte sequence")
//         .parse()
//         .unwrap();
// }

pub fn into_uri(base64str: &str) -> Uri {
    let bytes = base64::decode(base64str).unwrap();
    // return String::from_utf8(bytes)
    //     .expect("Invalid UTF-8 byte sequence")
    //     .parse()
    //     .unwrap();
    //     let bytes = Bytes::from("http://example.com/foo");
    // let uri = Uri::from_shared(bytes).unwrap();
    // return base64::decode(base64str)
    //     .unwrap()
    //     .Uri::from_shared(bytes)
    //     .unwrap()
    let uri: Uri = String::from_utf8(bytes)
        .expect("Invalid UTF-8 byte sequence")
        .parse()
        .unwrap();
    return uri;
}

// fn error_if_unsuccessful(j: serde_json::Value) {
// 	if j["success"] || () {
// 		let err = String::new();
// 		err.push_str(j["error"]["message"]);
// 		err.push_str("; error code");
// 		err.push_str(j["error"]["code"].to_string());
//
// 	}
// }
//
// error_if_unsuccessful(j::D) where {D <: Dict} =
// 	j["success"] || error(j["error"]["message"] * "; error code " * string(j["error"]["code"]))
