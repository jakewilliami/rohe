mod postcodes;
mod request;
mod response;
mod constants;
mod utils;

use postcodes::*;
use response::*;

extern crate clap;
use clap::{ArgAction, crate_version, Parser};

#[derive(Parser)]
#[command(
	name = "bats",
	author = "Jake·W.·Ireland.·<jakewilliami@icloud.com>",
	version = crate_version!(),
	about = "A command line interface for NZP's locator API.",
	long_about = "A command line interface for NZP's locator API.  The name 'rohe' is the Māori word for 'areas'.",
)]
struct Cli {
	/// Takes address as input.  Default return value is the address' associated postcode
	#[arg(
		short = 'a',
		long = "address",
		action = ArgAction::Set,
		num_args = 0..=1,
		value_name = "address",
	)]
	addr: Option<String>,

	/// Takes postcode as input.  Default return value is the postcode's associated region
	#[arg(
		short = 'p',
		long = "postcode",
		action = ArgAction::Set,
		num_args = 0..=1,
		value_name = "postcode"
	)]
	postcode: Option<String>,

	/// Return address as (latitude, longitude)
	#[arg(
		short = 'c',
		long = "coordinates",
		action = ArgAction::Set,
		num_args = 0..=1,
		value_name = "address",
	)]
	coords: Option<String>,
}

#[tokio::main]
async fn main() {
	let cli = Cli::parse();

	// Find postcode information
	if let Some(postcode_str) = cli.postcode {
		let bad_response: &str = "There was no postcode in the database that matched your input.";

		// get value of postcode
		let postcode = postcode_str.parse_postcode();

		// request postcodes from the API
		let matched_postcodes: Option<Vec<EachPostcode>> = request::get_suggested_postcodes(postcode).await;

		// initialise the response string
		let mut resp = String::new();
		if matched_postcodes.as_ref().is_none() || matched_postcodes.as_ref().unwrap().is_empty() {
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
				if let Some(details) = details {
					resp.push_str(full_partial);
					resp.push_str(" ∈ ");
					resp.push_str(details["CityTown"].as_str().unwrap());
					if i != (postcodes.len() - 1) {
						resp.push('\n');
					}
				} else {
					resp.push_str(bad_response);
				}
			}
		}

		println!("{}", resp);
	}

	// Find address information
	if let Some(addr) = cli.addr {
		// get value of address
		let resp: Option<Vec<EachAddress>> = request::get_suggested_addresses(addr).await;
		println!("{:?}", resp);
	}

	// Get address coordinated
	if let Some(_addr_for_coords) = cli.coords {
		todo!();
	}
}
