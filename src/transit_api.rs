use reqwest;
use std::error::Error;
use crate::models::RoutesData;
use crate::models::PlacemarksData;
use std::sync::OnceLock;

const TRANSIT_API_NEARBY_ROUTES_URL: &str = "https://external.transitapp.com/v3/public/nearby_routes";
const TRANSIT_API_PLACEMARKS_URL: &str = "https://external.transitapp.com/v3/map_layers/placemarks";

static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
fn get_client() -> &'static reqwest::Client {
    CLIENT.get_or_init(|| {
        reqwest::Client::new()
    })
}

async fn fetch_data<T: serde::de::DeserializeOwned>(
    url: &str,
    api_key: &str,
    lat: f64,
    lon: f64,
    max_distance: u32,
) -> Result<T, Box<dyn Error>> {

    let client = get_client();

    let response = client.get(url)
        .header("apiKey", api_key)
        .query(&[
            ("lat", lat),
            ("lon", lon),
            ("max_distance", max_distance.into()),
        ])
        .send()
        .await?
        .error_for_status()?;

    let data: T = response.json().await?;
    Ok(data)
}

pub async fn fetch_transit_data(
    api_key: &str,
    lat: f64,
    lon: f64,
    max_distance: u32,
) -> Result<RoutesData, Box<dyn Error>> {
    fetch_data(TRANSIT_API_NEARBY_ROUTES_URL, api_key, lat, lon, max_distance).await
}

pub async fn fetch_placemarks_data(
    api_key: &str,
    lat: f64,
    lon: f64,
    distance: u32,
) -> Result<PlacemarksData, Box<dyn Error>> {
    fetch_data(TRANSIT_API_PLACEMARKS_URL, api_key, lat, lon, distance).await
}









