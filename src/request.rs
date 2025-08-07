use super::constants::*;
use super::postcodes::Postcode;
use super::response::*;

/*
Main request method
*/

pub async fn make_request(uri: &str) -> serde_json::Map<String, serde_json::Value> {
    let client = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();

    //// string-based header names
    headers.append("host", BASE_API_URI_STR.parse().unwrap());
    headers.append("accept", "application/json".parse().unwrap());
    headers.append("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/90.0.4430.212 Safari/537.36".parse().unwrap());
    headers.append("origin", FULL_BASE_PUBLIC_URI_STR.parse().unwrap());
    // WE CAN IMPLEMENT THIS SOON // headers.append(ACCEPT_ENCODING, "gzip, deflate".parse().unwrap());
    headers.append("referer", PUBLIC_URI_STR.parse().unwrap());
    headers.append(
        "accept-language",
        "en-GB,en-US;q=0.9,en;q=0.8".parse().unwrap(),
    );
    headers.append("connection", "close".parse().unwrap());

    let res = client
        .get(uri)
        .headers(headers)
        .send()
        .await
        .expect("Failed to access URI");

    let _status = res.status();

    let body = res
        .text()
        .await
        .expect("Failed to retrieve body of response");

    let data: serde_json::Value =
        serde_json::from_str(&body).expect("The JSON response was not well defined");
    let map: serde_json::Map<String, serde_json::Value> = data.as_object().unwrap().clone();

    map
}

/*
Locator methods
*/

/***
```
get_suggested_postcodes(postcode: Postcode) -> Option<Vec<PostcodeInfo>>
```

Sends a request to get matching postcodes based on your input query.  Each returned dictionary contains the keys `"UniqueID"` and `"FullPartial"`.  `UniqueID` is used by `get_postcode_details`.
***/
// I have made the decision to use Option rather than Result because I don't actually want anything to error if it doesn't find anything, just want it to silently return.
pub async fn get_suggested_postcodes(postcode: Postcode) -> Option<Vec<PostcodeInfo>> {
    let mut base_url: String = String::new();
    base_url.push_str(&API_URI_STR);
    base_url.push_str(&UID_QUERY_STR);
    base_url.push_str(postcode.as_str());

    let data: serde_json::Map<String, serde_json::Value> = make_request(base_url.as_str()).await;

    let is_success = &data["success"];
    if is_success != true {
        return None;
    }

    let res = &data["addresses"];
    let potential_postcodes: Vec<PostcodeInfo> =
        serde_json::value::from_value(res.to_owned()).unwrap();

    Some(potential_postcodes)
}

/***
```
get_suggested_addresses(addr: String) -> Option<Vec<Address>>
```

Sends a request to get matching addresses based on your input query.  Each returned dictionary contains the keys `"SourceDesc"`, `"FullAddress"`, and `"DPID"`.  `DPID` is used by `get_address_details`.
***/
pub async fn get_suggested_addresses(addr: String) -> Option<Vec<Address>> {
    let mut base_url: String = String::new();
    base_url.push_str(&API_URI_STR);
    base_url.push_str(&DPID_QUERY_STR);
    base_url.push_str(addr.as_str());

    let data: serde_json::Map<String, serde_json::Value> = make_request(base_url.as_str()).await;

    let is_success = &data["success"];
    if is_success != true {
        return None;
    }

    let res = &data["addresses"];
    let potential_postcodes: Vec<Address> = serde_json::value::from_value(res.to_owned()).unwrap();

    Some(potential_postcodes)
}

/*
Details methods
*/

pub async fn get_postcode_details(
    unique_id: i64,
) -> Option<serde_json::Map<String, serde_json::Value>> {
    let mut base_url: String = String::new();
    base_url.push_str(&API_URI_STR);
    base_url.push_str(&PC_QUERY_STR);
    base_url.push_str(unique_id.to_string().as_str());

    let data: serde_json::Map<String, serde_json::Value> = make_request(base_url.as_str()).await;

    let is_success = &data["success"];
    if is_success != true {
        return None;
    }

    let res = &data["details"];

    let map: serde_json::Map<String, serde_json::Value> =
        res[0].to_owned().as_object().unwrap().clone();

    Some(map)
}
