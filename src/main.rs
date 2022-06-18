mod postcodes;
mod request;
mod response;
mod constants;
mod utils;

use postcodes::*;
use response::*;

extern crate clap;
use clap::{Arg, App};

#[tokio::main]
async fn main() {
    let matches = App::new("rohe")
                      .version("1.0")
                      .author("Jake W. Ireland. <jakewilliami@icloud.com>")
                      .about("A command line interface for NZP's locator API.  The name 'rohe' is the Māori word for 'areas'.")
							.arg(Arg::with_name("ADDR")
								.short("a")
								.long("address")
								.help("Takes address as input.  Default return value is the address' associated postcode.")
								.takes_value(true)
								.required(false)
								.multiple(false)
							)
							.arg(Arg::with_name("POSTCODE")
								.short("p")
								.long("postcode")
								.help("Takes postcode as input.  Default return value is the postcode's associated region.")
								.takes_value(true)
								.required(false)
								.multiple(false)
						   	)
							.arg(Arg::with_name("ADDR_FOR_COORDS")
								.short("c")
								.long("coordinates")
								.help("Return addresses as (latitude, longitude).")
								.takes_value(true)
								.required(false)
								.multiple(false)
						   	)
							.get_matches();
	
	// Find postcode information
	if matches.is_present("POSTCODE") {
		let bad_response: &str = "There was no postcode in the database that matched your input.";
		
		// get value of postcode
		let postcode = matches.value_of("POSTCODE").unwrap().parse_postcode();
		
		// request postcodes from the API
		let matched_postcodes: Option<Vec<EachPostcode>> = request::get_suggested_postcodes(postcode).await;
		
		// initialise the response string
		let mut resp = String::new();
		if matched_postcodes.as_ref().is_none() || matched_postcodes.as_ref().unwrap().len() == 0 {
			resp.push_str(bad_response);
		} else {
			let postcodes = &matched_postcodes.unwrap();
			for i in 0..postcodes.len() {
				// choose the first postcode and get its unique ID
				let chosen_postcode: &EachPostcode = &postcodes[i];
				// let chosen_postcodes =
				let unique_id: &i64 = &chosen_postcode.UniqueId;
				let full_partial: &str = &chosen_postcode.FullPartial;
				
				// send the unique ID for the chosen postcode to the API
				let details: Option<serde_json::Map<String, serde_json::Value>> = request::get_postcode_details(*unique_id).await;
				
				// construct the response string
				if details.is_none() {
					resp.push_str(bad_response);
				} else {
					resp.push_str(full_partial);
					resp.push_str(" ∈ ");
					resp.push_str(details.unwrap()["CityTown"].as_str().unwrap());
					if i != (postcodes.len() - 1) {
						resp.push_str("\n")
					}
				}
			}
		}
		
		println!("{}", resp);
	}
	
	// Find address information
	if matches.is_present("ADDR") {
		// get value of address
		let addr = matches.value_of("ADDR").unwrap();
		let resp: Option<Vec<EachAddress>> = request::get_suggested_addresses(addr.to_string()).await;
		println!("{:?}", resp);
	}
}
