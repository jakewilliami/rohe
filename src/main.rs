use anyhow::anyhow;
use clap::{ArgAction, Parser, crate_authors, crate_version};

mod constants;
mod postcodes;
mod request;
mod response;
mod utils;

use postcodes::PostcodeConstructor;

#[derive(Parser)]
#[command(
    name = "rohe",
    author = crate_authors!(),
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

    /// [WIP] Return address as (latitude, longitude)
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
        let _ = lookup_postcode(postcode_str).await;
        /*let postcode_in = postcode_str.parse_postcode();

        if let Some(postcodes) = request::get_suggested_postcodes(postcode_in).await {
            if !postcodes.is_empty() {
                for postcode in postcodes {
                    // choose the first postcode and get its unique ID
                    let unique_id = &postcode.unique_id;
                    let full_partial = &postcode.full_partial;

                    // send the unique ID for the chosen postcode to the API
                    if let Some(details) = request::get_postcode_details(*unique_id).await {
                        // Pretty print result
                        let loc = details["CityTown"]
                            .as_str()
                            .expect("Postcode must be somewhere");
                        println!("{full_partial} ∈ {loc}");
                    } else {
                        println!("Could not get details for postcode {full_partial}");
                    }
                }
            } else {
                println!("There was no postcode in the database that matched your input.");
            }
        } else {
            println!("Something went horribly wrong while searching for your input postcode.");
        }*/
    }

    // Find address information
    if let Some(addr) = cli.addr {
        let _ = lookup_address(addr).await;
    }

    // Get address coordinated
    if let Some(_addr_for_coords) = cli.coords {
        todo!();
    }
}

async fn lookup_postcode(postcode_str: String) -> anyhow::Result<()> {
    let postcode_in = postcode_str.parse_postcode();

    let postcodes = request::get_suggested_postcodes(postcode_in)
        .await
        .ok_or_else(|| anyhow!("Error while searching for your input postcode"))?;

    if postcodes.is_empty() {
        return Err(anyhow!(
            "There was no postcode in the database that matched your input."
        ));
    }

    // Report all results
    for postcode in postcodes {
        let unique_id = &postcode.unique_id;
        let full_partial = &postcode.full_partial;

        // Send the unique ID of the postcode to the API for more details
        let details = request::get_postcode_details(*unique_id)
            .await
            .ok_or_else(|| anyhow!("Could not get details for postcode {full_partial}"))?;

        let loc = details["CityTown"]
            .as_str()
            .ok_or_else(|| anyhow!("Postcode must be somewhere"))?;

        println!("{full_partial} ∈ {loc}\n");
    }

    Ok(())
}

async fn lookup_address(addr: String) -> anyhow::Result<()> {
    let addresses = request::get_suggested_addresses(addr)
        .await
        .ok_or_else(|| anyhow!("Error while searching for your input address"))?;

    if addresses.is_empty() {
        return Err(anyhow!(
            "There was no address in the database that matched your input."
        ));
    }

    println!("{addresses:#?}\n");

    Ok(())
}
