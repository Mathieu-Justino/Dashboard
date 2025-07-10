use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayElements {

    #[serde(default)] // Use default if the field is missing
    elements: Vec<Option<String>>, // Can contain null or string

    boxed_text: String,

    route_name_redundancy: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Alert {
    created_at: u64,
    description: String,
    effect: String,
    severity: String,
    title: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ParentStation {
    global_stop_id: String,
    location_type: u64,
    rt_stop_id: String,
    station_code: String,
    pub station_name: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ClosestStop {

    pub global_stop_id: String,

    location_type: u8,

    pub parent_station: Option<ParentStation>, // Can be null

    parent_station_global_stop_id: Option<String>, // Can be null

    route_type: u8,

    rt_stop_id: String,

    stop_code: String,

    pub stop_lat: f64,

    pub stop_lon: f64,

    pub stop_name: String,

    wheelchair_boarding: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScheduleItem {

    pub departure_time: u64,

    pub is_cancelled: bool,

    pub is_real_time: bool,

    rt_trip_id: String,

    pub scheduled_departure_time: u64,

    trip_search_key: String,

    wheelchair_accessible: u8,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Itinerary {

    branch_code: String,

    pub closest_stop: Option<ClosestStop>,

    pub direction_headsign: String,

    direction_id: u8,

    headsign: String,

    merged_headsign: String,

    #[serde(default)] // Use default if the field is missing
    pub schedule_items: Vec<ScheduleItem>,
}

// The main `Route` struct
#[derive(Debug, Deserialize, Serialize)]
pub struct Route {

    #[serde(default)] // Use default if the field is missing
    alerts: Vec<Alert>,

    compact_display_short_name: DisplayElements,

    #[serde(default)] // Use default if the field is missing
    fares: Vec<serde_json::Value>, // Using Value for fares since it's an empty array in the sample

    global_route_id: String,

    #[serde(default)] // Use default if the field is missing
    pub itineraries: Vec<Itinerary>,

    pub mode_name: String,

    pub real_time_route_id: String,

    pub route_color: String,

    pub route_display_short_name: DisplayElements,

    pub route_image: String,

    pub route_long_name: String,

    pub route_network_id: String,

    pub route_network_name: String,

    pub route_short_name: String,

    pub route_text_color: String,

    pub route_timezone: String,

    pub route_type: u8,

    pub sorting_key: String,

    pub tts_long_name: String,

    pub tts_short_name: String,

    pub vehicle: Vehicle,
}

// The top-level container for your JSON file
#[derive(Debug, Deserialize, Serialize)]
pub struct RoutesData {
    pub routes: Vec<Route>,
}



#[derive(Debug, Deserialize, Serialize)]
pub struct Vehicle {

    image: String,

    name: String,

    name_inflection: String,
}






#[derive(Debug, Deserialize, Serialize)]
pub struct Placemark {

    pub title: String,

    pub subtitle: String,

    pub id: String,

    #[serde(rename = "networkName")]
    pub network_name: String,

    #[serde(rename = "networkId")]
    pub network_id: String,

    pub color: String,

    #[serde(rename = "textColor")]
    pub text_color: String,

    pub latitude: f64,

    pub longitude: f64,

    pub r#type: String,

}

// The top-level container for your JSON file
#[derive(Debug, Deserialize, Serialize)]
pub struct PlacemarksData {
    pub placemarks: Vec<Placemark>,
}