/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

use crate::errors::InvalidDataCount;
use crate::parsers::factory_bp::utils::{read_i16, read_i8};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct BlueprintArea {
    pub index: i8,
    pub parent_index: i8,
    pub tropic_anchor: i16,
    pub area_segments: i16,
    pub anchor_local_offset_x: i16,
    pub anchor_local_offset_y: i16,
    pub width: i16,
    pub height: i16,
}

impl BlueprintArea {
    pub const SIZE: usize = 1 + 1 + 2 + 2 + 2 + 2 + 2 + 2; // 14 bytes

    pub fn deserialize(
        data: &[u8],
        offset: usize,
    ) -> Result<(Self, usize), Box<dyn std::error::Error>> {
        if data.len() < BlueprintArea::SIZE + offset {
            return Err(InvalidDataCount(data.len()).into());
        }
        let (index, offset) = read_i8(data, offset);
        let (parent_index, offset) = read_i8(data, offset);
        let (tropic_anchor, offset) = read_i16(data, offset);
        let (area_segments, offset) = read_i16(data, offset);
        let (anchor_local_offset_x, offset) = read_i16(data, offset);
        let (anchor_local_offset_y, offset) = read_i16(data, offset);
        let (width, offset) = read_i16(data, offset);
        let (height, offset) = read_i16(data, offset);
        Ok((
            BlueprintArea {
                index,
                parent_index,
                tropic_anchor,
                area_segments,
                anchor_local_offset_x,
                anchor_local_offset_y,
                width,
                height,
            },
            offset,
        ))
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_blueprint_area() {
        let data: Vec<u8> = vec![
            1, 2, // index, parent_index
            0x34, 0x12, // tropic_anchor = 0x1234
            0x78, 0x56, // area_segments = 0x5678
            0xBA, 0x00, // anchor_local_offset_x = 0xBA
            0xDC, 0x00, // anchor_local_offset_y = 0xDC
            0x11, 0x22, // width = 0x2211
            0x33, 0x44, // height = 0x4433
        ];
        let (area, new_offset) = BlueprintArea::deserialize(&data, 0).unwrap();
        assert_eq!(new_offset, BlueprintArea::SIZE);
        assert_eq!(area.index, 1);
        assert_eq!(area.parent_index, 2);
        assert_eq!(area.tropic_anchor, 0x1234);
        assert_eq!(area.area_segments, 0x5678);
        assert_eq!(area.anchor_local_offset_x, 0xBA);
        assert_eq!(area.anchor_local_offset_y, 0xDC);
        assert_eq!(area.width, 0x2211);
        assert_eq!(area.height, 0x4433);
    }
}
