/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

#![allow(unexpected_cfgs)]

use crate::entities::item::DysonSphereItem;
use crate::errors::UnknownDysonSphereItem;
use serde::Serialize;

#[derive(Debug, Serialize, PartialEq, Copy, Clone)]
pub struct StorageEntry {
    pub item_id: i32,
    pub local_logic: i32,
    pub remote_logic: i32,
    pub max_count: i32,
    pub keep_mode: i32,
    pub keep_inc: i32,
}

impl StorageEntry {
    pub fn item(&self) -> Result<DysonSphereItem, UnknownDysonSphereItem> {
        DysonSphereItem::try_from(self.item_id)
    }
}

#[derive(Debug, Serialize, PartialEq, Copy, Clone)]
pub struct SlotEntry {
    pub direction: LogisticsStationDirection,
    pub storage_index: i32,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct Parameters {
    pub work_energy: i32,
    pub drone_range: i32,
    pub vessel_range: i32,
    pub orbital_collector: bool,
    pub warp_distance: i32,
    pub equip_warper: bool,
    pub drone_count: i32,
    pub vessel_count: i32,
    pub piler_count: i32,
    pub drone_auto_replenish: bool,
    pub vessel_auto_replenish: bool,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct StationParameters {
    pub storage: Vec<Option<StorageEntry>>,
    pub slots: Vec<Option<SlotEntry>>,
    pub parameters: Parameters,
}

impl StationParameters {
    const STORAGE_OFFSET: usize = 0;
    const SLOTS_OFFSET: usize = Self::STORAGE_OFFSET + 192;
    const PARAMETERS_OFFSET: usize = Self::SLOTS_OFFSET + 128;

    pub fn new(params: &[i32], storage_len: usize, slots_len: usize) -> Self {
        let storage = Self::parse_storage(params, storage_len);
        let slots = Self::parse_slots(params, slots_len);
        let parameters = Self::parse_parameters(params);
        StationParameters {
            storage,
            slots,
            parameters,
        }
    }

    fn parse_storage(params: &[i32], storage_len: usize) -> Vec<Option<StorageEntry>> {
        let mut storage = Vec::with_capacity(storage_len);
        for i in 0..storage_len {
            let offset = Self::STORAGE_OFFSET + i * 6;
            let item_id = params[offset];
            if item_id == 0 {
                storage.push(None);
            } else {
                storage.push(Some(StorageEntry {
                    item_id,
                    local_logic: params[offset + 1],
                    remote_logic: params[offset + 2],
                    max_count: params[offset + 3],
                    keep_mode: params[offset + 4],
                    keep_inc: params[offset + 5],
                }));
            }
        }
        storage
    }

    fn parse_slots(params: &[i32], slots_len: usize) -> Vec<Option<SlotEntry>> {
        let mut slots = Vec::with_capacity(slots_len);
        for i in 0..slots_len {
            let offset = Self::SLOTS_OFFSET + i * 4;
            let storage_index = params[offset + 1];
            if storage_index == 0 {
                slots.push(None);
            } else {
                slots.push(Some(SlotEntry {
                    direction: LogisticsStationDirection::from(params[offset] as u8),
                    storage_index,
                }));
            }
        }
        slots
    }

    #[allow(clippy::identity_op)]
    fn parse_parameters(params: &[i32]) -> Parameters {
        Parameters {
            work_energy: params[Self::PARAMETERS_OFFSET + 0],
            drone_range: params[Self::PARAMETERS_OFFSET + 1],
            vessel_range: params[Self::PARAMETERS_OFFSET + 2],
            orbital_collector: params[Self::PARAMETERS_OFFSET + 3] == 1,
            warp_distance: params[Self::PARAMETERS_OFFSET + 4],
            equip_warper: params[Self::PARAMETERS_OFFSET + 5] == 1,
            drone_count: params[Self::PARAMETERS_OFFSET + 6],
            vessel_count: params[Self::PARAMETERS_OFFSET + 7],
            piler_count: params[Self::PARAMETERS_OFFSET + 8],
            // 9 is for vein collector speed mining
            drone_auto_replenish: params[Self::PARAMETERS_OFFSET + 10] == 1,
            vessel_auto_replenish: params[Self::PARAMETERS_OFFSET + 11] == 1,
            // 12, 13 for group priority
            // 14 is for route priority
        }
    }

    pub fn is_interstellar(&self) -> bool {
        self.storage.len() == 5
    }
    pub fn is_planetary(&self) -> bool {
        self.storage.len() == 4
    }

    /// Serializing to JSON-string
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[derive(Debug, Serialize, PartialEq, Copy, Clone)]
pub enum LogisticsStationDirection {
    North,
    East,
    South,
    West,
    Unknown(u8),
}

impl From<u8> for LogisticsStationDirection {
    fn from(value: u8) -> Self {
        match value {
            0 => LogisticsStationDirection::North,
            1 => LogisticsStationDirection::East,
            2 => LogisticsStationDirection::South,
            3 => LogisticsStationDirection::West,
            other => LogisticsStationDirection::Unknown(other),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::parsers::factory_bp::station::LogisticsStationDirection;

    #[test]
    fn test_logistics_station_direction() {
        assert_eq!(
            LogisticsStationDirection::from(0),
            LogisticsStationDirection::North
        );
        assert_eq!(
            LogisticsStationDirection::from(1),
            LogisticsStationDirection::East
        );
        assert_eq!(
            LogisticsStationDirection::from(2),
            LogisticsStationDirection::South
        );
        assert_eq!(
            LogisticsStationDirection::from(3),
            LogisticsStationDirection::West
        );
        assert_eq!(
            LogisticsStationDirection::from(10),
            LogisticsStationDirection::Unknown(10)
        );
    }
}
#[cfg(FALSE)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_storage() {
        let mut params = vec![0u32; 192 + 128 + 8];
        params[0] = 1; // item_id != 0
        params[1] = 10; // local_logic
        params[2] = 20; // remote_logic
        params[3] = 30; // max_count
        let station = StationParameters::new(&params, 2, 0);
        assert!(station.storage[0].is_some());
        assert!(station.storage[1].is_none());
    }

    #[test]
    fn test_parse_slots() {
        let mut params = vec![0u32; 192 + 128 + 8];
        let slot_offset = 192;
        params[slot_offset] = 1; // direction
        params[slot_offset + 1] = 2; // storage_index (не ноль)
        let station = StationParameters::new(&params, 0, 1);
        assert!(station.slots[0].is_some());
    }

    #[test]
    fn test_parse_parameters() {
        let mut params = vec![0u32; 192 + 128 + 8];
        let par_offset = 192 + 128;
        params[par_offset + 0] = 100;
        params[par_offset + 1] = 200;
        params[par_offset + 2] = 300;
        params[par_offset + 3] = 1; // orbital_collector true
        params[par_offset + 4] = 400;
        params[par_offset + 5] = 0; // equip_warper false
        params[par_offset + 6] = 50;
        params[par_offset + 7] = 60;
        let station = StationParameters::new(&params, 0, 0);
        assert_eq!(station.parameters.work_energy, 100);
        assert_eq!(station.parameters.orbital_collector, true);
    }
}
