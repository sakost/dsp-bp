/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

#![allow(dead_code)]

use crate::parsers::factory_bp::building::BlueprintBuildingParameters;
use log::{debug, info};

mod csharptime;
mod entities;
mod errors;
mod md5hash;
mod parsers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let bp = parsers::factory_bp::blueprint::Blueprint::read_from_file("bp.txt", true)?;
    for building in bp.decoded_data()?.buildings.iter() {
        let params = building.get_parameters();
        match params {
            BlueprintBuildingParameters::Station(station_params) => {
                info!(
                    "handling {} station",
                    if station_params.is_planetary() {
                        "planetary"
                    } else {
                        "interstellar"
                    }
                );
                for storage_entry in station_params.storage.iter() {
                    debug!(
                        "{} drone range for {}",
                        station_params.parameters.drone_range,
                        match storage_entry {
                            None => "empty storage slot".to_string(),
                            Some(storage_entry) => format!("{}", storage_entry.item()?),
                        }
                    );
                }
            }
            BlueprintBuildingParameters::Raw(_) => {
                info!("Skipping unknown building");
                continue;
            }
        }
    }
    println!("{}", bp.to_json()?);
    Ok(())
}
