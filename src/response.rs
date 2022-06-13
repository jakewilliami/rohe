use serde::{Serialize, Deserialize};

// successful postcode response

#[derive(Serialize, Deserialize, Debug)]
pub struct EachPostcode {
    pub UniqueId: i64,
    pub FullPartial: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PostcodeSearchResponse {
    pub success: bool,
    pub addresses: Vec<EachPostcode>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct PostcodeResponse {
//     // pub success: bool,
//     pub addresses: Vec<EachPostcode>,
// }

// successful address response

#[derive(Serialize, Deserialize, Debug)]
pub struct EachAddress {
    pub SourceDesc: String,
    pub FullAddress: String,
    pub DPID: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AddressSearchResponse {
    pub addresses: Vec<EachAddress>,
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
