use serde::{Deserialize, Serialize};

// successful postcode response

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PostcodeInfo {
    pub unique_id: i64,
    pub full_partial: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostcodeSearchResponse {
    pub success: bool,
    pub addresses: Vec<PostcodeInfo>,
}

// successful address response

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    pub source_desc: String,
    pub full_address: String,
    #[serde(rename = "DPID")]
    pub dp_id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressSearchResponse {
    pub addresses: Vec<Address>,
    pub status: String,
    pub success: bool,
}

/*
#[derive(Serialize, Deserialize, Debug)]
pub struct GeoProperties {
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CRS {
    pub properties: GeoProperties,
    pub type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NZGD2kCoordContainer {
    pub coordinates: Vec<f64>,
    pub type: String,
    pub crs: CRS,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NZGD2kBBOXContainer {
    pub coordinates: Vec<Vec<f64>>,
    pub type: String,
    pub crs: CRS,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressDetails {
    pub RoadSuffixName: String,
    pub CityTown: String,
    pub BoxBagPostcode: bool,
    pub FullPartial: String,
    pub NZGD2kCoord: NZGD2kCoordContainer,
    pub NZGD2kBBOX: NZGD2kBBOXContainer,
    pub RoadTypeName: String,
    pub Postcode: String,
    pub SuburbName: String,
    pub RoadName: String,
    pub UniqueId: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressDetailsResponse {
    pub details: Vec<AddressDetails>,
    pub success: bool,
}
*/

// unsuccessful response

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorDetails {
    pub message: String,
    pub code: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BadResponse {
    pub success: bool,
    pub error: ErrorDetails,
}
