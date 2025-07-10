use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
// Make sure to import all relevant structs from models
use crate::{models::{RoutesData, ClosestStop, ParentStation, Route, Itinerary, ScheduleItem}, transit_api};
use std::error::Error;
use askama::Template;
use askama_derive_axum::IntoResponse;
use axum::extract::Query;
use serde::Deserialize;


const DEFAULT_LAT: f64 = 45.597298;
const DEFAULT_LON: f64 = -73.558154;
const DEFAULT_MAX_DISTANCE: u32 = 5000;


#[derive(Debug)]
pub struct FormattedRoute {
    // These fields correspond to the variables you use in templates/index.html
    pub mode_name: String,
    pub route_color: Option<String>,
    pub route_short_name: String,
    pub stop_name: String,
    pub stop_id: String,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub direction_headsign: String,
    pub departure_time: u64,
    pub is_cancelled: bool,
    pub scheduled_departure_time: u64,
    pub is_real_time: bool,
}

#[derive(Template, IntoResponse)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub routes: Vec<FormattedRoute>, 
}

pub async fn root_handler() -> impl IntoResponse {
    let api_key = std::env::var("API_KEY")
        .expect("API_KEY not set for root_handler");

    let routes_data_result = transit_api::fetch_transit_data(
        &api_key,
        DEFAULT_LAT,
        DEFAULT_LON,
        DEFAULT_MAX_DISTANCE
    ).await;

    // Prepare the template context
    let mut template_data = IndexTemplate {
        routes: Vec::new(), 
    };

    match routes_data_result {
        Ok(routes_data) => {
            for route_from_api in routes_data.routes { // Renamed for clarity
                for itinerary_from_api in route_from_api.itineraries { // Renamed for clarity
                    if let Some(closest_stop_from_api) = itinerary_from_api.closest_stop {

                        // Iterate through schedule items to create a FormattedRoute for each departure
                        for schedule_item_from_api in itinerary_from_api.schedule_items {
                             // Assuming `direction_name` is the headsign for this itinerary

                            template_data.routes.push(FormattedRoute {
                                mode_name: route_from_api.mode_name.clone(), // Assuming route_from_api has this
                                route_color: Some(route_from_api.route_color.clone()),   // Assuming route_from_api has this
                                route_short_name: route_from_api.route_short_name.clone(),
                                stop_name: closest_stop_from_api.stop_name.clone(),
                                stop_id: closest_stop_from_api.global_stop_id.clone(),
                                stop_lat: closest_stop_from_api.stop_lat,
                                stop_lon: closest_stop_from_api.stop_lon,
                                direction_headsign: itinerary_from_api.direction_headsign.clone(),
                                departure_time: schedule_item_from_api.departure_time,
                                is_cancelled: schedule_item_from_api.is_cancelled,
                                scheduled_departure_time: schedule_item_from_api.scheduled_departure_time,
                                is_real_time: schedule_item_from_api.is_real_time,
                            });
                        }
                    }
                }
            }
            // Render the template and return the response.
            template_data.into_response()
        }
        Err(e) => {
            eprintln!("Error fetching transit data: {}", e);
            // On error, return a generic error HTML page
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Html(format!("<h1>Error Fetching Data</h1><p>Could not retrieve transit information: {}</p><p>Please check your API key and try again.</p>", e))
            ).into_response()
        }
    }
}