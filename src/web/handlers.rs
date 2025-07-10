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
use std::collections::HashMap;


const DEFAULT_LAT: f64 = 45.597298;
const DEFAULT_LON: f64 = -73.558154;
const DEFAULT_MAX_DISTANCE: u32 = 5000;


#[derive(Debug, Clone)] // Add Clone if you need to duplicate these items
pub struct FormattedScheduleItem {
    pub departure_time: u64, 
    pub scheduled_departure_time: u64,
    pub is_cancelled: bool,
    pub is_real_time: bool,
}

#[derive(Debug)]
pub struct FormattedRoute {
    pub mode_name: String,
    pub route_color: Option<String>,
    pub route_short_name: String,
    pub stop_name: String,
    pub stop_id: String,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub direction_headsign: String,
    pub upcoming_departures: Vec<FormattedScheduleItem>, 
}

#[derive(Template, IntoResponse)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    pub routes: Vec<FormattedRoute>, 
}

pub async fn root_handler() -> impl IntoResponse {
    let api_key = std::env::var("API_KEY")
        .expect("API_KEY not set for root_handler");

    println!("Attempting to fetch transit data...");

    let routes_data_result = transit_api::fetch_transit_data(
        &api_key,
        DEFAULT_LAT,
        DEFAULT_LON,
        DEFAULT_MAX_DISTANCE
    ).await;

    // Prepare the template context
    let template_data = IndexTemplate {
        routes: Vec::new(), 
    };

    match routes_data_result {
        Ok(routes_data) => {
            println!("API call successful. RoutesData received.");
            println!("Number of top-level routes: {}", routes_data.routes.len());

            let mut aggregated_departures: HashMap<(String, String, String), FormattedRoute> = HashMap::new();
            if routes_data.routes.is_empty() {
                println!("No routes found in the API response.");
            }

            for route_from_api in routes_data.routes { // Renamed for clarity
                for itinerary_from_api in route_from_api.itineraries { // Renamed for clarity
                    
                    println!("    Processing Itinerary: Direction={}", itinerary_from_api.direction_headsign);

                    if let Some(closest_stop_from_api) = itinerary_from_api.closest_stop {

                        println!("      Found Closest Stop: Name={}, ID={}",
                                 closest_stop_from_api.stop_name, closest_stop_from_api.global_stop_id);

                        let key = (
                            route_from_api.global_route_id.clone(),
                            closest_stop_from_api.global_stop_id.clone(),
                            itinerary_from_api.direction_headsign.clone(),
                        );

                        println!("        Generated HashMap Key: {:?}", key);

                        // Get or insert the FormattedRoute for this specific route-stop-direction
                        let entry = aggregated_departures.entry(key.clone()).or_insert_with(|| {
                            println!("          Creating NEW FormattedRoute entry for key: {:?}", key);
                            FormattedRoute {
                                mode_name: route_from_api.mode_name.clone(), // Assuming route_from_api has this
                                route_color: route_from_api.route_color.clone(),   // Assuming route_from_api has this
                                route_short_name: route_from_api.route_short_name.clone(),
                                stop_name: closest_stop_from_api.stop_name.clone(),
                                stop_id: closest_stop_from_api.global_stop_id.clone(),
                                stop_lat: closest_stop_from_api.stop_lat,
                                stop_lon: closest_stop_from_api.stop_lon,
                                direction_headsign: itinerary_from_api.direction_headsign.clone(),
                                upcoming_departures: Vec::new(), // Initialize the list
                            }
                        });

                        // Now, add all schedule items for this itinerary to the `upcoming_departures` list
                        for schedule_item_from_api in itinerary_from_api.schedule_items {
                            let formatted_schedule_item = FormattedScheduleItem {
                                
                                departure_time: schedule_item_from_api.departure_time,
                                scheduled_departure_time: schedule_item_from_api.scheduled_departure_time,
                                is_cancelled: schedule_item_from_api.is_cancelled,
                                is_real_time: schedule_item_from_api.is_real_time,
                            };
                            entry.upcoming_departures.push(formatted_schedule_item);
                        }
                    }
                }
            }

            println!("Total entries in aggregated_departures HashMap: {}", aggregated_departures.len());

            for (_key, formatted_route) in aggregated_departures.iter_mut() {
                 formatted_route.upcoming_departures.sort_by_key(|item| item.departure_time);
            }

            let collected_routes_vec: Vec<FormattedRoute> = aggregated_departures.into_values().collect();

            println!("Number of routes collected into vector: {}", collected_routes_vec.len()); // Check this print!
            
            let template_data = IndexTemplate {
                routes: collected_routes_vec, // Assign the explicitly collected vector
            };

            println!("Final number of routes passed to template: {}", template_data.routes.len());

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