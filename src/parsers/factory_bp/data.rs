/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

#![allow(unexpected_cfgs)]

use crate::errors::{CorruptedData, InvalidDataCount};
use crate::parsers::factory_bp::area::BlueprintArea;
use crate::parsers::factory_bp::building::BlueprintBuilding;
use crate::parsers::factory_bp::utils::{read_i32, read_i8};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BlueprintDataHeader {
    pub patch: i32,
    pub cursor_offset_x: i32,
    pub cursor_offset_y: i32,
    pub cursor_target_area: i32,
    pub dragbox_size_x: i32,
    pub dragbox_size_y: i32,
    pub primary_area_index: i32,
    pub area_count: i8,
}

impl BlueprintDataHeader {
    pub const SIZE: usize = 4 * 7 + 1; // 29 bytes

    pub fn deserialize(
        data: &[u8],
        offset: usize,
    ) -> Result<(Self, usize), Box<dyn std::error::Error>> {
        if data.len() < BlueprintDataHeader::SIZE + offset {
            return Err(InvalidDataCount(data.len()).into());
        }
        let (patch, offset) = read_i32(data, offset);
        let (cursor_offset_x, offset) = read_i32(data, offset);
        let (cursor_offset_y, offset) = read_i32(data, offset);
        let (cursor_target_area, offset) = read_i32(data, offset);
        let (dragbox_size_x, offset) = read_i32(data, offset);
        let (dragbox_size_y, offset) = read_i32(data, offset);
        let (primary_area_index, offset) = read_i32(data, offset);
        let (area_count, offset) = read_i8(data, offset);
        if !(0..=64).contains(&area_count)
            || primary_area_index < -1
            || primary_area_index > area_count as i32
        {
            return Err(CorruptedData.into());
        }
        Ok((
            BlueprintDataHeader {
                patch,
                cursor_offset_x,
                cursor_offset_y,
                cursor_target_area,
                dragbox_size_x,
                dragbox_size_y,
                primary_area_index,
                area_count,
            },
            offset,
        ))
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub struct BuildingHeader {
    pub building_count: i32,
}

impl BuildingHeader {
    pub const SIZE: usize = 4;
    pub fn deserialize(
        data: &[u8],
        offset: usize,
    ) -> Result<(Self, usize), Box<dyn std::error::Error>> {
        if data.len() < BlueprintDataHeader::SIZE + offset {
            return Err(InvalidDataCount(data.len()).into());
        }
        let (building_count, offset) = read_i32(data, offset);
        Ok((BuildingHeader { building_count }, offset))
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BlueprintData {
    pub header: BlueprintDataHeader,
    pub areas: Vec<BlueprintArea>,
    pub buildings: Vec<BlueprintBuilding>,
}

impl BlueprintData {
    pub fn size(&self) -> usize {
        todo!("Implement calculating size of BlueprintData")
    }

    pub fn deserialize(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let (header, mut offset) = BlueprintDataHeader::deserialize(data, 0)?;
        let mut areas = Vec::with_capacity(header.area_count as usize);
        for _ in 0..header.area_count {
            let (area, new_offset) = BlueprintArea::deserialize(data, offset)?;
            offset = new_offset;
            areas.push(area);
        }
        let (b_header, new_offset) = BuildingHeader::deserialize(data, offset)?;
        offset = new_offset;
        let mut buildings = Vec::with_capacity(b_header.building_count as usize);
        for _ in 0..b_header.building_count {
            let (building, new_offset) = BlueprintBuilding::deserialize(data, offset)?;
            offset = new_offset;
            buildings.push(building);
        }
        let mut bp_data = BlueprintData {
            header,
            areas,
            buildings,
        };
        bp_data.data_repair();
        Ok(bp_data)
    }

    /// something from DSP decompiled code
    fn data_repair(&mut self) {
        let num = self.areas.len();
        if self.header.patch < 1
            && num > 0
            && ((self.areas[0].area_segments == 4 && self.areas[0].anchor_local_offset_x == 17)
                || (self.areas[num - 1].area_segments == 4
                    && self.areas[num - 1].anchor_local_offset_x == 17))
        {
            for i in 0..num {
                self.areas[i].anchor_local_offset_x = 0;
            }
        }
    }

    /// Serialize all structure to formatted JSON.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[cfg(FALSE)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_blueprint_data_header() {
        let mut data = vec![0u8; BlueprintDataHeader::SIZE];
        data[0..4].copy_from_slice(&1u32.to_le_bytes()); // patch
        data[4..8].copy_from_slice(&2u32.to_le_bytes()); // cursor_offset_x
        data[8..12].copy_from_slice(&3u32.to_le_bytes()); // cursor_offset_y
        data[12..16].copy_from_slice(&4u32.to_le_bytes()); // cursor_target_area
        data[16..20].copy_from_slice(&5u32.to_le_bytes()); // dragbox_size_x
        data[20..24].copy_from_slice(&6u32.to_le_bytes()); // dragbox_size_y
        data[24..28].copy_from_slice(&7u32.to_le_bytes()); // primary_area_index
        data[28] = 1; // area_count
        let (header, new_offset) = BlueprintDataHeader::deserialize(&data, 0).unwrap();
        assert_eq!(header.patch, 1);
        assert_eq!(header.area_count, 1);
        assert_eq!(new_offset, BlueprintDataHeader::SIZE);
    }

    #[test]
    fn test_deserialize_blueprint_data() {
        let mut data = Vec::new();

        // Header:
        {
            let mut header = vec![0u8; BlueprintDataHeader::SIZE];
            header[0..4].copy_from_slice(&1u32.to_le_bytes()); // patch
            header[4..8].copy_from_slice(&0u32.to_le_bytes());
            header[8..12].copy_from_slice(&0u32.to_le_bytes());
            header[12..16].copy_from_slice(&0u32.to_le_bytes());
            header[16..20].copy_from_slice(&0u32.to_le_bytes());
            header[20..24].copy_from_slice(&0u32.to_le_bytes());
            header[24..28].copy_from_slice(&0u32.to_le_bytes());
            header[28] = 1; // area_count = 1
            data.extend_from_slice(&header);
        }

        // 1 Area:
        {
            let area = vec![
                1, 2, // index, parent_index
                0x34, 0x12, // tropic_anchor = 0x1234
                0x78, 0x56, // area_segments = 0x5678
                0xAA, 0xBB, // anchor_local_offset_x = 0xBBAA
                0xCC, 0xDD, // anchor_local_offset_y = 0xDDCC
                0x11, 0x22, // width = 0x2211
                0x33, 0x44, // height = 0x4433
            ];
            data.extend_from_slice(&area);
        }

        // BuildingHeader: building_count = 1
        {
            data.extend_from_slice(&1u32.to_le_bytes());
        }

        // 1 Building:
        {
            let mut building = vec![0u8; BlueprintBuilding::HEADER_SIZE];
            building[0..4].copy_from_slice(&1u32.to_le_bytes()); // index
            building[4] = 0; // area_index
            building[5..9].copy_from_slice(&1.0f32.to_le_bytes());
            building[9..13].copy_from_slice(&2.0f32.to_le_bytes());
            building[13..17].copy_from_slice(&3.0f32.to_le_bytes());
            building[17..21].copy_from_slice(&4.0f32.to_le_bytes());
            building[21..25].copy_from_slice(&5.0f32.to_le_bytes());
            building[25..29].copy_from_slice(&6.0f32.to_le_bytes());
            building[29..33].copy_from_slice(&0.5f32.to_le_bytes());
            building[33..37].copy_from_slice(&0.6f32.to_le_bytes());
            building[37..39].copy_from_slice(&1u16.to_le_bytes());
            building[39..41].copy_from_slice(&0u16.to_le_bytes());
            building[41..45].copy_from_slice(&10u32.to_le_bytes());
            building[45..49].copy_from_slice(&20u32.to_le_bytes());
            building[49] = 1;
            building[50] = 2;
            building[51] = 3;
            building[52] = 4;
            building[53] = 5;
            building[54] = 6;
            building[55..57].copy_from_slice(&100u16.to_le_bytes());
            building[57..59].copy_from_slice(&200u16.to_le_bytes());
            building[59..61].copy_from_slice(&1u16.to_le_bytes()); // parameter_count = 1
            data.extend_from_slice(&building);

            // Parameter: 4 bytes
            data.extend_from_slice(&999u32.to_le_bytes());
        }

        let blueprint = BlueprintData::deserialize(&data).unwrap();
        assert_eq!(blueprint.header.patch, 1);
        assert_eq!(blueprint.areas.len(), 1);
        assert_eq!(blueprint.buildings.len(), 1);
    }
}
