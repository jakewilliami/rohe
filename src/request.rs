// #[path = "postcodes.rs"] mod postcodes;
use super::postcodes::Postcode;
use super::response::*;
use super::constants::*;
// use super::utils::*;

// extern crate base64;
// extern crate hyper;
// extern crate futures;
// extern crate
extern crate reqwest;
use reqwest::header::*;
// use reqwest::Url
// use hyper::{Client, Uri, HeaderMap, Request};
// use hyper::Uri;
// use std::str;
// extern crate bytes;
// use bytes::Bytes;

extern crate serde_json;

/*
=== Main request method ===
*/

// #[tokio::main]
// async url: Uri
pub async fn make_request(uri: &str) -> serde_json::Map<String, serde_json::Value> {
// pub async fn make_request(uri: &str) -> String {
// pub async fn make_request(uri: &str) -> impl Future<Output = serde_json::Value> {
    // build the client using the builder pattern
    let client = reqwest::Client::new();
    //     .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36")
    //     .build()
    //     .expect("Failed to initilise HTTP interface");
    // let client = reqwest::Client::builder()
    //     .build()
    //     .expect("Failed to initialise HTTP interface");
    // let mut headers = header
    // let mut headers = reqwest::header::HeaderMap::new(); headers.insert(reqwest::header::USER_AGENT,reqwest::header::HeaderValue::from_static("curl"));
    let mut headers = reqwest::header::HeaderMap::new();
    
    //// const reqwest header names
    // headers.append(HOST, BASE_API_URI_STR.parse().unwrap());
    // // headers.append("Sec-Ch-Ua", "\" Not A;Brand\";v=\"99\", \"Chromium\";v=\"90\"".parse().unwrap());
    // headers.append(ACCEPT, "application/json".parse().unwrap());
    // // headers.append("Sec-Ch-Ua-Mobile", "?0".parse().unwrap());
    // headers.append(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36".parse().unwrap());
    // headers.append(ORIGIN, FULL_BASE_PUBLIC_URI_STR.parse().unwrap());
    // // WE CAN IMPLEMENT THIS SOON // headers.append(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    // // headers.append("Sec-Fetch-Site", "same-site".parse().unwrap());
    // // headers.append("Sec-Fetch-Mode", "cors".parse().unwrap());
    // // headers.append("Sec-Fetch-Dest", "empty".parse().unwrap());
    // headers.append(REFERER, PUBLIC_URI_STR.parse().unwrap());
    // headers.append(ACCEPT_LANGUAGE, "en-GB,en-US;q=0.9,en;q=0.8".parse().unwrap());
    // headers.append(CONNECTION, "close".parse().unwrap());
    
    //// string-based header names
    headers.append("host", BASE_API_URI_STR.parse().unwrap());
    // headers.append("Sec-Ch-Ua", "\" Not A;Brand\";v=\"99\", \"Chromium\";v=\"90\"".parse().unwrap());
    headers.append("accept", "application/json".parse().unwrap());
    // headers.append("Sec-Ch-Ua-Mobile", "?0".parse().unwrap());
    headers.append("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36".parse().unwrap());
    headers.append("origin", FULL_BASE_PUBLIC_URI_STR.parse().unwrap());
    // WE CAN IMPLEMENT THIS SOON // headers.append(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    // headers.append("Sec-Fetch-Site", "same-site".parse().unwrap());
    // headers.append("Sec-Fetch-Mode", "cors".parse().unwrap());
    // headers.append("Sec-Fetch-Dest", "empty".parse().unwrap());
    headers.append("referer", PUBLIC_URI_STR.parse().unwrap());
    headers.append("accept-language", "en-GB,en-US;q=0.9,en;q=0.8".parse().unwrap());
    headers.append("connection", "close".parse().unwrap());
    
    // println!("Requesting from: {}", uri);
    
    // let uri: Uri = postcode_uri.parse().unwrap();
    
    let res = client
        .get(uri)
        .headers(headers)
        // .headers_mut()
        // .insert("host", BASE_API_URI_STR.parse().unwrap())
        // .insert("accept", "application/json".parse().unwrap())
        // .insert("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36".parse().unwrap())
        // .insert("origin", FULL_BASE_PUBLIC_URI_STR.parse().unwrap())
        // .insert("referer", PUBLIC_URI_STR.parse().unwrap())
        // .insert("accept-language", "en-GB,en-US;q=0.9,en;q=0.8".parse().unwrap())
        // .insert("connection", "close".parse().unwrap())
        .send()
        .await
        .expect("Failed to access URI");
    
    // match res { // .await
    //     Ok(res) => println!("Resp: {}", res.status()),
    //     Err(err) => println!("Error: {}", err),
    // }
    
    let status = res.status();
    
    let body = res
		.text()
		.await
		.expect("Failed to retrieve body of response");
    
    // println!("Resp: {}", body);
    
    let data: serde_json::Value = serde_json::from_str(&body).expect("The JSON response was not well defined");
    let map: serde_json::Map<String, serde_json::Value> = data.as_object().unwrap().clone();
    
    return map;
    // return body;
}

/*
=== Locator methods ===
*/

/***
```
get_suggested_postcodes(postcode: Postcode) -> Option<Vec<EachPostcode>>
```

Sends a request to get matching postcodes based on your input query.  Each returned dictionary contains the keys `"UniqueID"` and `"FullPartial"`.  `UniqueID` is used by `get_postcode_details`.
***/
// I have made the decision to use Option rather than Result because I don't actually want anything to error if it doesn't find anything, just want it to silently return.
// pub async fn get_suggested_postcodes(postcode: Postcode) -> Result<serde_json::Value, &'static serde_json::Value> {
// pub async fn get_suggested_postcodes(postcode: Postcode) -> Option<&'static serde_json::Value> {
// pub async fn get_suggested_postcodes(postcode: Postcode) -> Option<Vec<EachPostcode>> {
// pub async fn get_suggested_postcodes(postcode: Postcode) -> Option<serde_json::Map<String, serde_json::Value>> {
pub async fn get_suggested_postcodes(postcode: Postcode) -> Option<Vec<EachPostcode>> {
    let mut base_URL: String = String::new();
    base_URL.push_str(&API_URI_STR);
    base_URL.push_str(&UID_QUERY_STR);
    base_URL.push_str(postcode.postcode.as_str());
    
    let data: serde_json::Map<String, serde_json::Value> = make_request(base_URL.as_str()).await;
    // let data = make_request(base_URL.as_str()).await;
    // let deserialised: PostcodeResponse = serde_json::from_str(data.as_str()).unwrap();
    // println!("{}", base_URL);
    // let resp = make_request(base_URL.as_str());
    // print_type_of(&resp);
    
    let is_success = &data["success"];
    // let is_success: bool = deserialised.success;
    if is_success != true {
        // let err = String::new();
		// err.push_str(data["error"]["message"].as_str().unwrap());
		// err.push_str("; error code");
		// err.push_str(data["error"]["code"].as_str());
		// return Err(err);
        // let err = &data["error"]["message"];
        // let err = serde_json::from_value(json["subtree"][0].take())?
        // return Err(err);
        return None;
	}
    
    // let deserialised: PostcodeSearchResponse = serde_json::value::from_value(data["addresses"]).unwrap();
    let res = &data["addresses"];
    // let potential_postcodes: serde_json::Map<String, serde_json::Value> = res.as_object().unwrap().clone();
    let potential_postcodes: Vec<EachPostcode> = serde_json::value::from_value(res.to_owned()).unwrap();
    
    // println!("{:?}", deserialised);
    // println!("{:?}", potential_postcodes);
    
    // let response_key = "addresses";
    
    // match res { // .await
    //     Ok(res) => println!("Resp: {}", res.status()),
    //     Err(err) => println!("Error: {}", err),
    // }
    
    // return Ok(data[response_key]);
    // let potential_postcodes = data.get(response_key);
    // return potential_postcodes;
    // let potential_postcodes = &data[response_key];
    
    return Some(potential_postcodes);
    // return Some(potential_postcodes);
    // return Some(deserialised.addresses)
    // return Some(potential_postcodes);
    
    // if potential_postcodes.is_some() {
    //     // return Some(potential_postcodes.unwrap())
    //     return potential_postcodes;
    // } else {
    //     return None;
    // }
    // return Some(data.get(response_key).unwrap())
}

/***
```
get_suggested_addresses(addr: String) -> Option<Vec<EachAddress>>
```

Sends a request to get matching addresses based on your input query.  Each returned dictionary contains the keys `"SourceDesc"`, `"FullAddress"`, and `"DPID"`.  `DPID` is used by `get_address_details`.
***/
pub async fn get_suggested_addresses(addr: String) -> Option<Vec<EachAddress>> {
    let mut base_URL: String = String::new();
    base_URL.push_str(&API_URI_STR);
    base_URL.push_str(&DPID_QUERY_STR);
    base_URL.push_str(addr.as_str());
    
    let data: serde_json::Map<String, serde_json::Value> = make_request(base_URL.as_str()).await;
    
    let is_success = &data["success"];
    if is_success != true {
        return None;
	}
    
    let res = &data["addresses"];
    let potential_postcodes: Vec<EachAddress> = serde_json::value::from_value(res.to_owned()).unwrap();
    
    return Some(potential_postcodes);
}

/*
=== Details methods ===
*/

// NOTE
// THIS OUTPUT OF -p IS PROBABLY WRONG
/*
./rohe -p 20
2010 ∈ Auckland
2012 ∈ Auckland
2013 ∈ Auckland
State Highway 20, Point Chevalier, Auckland 1022 ∈ Auckland
State Highway 20, Waterview, Auckland 1026 ∈ Auckland
*/
// Why is "State Highway 20, Point Chevalier, Auckland 1022" the "full partial" for a post code?

pub async fn get_postcode_details(unique_id: i64) -> Option<serde_json::Map<String, serde_json::Value>> {
	let mut base_URL: String = String::new();
    base_URL.push_str(&API_URI_STR);
    base_URL.push_str(&PC_QUERY_STR);
    base_URL.push_str(unique_id.to_string().as_str());
	
	let data: serde_json::Map<String, serde_json::Value> = make_request(base_URL.as_str()).await;
	
	let is_success = &data["success"];
	if is_success != true {
		return None;
	}
	
	let res = &data["details"];
	// println!("{:?}", res);
	// let details: serde_json::Value = serde_json::value::from_value(res.to_owned()).unwrap();
	
	let map: serde_json::Map<String, serde_json::Value> = res[0].to_owned().as_object().unwrap().clone();
	
	return Some(map);
	
	// "FullPartial"
	// "CityTown"
	
	// return Some(details);
}
