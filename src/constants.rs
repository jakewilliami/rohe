use super::utils::*;
use hyper::Uri;

extern crate lazy_static;
use lazy_static::lazy_static;

pub const TOOLS0: &str = "dG9vbHMubnpwb3N0LmNvLm56";
pub const TOOLS1: &str = "aHR0cHM6Ly90b29scy5uenBvc3QuY28ubnovbGVnYWN5L2FwaS9zdWdnZXN0X3BhcnRpYWw=";
pub const TOOLS2: &str = "aHR0cHM6Ly90b29scy5uenBvc3QuY28ubnovbGVnYWN5L2FwaS9wYXJ0aWFsX2RldGFpbHM=";
pub const BASE0: &str = "aHR0cHM6Ly93d3cubnpwb3N0LmNvLm56";
pub const BASE1: &str = "aHR0cHM6Ly93d3cubnpwb3N0LmNvLm56L3Rvb2xzL2FkZHJlc3MtcG9zdGNvZGUtZmluZGVy";

pub const GLOB_BASE_ENCODED: &str = "aHR0cHM6Ly93d3cu";
const BASE_PUBLIC_URI_ENCODED: &str = "bnpwb3N0LmNvLm56";
// const PUBLIC_URI_ENCODED = "aHR0cHM6Ly93d3cubnpwb3N0LmNvLm56L3Rvb2xzL2FkZHJlc3MtcG9zdGNvZGUtZmluZGVy";
pub const PUBLIC_URI_ENCODED: &str = "L3Rvb2xzL2FkZHJlc3MtcG9zdGNvZGUtZmluZGVy";
pub const BASE_API_URI_ENCODED: &str = "dG9vbHMubnpwb3N0LmNvLm56";
pub const API_URI_ENCODED: &str = "aHR0cHM6Ly90b29scy5uenBvc3QuY28ubnovbGVnYWN5L2FwaS8=";

pub const LOCATOR_SUFFIX_ENCODED: &str = "Jk1heERhdGE9bWF4JTNBMTA=";
pub const UID_QUERY_ENCODED: &str = "L3N1Z2dlc3RfcGFydGlhbD9xPQ==";
pub const DPID_QUERY_ENCODED: &str = "L3N1Z2dlc3Q/cT0=";
pub const PC_QUERY_ENCODED: &str = "L3BhcnRpYWxfZGV0YWlscz91bmlxdWVfaWQ9";
pub const ADDR_QUERY_ENCODED: &str = "L2RldGFpbHM/ZHBpZD0=";

pub const COORD_KEY_ENCODED: &str = "TlpHRDJrQ29vcmQ=";
pub const UID_KEY_ENCODED: &str = "VW5pcXVlSWQ=";
pub const PC_KEY_ENCODED: &str = "RnVsbFBhcnRpYWw=";
pub const REGION_KEY_ENCODED: &str = "Q2l0eVRvd24=";

lazy_static!{
    // static ref STATE: Mutex<MyState> = Mutex::new(MyState::whatever());
    
    // AS URIs
    pub static ref GLOB_BASE: Uri = into_uri(GLOB_BASE_ENCODED);
    pub static ref BASE_PUBLIC_URI: Uri = into_uri(BASE_PUBLIC_URI_ENCODED);
    pub static ref FULL_BASE_PUBLIC_URI: Uri = into_uri(
        format!("{}{}",
            base64_into_str(GLOB_BASE_ENCODED),
            base64_into_str(BASE_PUBLIC_URI_ENCODED)
        ).as_str()
    );
    // static ref PUBLIC_URI: String = GLOB_BASE
    //     .push_str(into_uri(BASE_PUBLIC_URI_ENCODED))
    //     .push_str(into_uri(PUBLIC_URI_ENCODED));
    pub static ref PUBLIC_URI: Uri = into_uri(
        format!("{}{}{}",
            base64_into_str(GLOB_BASE_ENCODED),
            base64_into_str(BASE_PUBLIC_URI_ENCODED),
            base64_into_str(PUBLIC_URI_ENCODED)
        ).as_str()
    );
    // static ref PUBLIC_URI: String = String::new()
    //     .push_str(&GLOB_BASE)
    //     .push_str(&into_uri(BASE_PUBLIC_URI_ENCODED))
    //     .push_str(&into_uri(PUBLIC_URI_ENCODED));
    // PUBLIC_URI.push_str(into_uri(BASE_PUBLIC_URI_ENCODED));
    // PUBLIC_URI.push_str(into_uri(PUBLIC_URI_ENCODED));
    pub static ref BASE_API_URI: Uri = into_uri(BASE_API_URI_ENCODED);
    pub static ref API_URI: Uri = into_uri(API_URI_ENCODED);
    
    pub static ref LOCATOR_SUFFIX: Uri = into_uri(LOCATOR_SUFFIX_ENCODED);
    pub static ref UID_QUERY: Uri = into_uri(UID_QUERY_ENCODED);
    pub static ref DPID_QUERY: Uri = into_uri(DPID_QUERY_ENCODED);
    pub static ref PC_QUERY: Uri = into_uri(PC_QUERY_ENCODED);
    pub static ref ADDR_QUERY: Uri = into_uri(ADDR_QUERY_ENCODED);
    
    pub static ref COORD_KEY: Uri = into_uri(COORD_KEY_ENCODED);
    pub static ref UID_KEY: Uri = into_uri(UID_KEY_ENCODED);
    pub static ref PC_KEY: Uri = into_uri(PC_KEY_ENCODED);
    pub static ref REGION_KEY: Uri = into_uri(REGION_KEY_ENCODED);
    
    // AS STRINGS
    pub static ref GLOB_BASE_STR: String = base64_into_str(GLOB_BASE_ENCODED);
    pub static ref BASE_PUBLIC_URI_STR: String = base64_into_str(BASE_PUBLIC_URI_ENCODED);
    pub static ref FULL_BASE_PUBLIC_URI_STR: String = format!("{}{}",
        base64_into_str(GLOB_BASE_ENCODED),
        base64_into_str(BASE_PUBLIC_URI_ENCODED)
    );
    pub static ref PUBLIC_URI_STR: String = format!("{}{}{}",
        base64_into_str(GLOB_BASE_ENCODED),
        base64_into_str(BASE_PUBLIC_URI_ENCODED),
        base64_into_str(PUBLIC_URI_ENCODED)
    );
    pub static ref BASE_API_URI_STR: String = base64_into_str(BASE_API_URI_ENCODED);
    pub static ref API_URI_STR: String = base64_into_str(API_URI_ENCODED);
    
    pub static ref LOCATOR_SUFFIX_STR: String = base64_into_str(LOCATOR_SUFFIX_ENCODED);
    pub static ref UID_QUERY_STR: String = base64_into_str(UID_QUERY_ENCODED);
    pub static ref DPID_QUERY_STR: String = base64_into_str(DPID_QUERY_ENCODED);
    pub static ref PC_QUERY_STR: String = base64_into_str(PC_QUERY_ENCODED);
    pub static ref ADDR_QUERY_STR: String = base64_into_str(ADDR_QUERY_ENCODED);
    
    pub static ref COORD_KEY_STR: String = base64_into_str(COORD_KEY_ENCODED);
    pub static ref UID_KEY_STR: String = base64_into_str(UID_KEY_ENCODED);
    pub static ref PC_KEY_STR: String = base64_into_str(PC_KEY_ENCODED);
    pub static ref REGION_KEY_STR: String = base64_into_str(REGION_KEY_ENCODED);
}
