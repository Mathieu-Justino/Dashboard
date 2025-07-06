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

#[derive(Template, IntoResponse)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    // These fields correspond to the variables you use in templates/index.html
    pub closest_stop_name: Option<String>,
    pub closest_stop_id: Option<String>,
    pub closest_stop_lat: Option<f64>,
    pub closest_stop_lon: Option<f64>,
    pub closest_stop_parent_station_name: Option<String>,
    // If you wanted to pass full route data, uncomment this and the processing below:
    // pub routes: Vec<FormattedRoute>,
}
// Optional: If you wanted to pass more complex data like formatted routes and schedules
// #[derive(Debug)]
// pub struct FormattedRoute {
//     pub short_name: String,
//     pub long_name: String,
//     pub direction_name: String,
//     pub description: Option<String>,
//     pub itineraries: Vec<FormattedItinerary>,
// }

// #[derive(Debug)]
// pub struct FormattedItinerary {
//     pub schedule_items: Vec<FormattedScheduleItem>,
// }

// #[derive(Debug)]
// pub struct FormattedScheduleItem {
//     pub trip_headsign: Option<String>,
//     pub minutes_until_departure: i64,
// }

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
        closest_stop_name: None,
        closest_stop_id: None,
        closest_stop_lat: None,
        closest_stop_lon: None,
        closest_stop_parent_station_name: None,
        // routes: Vec::new(), // Initialize if passing routes
    };

    match routes_data_result {
        Ok(routes_data) => {
            let mut found_closest_stop = false;

            // Iterate to find the first closest stop and populate template_data
            for route in routes_data.routes {
                for itinerary in route.itineraries {
                    if let Some(closest_stop) = itinerary.closest_stop {
                        template_data.closest_stop_name = Some(closest_stop.stop_name);
                        template_data.closest_stop_id = Some(closest_stop.global_stop_id);
                        template_data.closest_stop_lat = Some(closest_stop.stop_lat);
                        template_data.closest_stop_lon = Some(closest_stop.stop_lon);

                        if let Some(parent_station_details) = closest_stop.parent_station {
                            template_data.closest_stop_parent_station_name = Some(parent_station_details.station_name);
                        }
                        found_closest_stop = true;

                        // If you wanted to pass all schedule items for this closest stop,
                        // you'd format them here and add to template_data.
                        // Example:
                        // let now_utc = chrono::Utc::now();
                        // let now_montreal = now_utc.with_timezone(&chrono_tz::America::Montreal);
                        // let mut formatted_schedule_items = Vec::new();
                        // for item in itinerary.schedule_items {
                        //     let departure_utc = chrono::Utc.timestamp_opt(item.departure_time as i64, 0).single().unwrap_or_default();
                        //     let departure_montreal = departure_utc.with_timezone(&chrono_tz::America::Montreal);
                        //     let duration_until_departure = departure_montreal.signed_duration_since(now_montreal);
                        //     let minutes_until_departure = duration_until_departure.num_minutes();
                        //     formatted_schedule_items.push(FormattedScheduleItem {
                        //         trip_headsign: item.trip_headsign,
                        //         minutes_until_departure,
                        //     });
                        // }
                        // template_data.schedule_items = formatted_schedule_items;

                        break; // Exit itinerary loop after finding first stop
                    }
                }
                if found_closest_stop {
                    break; // Exit route loop
                }
            }

            // Render the template and return the response.
            template_data.into_response() // Askama's Template trait implements IntoResponse
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
