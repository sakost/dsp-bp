/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

#![allow(unexpected_cfgs)]

use crate::entities::item::DysonSphereItem;
use crate::parsers::factory_bp::station::StationParameters;
use crate::parsers::factory_bp::utils::{read_f32, read_i16, read_i32, read_i8};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BlueprintBuilding {
    pub index: i32,
    pub area_index: i8,
    pub local_offset_x: f32,
    pub local_offset_y: f32,
    pub local_offset_z: f32,
    pub local_offset_x2: f32,
    pub local_offset_y2: f32,
    pub local_offset_z2: f32,
    pub yaw: f32,
    pub yaw2: f32,
    pub item_id: i16,
    pub model_index: i16,
    pub output_object_index: i32, // index of item within building array
    pub input_object_index: i32,  // same as output_object_index
    pub output_to_slot: i8,
    pub input_from_slot: i8,
    pub output_from_slot: i8,
    pub input_to_slot: i8,
    pub output_offset: i8,
    pub input_offset: i8,
    pub recipe_id: i16,
    pub filter_id: i16,
    pub tilt: f32,
    pub tilt2: f32,
    pub pitch: f32,
    pub pitch2: f32,
    pub parameters: Vec<i32>,
    pub content: String,
}

impl BlueprintBuilding {
    pub(crate) fn size(&self) -> usize {
        todo!("Implement actual calculation of BlueprintBuilding size")
    }

    /// Deserializing DSP building
    pub fn deserialize(
        data: &[u8],
        offset: usize,
    ) -> Result<(Self, usize), Box<dyn std::error::Error>> {
        let (num, offset) = read_i32(data, offset);

        let mut offset: usize = offset;
        let index: i32;
        let item_id: i16;
        let model_index: i16;
        let area_index: i8;
        let local_offset_x: f32;
        let local_offset_y: f32;
        let local_offset_z: f32;
        let yaw: f32;
        let tilt;
        let pitch: f32;
        let local_offset_x2: f32;
        let local_offset_y2: f32;
        let local_offset_z2: f32;
        let yaw2: f32;
        let tilt2: f32;
        let pitch2: f32;

        let output_object_index: i32;
        let input_object_index: i32;
        let output_to_slot: i8;
        let input_from_slot: i8;
        let output_from_slot: i8;
        let input_to_slot: i8;
        let output_offset: i8;
        let input_offset: i8;
        let recipe_id: i16;
        let filter_id: i16;

        let parameter_count: i16;

        let mut parameters: Vec<i32>;

        let content: String = "".to_string(); // todo implement content reading from -102

        // wtf? why -102? -101? -100?
        if num <= -102 {
            (index, offset) = read_i32(data, offset);
            (item_id, offset) = read_i16(data, offset);
            (model_index, offset) = read_i16(data, offset);
            (area_index, offset) = read_i8(data, offset);
            (local_offset_x, offset) = read_f32(data, offset);
            (local_offset_y, offset) = read_f32(data, offset);
            (local_offset_z, offset) = read_f32(data, offset);
            (yaw, offset) = read_f32(data, offset);

            let item = DysonSphereItem::try_from(item_id as i32)?;
            if item.is_conveyor_belt() {
                (tilt, offset) = read_f32(data, offset);
                pitch = 0.;
                (local_offset_x2, offset) = read_f32(data, offset);
                (local_offset_y2, offset) = read_f32(data, offset);
                (local_offset_z2, offset) = read_f32(data, offset);
                yaw2 = yaw;
                tilt2 = tilt;
                pitch2 = 0.;
            } else if item.is_sorter() {
                (tilt, offset) = read_f32(data, offset);
                (pitch, offset) = read_f32(data, offset);
                (local_offset_x2, offset) = read_f32(data, offset);
                (local_offset_y2, offset) = read_f32(data, offset);
                (local_offset_z2, offset) = read_f32(data, offset);
                (yaw2, offset) = read_f32(data, offset);
                (tilt2, offset) = read_f32(data, offset);
                (pitch2, offset) = read_f32(data, offset);
            } else {
                tilt = 0.;
                pitch = 0.;
                (local_offset_x2, offset) = read_f32(data, offset);
                (local_offset_y2, offset) = read_f32(data, offset);
                (local_offset_z2, offset) = read_f32(data, offset);
                yaw2 = yaw;
                tilt2 = 0.;
                pitch2 = 0.;
            }

            (output_object_index, offset) = read_i32(data, offset);
            (input_object_index, offset) = read_i32(data, offset);
            (output_to_slot, offset) = read_i8(data, offset);
            (input_from_slot, offset) = read_i8(data, offset);
            (output_from_slot, offset) = read_i8(data, offset);
            (input_to_slot, offset) = read_i8(data, offset);
            (output_offset, offset) = read_i8(data, offset);
            (input_offset, offset) = read_i8(data, offset);
            (recipe_id, offset) = read_i16(data, offset);
            (filter_id, offset) = read_i16(data, offset);

            (parameter_count, offset) = read_i16(data, offset);

            parameters = vec![0; parameter_count as usize];

            for param_idx in 0..parameter_count {
                (parameters[param_idx as usize], offset) = read_i32(data, offset);
            }

            let (content_len, new_offset) = read_i32(data, offset);
            offset = new_offset;

            // todo: add content reading. now just skip
            for _ in 0..content_len {
                let (_, new_offset) = read_i8(data, offset);
                offset = new_offset;
            }
        } else if num <= -101 {
            (index, offset) = read_i32(data, offset);
            (item_id, offset) = read_i16(data, offset);
            (model_index, offset) = read_i16(data, offset);
            (area_index, offset) = read_i8(data, offset);
            (local_offset_x, offset) = read_f32(data, offset);
            (local_offset_y, offset) = read_f32(data, offset);
            (local_offset_z, offset) = read_f32(data, offset);
            (yaw, offset) = read_f32(data, offset);

            let item = DysonSphereItem::try_from(item_id as i32)?;
            if item.is_conveyor_belt() {
                (tilt, offset) = read_f32(data, offset);
                pitch = 0.;
                local_offset_x2 = local_offset_x;
                local_offset_y2 = local_offset_y;
                local_offset_z2 = local_offset_z;
                yaw2 = yaw;
                tilt2 = tilt;
                pitch2 = 0.;
            } else if item.is_sorter() {
                (tilt, offset) = read_f32(data, offset);
                (pitch, offset) = read_f32(data, offset);
                (local_offset_x2, offset) = read_f32(data, offset);
                (local_offset_y2, offset) = read_f32(data, offset);
                (local_offset_z2, offset) = read_f32(data, offset);
                (yaw2, offset) = read_f32(data, offset);
                (tilt2, offset) = read_f32(data, offset);
                (pitch2, offset) = read_f32(data, offset);
            } else {
                tilt = 0.;
                pitch = 0.;
                local_offset_x2 = local_offset_x;
                local_offset_y2 = local_offset_y;
                local_offset_z2 = local_offset_z;
                yaw2 = yaw;
                tilt2 = 0.;
                pitch2 = 0.;
            }
            (output_object_index, offset) = read_i32(data, offset);
            (input_object_index, offset) = read_i32(data, offset);
            (output_to_slot, offset) = read_i8(data, offset);
            (input_from_slot, offset) = read_i8(data, offset);
            (output_from_slot, offset) = read_i8(data, offset);
            (input_to_slot, offset) = read_i8(data, offset);
            (output_offset, offset) = read_i8(data, offset);
            (input_offset, offset) = read_i8(data, offset);
            (recipe_id, offset) = read_i16(data, offset);
            (filter_id, offset) = read_i16(data, offset);

            (parameter_count, offset) = read_i16(data, offset);

            parameters = vec![0; parameter_count as usize];

            for param_idx in 0..parameter_count {
                (parameters[param_idx as usize], offset) = read_i32(data, offset);
            }
        } else if num <= -100 {
            (index, offset) = read_i32(data, offset);
            (area_index, offset) = read_i8(data, offset);
            (local_offset_x, offset) = read_f32(data, offset);
            (local_offset_y, offset) = read_f32(data, offset);
            (local_offset_z, offset) = read_f32(data, offset);
            (local_offset_x2, offset) = read_f32(data, offset);
            (local_offset_y2, offset) = read_f32(data, offset);
            (local_offset_z2, offset) = read_f32(data, offset);
            pitch = 0.;
            pitch2 = 0.;
            (yaw, offset) = read_f32(data, offset);
            (yaw2, offset) = read_f32(data, offset);
            (tilt, offset) = read_f32(data, offset);
            tilt2 = 0.;
            (item_id, offset) = read_i16(data, offset);
            (model_index, offset) = read_i16(data, offset);

            (output_object_index, offset) = read_i32(data, offset);
            (input_object_index, offset) = read_i32(data, offset);
            (output_to_slot, offset) = read_i8(data, offset);
            (input_from_slot, offset) = read_i8(data, offset);
            (output_from_slot, offset) = read_i8(data, offset);
            (input_to_slot, offset) = read_i8(data, offset);
            (output_offset, offset) = read_i8(data, offset);
            (input_offset, offset) = read_i8(data, offset);
            (recipe_id, offset) = read_i16(data, offset);
            (filter_id, offset) = read_i16(data, offset);

            (parameter_count, offset) = read_i16(data, offset);

            parameters = vec![0; parameter_count as usize];

            for param_idx in 0..parameter_count {
                (parameters[param_idx as usize], offset) = read_i32(data, offset);
            }
        } else {
            index = num;
            (area_index, offset) = read_i8(data, offset);
            (local_offset_x, offset) = read_f32(data, offset);
            (local_offset_y, offset) = read_f32(data, offset);
            (local_offset_z, offset) = read_f32(data, offset);
            (local_offset_x2, offset) = read_f32(data, offset);
            (local_offset_y2, offset) = read_f32(data, offset);
            (local_offset_z2, offset) = read_f32(data, offset);
            pitch = 0.;
            pitch2 = 0.;
            (yaw, offset) = read_f32(data, offset);
            (yaw2, offset) = read_f32(data, offset);
            tilt = 0.;
            tilt2 = 0.;
            (item_id, offset) = read_i16(data, offset);
            (model_index, offset) = read_i16(data, offset);

            (output_object_index, offset) = read_i32(data, offset);
            (input_object_index, offset) = read_i32(data, offset);
            (output_to_slot, offset) = read_i8(data, offset);
            (input_from_slot, offset) = read_i8(data, offset);
            (output_from_slot, offset) = read_i8(data, offset);
            (input_to_slot, offset) = read_i8(data, offset);
            (output_offset, offset) = read_i8(data, offset);
            (input_offset, offset) = read_i8(data, offset);
            (recipe_id, offset) = read_i16(data, offset);
            (filter_id, offset) = read_i16(data, offset);

            (parameter_count, offset) = read_i16(data, offset);

            parameters = vec![0; parameter_count as usize];

            for param_idx in 0..parameter_count {
                (parameters[param_idx as usize], offset) = read_i32(data, offset);
            }
        }
        Ok((
            BlueprintBuilding {
                index,
                area_index,
                local_offset_x,
                local_offset_y,
                local_offset_z,
                local_offset_x2,
                local_offset_y2,
                local_offset_z2,
                yaw,
                yaw2,
                pitch,
                pitch2,
                item_id,
                model_index,
                output_object_index,
                input_object_index,
                output_to_slot,
                input_from_slot,
                output_from_slot,
                input_to_slot,
                output_offset,
                input_offset,
                recipe_id,
                filter_id,
                tilt,
                tilt2,
                parameters,
                content,
            },
            offset,
        ))
    }

    /// If item_id is unknown id – returns None.
    pub fn item(&self) -> Option<DysonSphereItem> {
        let item = (self.item_id as i32).try_into();
        match item {
            Err(_) => None,
            Ok(item) => Some(item),
        }
    }

    /// Returns parsed BlueprintBuildingParameters
    pub fn get_parameters(&self) -> BlueprintBuildingParameters {
        match self.item() {
            Some(DysonSphereItem::PlanetaryLogisticsStation) => {
                BlueprintBuildingParameters::Station(StationParameters::new(
                    &self.parameters,
                    4,
                    12,
                ))
            }
            Some(DysonSphereItem::InterstellarLogisticsStation) => {
                BlueprintBuildingParameters::Station(StationParameters::new(
                    &self.parameters,
                    5,
                    12,
                ))
            }
            _ => BlueprintBuildingParameters::Raw(self.parameters.clone()),
        }
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub enum BlueprintBuildingParameters {
    Station(StationParameters),
    // todo: add other buildings
    // Splitter(SplitterParameters),
    Raw(Vec<i32>),
}

impl BlueprintBuildingParameters {
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[cfg(FALSE)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsers::factory_bp::common_dto::DysonSphereItem;

    #[test]
    fn test_deserialize_blueprint_building_header() {
        let mut data = vec![0u8; BlueprintBuilding::HEADER_SIZE];
        data[0..4].copy_from_slice(&1u32.to_le_bytes()); // index
        data[4] = 2; // area_index
        data[5..9].copy_from_slice(&1.0f32.to_le_bytes()); // local_offset_x
        data[9..13].copy_from_slice(&2.0f32.to_le_bytes()); // local_offset_y
        data[13..17].copy_from_slice(&3.0f32.to_le_bytes()); // local_offset_z
        data[17..21].copy_from_slice(&4.0f32.to_le_bytes()); // local_offset_x2
        data[21..25].copy_from_slice(&5.0f32.to_le_bytes()); // local_offset_y2
        data[25..29].copy_from_slice(&6.0f32.to_le_bytes()); // local_offset_z2
        data[29..33].copy_from_slice(&0.5f32.to_le_bytes()); // yaw
        data[33..37].copy_from_slice(&0.6f32.to_le_bytes()); // yaw2
        data[37..39].copy_from_slice(&1u16.to_le_bytes()); // item_id = 1 (PlanetaryLogisticsStation)
        data[39..41].copy_from_slice(&0u16.to_le_bytes()); // model_index
        data[41..45].copy_from_slice(&10u32.to_le_bytes()); // output_object_index
        data[45..49].copy_from_slice(&20u32.to_le_bytes()); // input_object_index
        data[49] = 1; // output_to_slot
        data[50] = 2; // input_from_slot
        data[51] = 3; // output_from_slot
        data[52] = 4; // input_to_slot
        data[53] = 5; // output_offset
        data[54] = 6; // input_offset
        data[55..57].copy_from_slice(&100u16.to_le_bytes()); // recipe_id
        data[57..59].copy_from_slice(&200u16.to_le_bytes()); // filter_id
        data[59..61].copy_from_slice(&2u16.to_le_bytes()); // parameter_count = 2

        let (header, pos_after) = BlueprintBuildingHeader::deserialize(&data, 0);
        assert_eq!(pos_after, BlueprintBuildingHeader::SIZE);
        assert_eq!(header.index, 1);
        assert_eq!(header.area_index, 2);
        assert!((header.local_offset_x - 1.0).abs() < 1e-6);
        assert_eq!(header.item_id, 1);
        assert_eq!(header.parameter_count, 2);
    }

    #[test]
    fn test_deserialize_blueprint_building() {
        let mut header_data = vec![0u8; BlueprintBuildingHeader::SIZE];
        header_data[0..4].copy_from_slice(&1u32.to_le_bytes()); // index
        header_data[4] = 2; // area_index
        header_data[5..9].copy_from_slice(&1.0f32.to_le_bytes()); // local_offset_x
        header_data[9..13].copy_from_slice(&2.0f32.to_le_bytes()); // local_offset_y
        header_data[13..17].copy_from_slice(&3.0f32.to_le_bytes()); // local_offset_z
        header_data[17..21].copy_from_slice(&4.0f32.to_le_bytes()); // local_offset_x2
        header_data[21..25].copy_from_slice(&5.0f32.to_le_bytes()); // local_offset_y2
        header_data[25..29].copy_from_slice(&6.0f32.to_le_bytes()); // local_offset_z2
        header_data[29..33].copy_from_slice(&0.5f32.to_le_bytes()); // yaw
        header_data[33..37].copy_from_slice(&0.6f32.to_le_bytes()); // yaw2
        header_data[37..39].copy_from_slice(&1u16.to_le_bytes()); // item_id = 1
        header_data[39..41].copy_from_slice(&0u16.to_le_bytes()); // model_index
        header_data[41..45].copy_from_slice(&10u32.to_le_bytes()); // output_object_index
        header_data[45..49].copy_from_slice(&20u32.to_le_bytes()); // input_object_index
        header_data[49] = 1; // output_to_slot
        header_data[50] = 2; // input_from_slot
        header_data[51] = 3; // output_from_slot
        header_data[52] = 4; // input_to_slot
        header_data[53] = 5; // output_offset
        header_data[54] = 6; // input_offset
        header_data[55..57].copy_from_slice(&100u16.to_le_bytes()); // recipe_id
        header_data[57..59].copy_from_slice(&200u16.to_le_bytes()); // filter_id
        header_data[59..61].copy_from_slice(&1u16.to_le_bytes()); // parameter_count = 1

        let param_data = 999u32.to_le_bytes();

        let mut data = header_data;
        data.extend_from_slice(&param_data);

        let (building, pos_after) = BlueprintBuilding::deserialize(&data, 0);
        assert_eq!(pos_after, BlueprintBuildingHeader::SIZE + 4);
        assert_eq!(building.header.index, 1);
        assert_eq!(building.header.parameter_count, 1);
        assert_eq!(building.parameters.len(), 1);
        assert_eq!(building.parameters[0], 999);
        assert_eq!(building.item(), Some(DysonSphereItem::from(1)));
    }
}
