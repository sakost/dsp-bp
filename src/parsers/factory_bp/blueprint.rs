/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

use base64::Engine;
use chrono::NaiveDateTime;
use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use urlencoding::{decode, encode};

use crate::csharptime;
use crate::md5hash;
use crate::parsers::factory_bp::data;

#[derive(Debug, Clone)]
pub struct Blueprint {
    layout: u32,
    icon0: u32,
    icon1: u32,
    icon2: u32,
    icon3: u32,
    icon4: u32,
    timestamp: NaiveDateTime,
    game_version: String,
    short_desc: String,
    long_desc: String,
    data: Vec<u8>,
}

/// Custom error type for hash validation errors.
#[derive(Debug)]
pub struct InvalidHashValueException(String);

impl std::fmt::Display for InvalidHashValueException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InvalidHashValueException: {}", self.0)
    }
}
impl std::error::Error for InvalidHashValueException {}

impl Blueprint {
    /// Constructor.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        game_version: String,
        data: Vec<u8>,
        layout: u32,
        icon0: u32,
        icon1: u32,
        icon2: u32,
        icon3: u32,
        icon4: u32,
        timestamp: Option<NaiveDateTime>,
        short_desc: String,
        long_desc: String,
    ) -> Self {
        let ts =
            timestamp.unwrap_or_else(|| csharptime::csharp_to_datetime(csharptime::csharp_now()));
        Blueprint {
            layout,
            icon0,
            icon1,
            icon2,
            icon3,
            icon4,
            timestamp: ts,
            game_version,
            short_desc,
            long_desc,
            data,
        }
    }

    /// Getter for timestamp.
    pub fn timestamp(&self) -> NaiveDateTime {
        self.timestamp
    }
    /// Setter for timestamp.
    pub fn set_timestamp(&mut self, value: NaiveDateTime) {
        self.timestamp = value;
    }

    /// Getter for short description.
    pub fn short_desc(&self) -> &str {
        &self.short_desc
    }
    /// Setter for short description.
    pub fn set_short_desc(&mut self, value: String) {
        self.short_desc = value;
    }

    /// Getter for long description.
    pub fn long_desc(&self) -> &str {
        &self.long_desc
    }
    /// Setter for long description.
    pub fn set_long_desc(&mut self, value: String) {
        self.long_desc = value;
    }

    /// Returns deserialized blueprint data.
    pub fn decoded_data(&self) -> Result<data::BlueprintData, Box<dyn std::error::Error>> {
        data::BlueprintData::deserialize(&self.data)
    }

    /// Parses a blueprint string and returns a Blueprint instance.
    ///
    /// If `validate_hash` is true, computes the MD5 hash of the header portion and compares it
    /// with the reference hash appended at the end.
    pub fn from_blueprint_string(
        bp_string: &str,
        validate_hash: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        // Validate hash if requested.
        if validate_hash {
            let index = bp_string
                .rfind('"')
                .ok_or("No double quote found for hash separation")?;
            let hashed_data = &bp_string[..index];
            let ref_value = bp_string[index + 1..].trim().to_lowercase();
            let computed_hash = md5hash::DysonSphereMD5::new(md5hash::Variant::MD5F)
                .update(hashed_data.as_bytes())
                .finalize()
                .hexdigest()?;
            if ref_value != computed_hash {
                return Err(Box::new(InvalidHashValueException(
                    "Blueprint string has invalid hash value.".to_string(),
                )));
            }
        }

        // Ensure string starts with "BLUEPRINT:".
        if !bp_string.starts_with("BLUEPRINT:") {
            return Err("Blueprint string does not start with 'BLUEPRINT:'".into());
        }
        let content = &bp_string[10..]; // Remove prefix.
        let components: Vec<&str> = content.split(',').collect();
        if components.len() != 12 {
            return Err("Invalid number of components in blueprint string".into());
        }

        // Unpack components.
        let fixed0_1: i32 = components[0].parse()?;
        let layout: u32 = components[1].parse()?;
        let icon0: u32 = components[2].parse()?;
        let icon1: u32 = components[3].parse()?;
        let icon2: u32 = components[4].parse()?;
        let icon3: u32 = components[5].parse()?;
        let icon4: u32 = components[6].parse()?;
        let fixed0_2: i32 = components[7].parse()?;
        let timestamp_ticks: i64 = components[8].parse()?;
        let game_version = components[9].to_string();
        let short_desc_enc = components[10];
        let b64data_hash = components[11];

        if fixed0_1 != 0 || fixed0_2 != 0 {
            return Err("Fixed components are not zero".into());
        }

        let timestamp = csharptime::csharp_to_datetime(timestamp_ticks);
        let short_desc = decode(short_desc_enc)?.into_owned();

        // b64data_hash is expected to have three parts separated by double quotes.
        let parts: Vec<&str> = b64data_hash.split('"').collect();
        if parts.len() != 3 {
            return Err("Invalid format for b64data_hash".into());
        }
        let long_desc_enc = parts[0];
        let b64data = parts[1];
        // The third part is the hash value (ignored here)
        let long_desc = decode(long_desc_enc)?.into_owned();

        // Decode base64 and decompress gzip.
        let compressed_data = base64::engine::general_purpose::STANDARD.decode(b64data)?;
        let mut gz = GzDecoder::new(&compressed_data[..]);
        let mut data = Vec::new();
        gz.read_to_end(&mut data)?;

        Ok(Blueprint::new(
            game_version,
            data,
            layout,
            icon0,
            icon1,
            icon2,
            icon3,
            icon4,
            Some(timestamp),
            short_desc,
            long_desc,
        ))
    }

    /// Serializes the blueprint into a blueprint string.
    pub fn serialize(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Compress the data using gzip.
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(&self.data)?;
        let compressed_data = encoder.finish()?;
        let b64_data = base64::engine::general_purpose::STANDARD.encode(&compressed_data);

        // Build header components.
        let components = vec![
            "0".to_string(),
            self.layout.to_string(),
            self.icon0.to_string(),
            self.icon1.to_string(),
            self.icon2.to_string(),
            self.icon3.to_string(),
            self.icon4.to_string(),
            "0".to_string(),
            csharptime::datetime_to_csharp(self.timestamp).to_string(),
            self.game_version.clone(),
            encode(&self.short_desc).into_owned(),
        ];
        let header = format!("BLUEPRINT:{}", components.join(","));
        let hashed_data = format!("{},\"{}", header, b64_data);
        let hash_value = md5hash::DysonSphereMD5::new(md5hash::Variant::MD5F)
            .update(hashed_data.as_bytes())
            .finalize()
            .hexdigest()?;
        Ok(format!("{}\"{}", hashed_data, hash_value))
    }

    /// Returns a JSON representation of the blueprint.
    pub fn to_json(&self) -> Result<String, Box<dyn std::error::Error>> {
        use serde_json::json;
        let dict = json!({
            "icon": {
                "layout": self.layout,
                "images": [self.icon0, self.icon1, self.icon2, self.icon3, self.icon4],
            },
            "timestamp": self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string(),
            "game_version": self.game_version,
            "short_desc": self.short_desc,
            "data": serde_json::from_str::<data::BlueprintData>(&self.decoded_data()?.to_json())?
        });
        Ok(dict.to_string())
    }

    /// Reads a blueprint from a file.
    pub fn read_from_file<P: AsRef<Path>>(
        filename: P,
        validate_hash: bool,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string(filename)?;
        Blueprint::from_blueprint_string(&contents, validate_hash)
    }

    /// Writes the serialized blueprint string to a file.
    pub fn write_to_file<P: AsRef<Path>>(
        &self,
        filename: P,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let serialized = self.serialize()?;
        fs::write(filename, serialized)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, NaiveTime};

    fn dummy_bp_data() -> Vec<u8> {
        [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1,
            0, 255, 0, 0, 200, 0, 0, 0, 0, 0, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 1, 192, 25, 182, 0, 0, 0, 0, 0, 0, 0, 0, 1, 192, 25, 182, 0, 0, 0, 0, 0, 0, 0,
            0, 193, 11, 197, 1, 255, 255, 255, 255, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 110, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 77, 4, 0, 0, 77, 4, 0, 0, 77, 4, 0, 0, 77,
            4, 0, 0, 80, 4, 0, 0, 80, 4, 0, 0, 80, 4, 0, 0, 80, 4, 0, 0, 84, 4, 0, 0, 78, 4, 0, 0,
            81, 4, 0, 0, 81, 4, 0, 0, 82, 4, 0, 0, 82, 4, 0, 0, 85, 4, 0, 0, 85, 4, 0, 0, 79, 4, 0,
            0, 79, 4, 0, 0, 86, 4, 0, 0, 87, 4, 0, 0, 177, 4, 0, 0, 177, 4, 0, 0, 178, 4, 0, 0,
            178, 4, 0, 0, 21, 5, 0, 0, 21, 5, 0, 0, 127, 5, 0, 0, 127, 5, 0, 0, 86, 20, 0, 0, 86,
            20, 0, 0, 88, 4, 0, 0, 88, 4, 0, 0, 179, 4, 0, 0, 179, 4, 0, 0, 22, 5, 0, 0, 22, 5, 0,
            0, 121, 5, 0, 0, 121, 5, 0, 0, 91, 4, 0, 0, 91, 4, 0, 0, 104, 4, 0, 0, 104, 4, 0, 0,
            107, 4, 0, 0, 107, 4, 0, 0, 89, 4, 0, 0, 89, 4, 0, 0, 180, 4, 0, 0, 180, 4, 0, 0, 23,
            5, 0, 0, 23, 5, 0, 0, 124, 5, 0, 0, 124, 5, 0, 0, 96, 4, 0, 0, 96, 4, 0, 0, 99, 4, 0,
            0, 99, 4, 0, 0, 94, 4, 0, 0, 94, 4, 0, 0, 125, 5, 0, 0, 125, 5, 0, 0, 64, 156, 0, 0, 1,
            0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 237, 19, 0, 0,
            237, 19, 0, 0, 237, 19, 0, 0, 237, 19, 0, 0, 237, 19, 0, 0, 237, 19, 0, 0, 237, 19, 0,
            0, 237, 19, 0, 0, 237, 19, 0, 0, 237, 19, 0, 0, 237, 19, 0, 0, 237, 19, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
        .to_vec()
    }

    /// Helper function to create a dummy blueprint string via serialization.
    fn dummy_blueprint_string() -> String {
        // Create a Blueprint instance with dummy data.
        let bp = Blueprint::new(
            "1.0".to_string(),
            dummy_bp_data(),
            10, // layout
            0,  // icon0
            1,  // icon1
            2,  // icon2
            3,  // icon3
            4,  // icon4
            Some(
                NaiveDate::from_ymd_opt(2020, 1, 1)
                    .unwrap()
                    .and_hms_opt(12, 0, 0)
                    .unwrap(),
            ),
            "Short description".to_string(),
            "Long description".to_string(),
        );
        bp.serialize().unwrap()
    }

    #[test]
    fn test_parse_blueprint() {
        let bp_raw_data = "BLUEPRINT:0,23,3009,609,0,0,0,0,638391476082347356,0.10.28.21014,BAB%20%28Filtered%20for%20Level%209%29,\"H4sIAAAAAAAAC2NkQAWMUAxh/2dgOAFlMsKFEWoPSG7Dxj7IfZTxPxQgVOeBSU50Q7AAXxZUHICGQ4DYD4gDoTgIikOh2B+Kw4A4HIg3QvEmKBZlheB6KA4TgeAIFgjeDMVirBBcCcXRLBCcAcXZUBwJxVugWJwVgmugOIEFgpOhOA6Ka1kh2GEOItzR8Vth0jAtAAAlI45WJAIAAA==\"09DCE7720CA8695F93D0C611DD833255";
        let bp = Blueprint::from_blueprint_string(bp_raw_data, true).unwrap();

        assert_eq!(bp.game_version, "0.10.28.21014");
        assert_eq!(bp.layout, 23);
        assert_eq!(bp.icon0, 3009);
        assert_eq!(bp.icon1, 609);
        assert_eq!(bp.icon2, 0);
        assert_eq!(bp.icon3, 0);
        assert_eq!(bp.icon4, 0);
        assert_eq!(
            bp.timestamp,
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(2023, 12, 26).unwrap(),
                NaiveTime::from_hms_micro_opt(00, 33, 28, 234735).unwrap()
            )
        );
        assert_eq!(bp.short_desc, "BAB (Filtered for Level 9)");
        assert_eq!(bp.long_desc, "");
        assert_eq!(bp.data, dummy_bp_data(),);
    }

    #[test]
    fn test_serialization_and_deserialization() {
        let bp_string = dummy_blueprint_string();
        // Validate hash should succeed.
        let bp = Blueprint::from_blueprint_string(&bp_string, true).unwrap();
        assert_eq!(bp.layout, 10);
        assert_eq!(bp.icon1, 1);
        assert_eq!(bp.short_desc, "Short description");
    }

    #[test]
    fn test_to_dict() {
        let bp_string = dummy_blueprint_string();
        let bp = Blueprint::from_blueprint_string(&bp_string, true).unwrap();
        let dict = bp.to_json().unwrap();
        // Ensure JSON contains expected keys.
        assert!(dict.contains("\"icon\""));
        assert!(dict.contains("\"timestamp\""));
        assert!(dict.contains("\"game_version\""));
        assert!(dict.contains("\"short_desc\""));
        assert!(dict.contains("\"data\""));
    }

    #[test]
    fn test_invalid_hash() {
        let mut bp_string = dummy_blueprint_string();
        // Tamper with the blueprint string so that the hash becomes invalid.
        bp_string.push_str("tamper");
        let res = Blueprint::from_blueprint_string(&bp_string, true);
        assert!(res.is_err());
    }
}
