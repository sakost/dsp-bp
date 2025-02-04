/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

use serde::Serializer;

#[derive(Debug)]
pub struct InvalidDataCount(pub usize);

impl std::fmt::Display for InvalidDataCount {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.serialize_newtype_struct("invalid_data_count", &self.0)
    }
}

impl std::error::Error for InvalidDataCount {}

#[derive(Debug)]
pub struct CorruptedData;

impl std::fmt::Display for CorruptedData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.serialize_unit_struct("corrupted_data")
    }
}

impl std::error::Error for CorruptedData {}

#[derive(Debug)]
pub struct UnknownDysonSphereItem(pub i32);

impl std::fmt::Display for UnknownDysonSphereItem {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.serialize_newtype_struct("unknown_dyson_sphere_item", &self.0)
    }
}

impl std::error::Error for UnknownDysonSphereItem {}

#[derive(Debug)]
pub struct UnknownDysonSphereIconLayout(pub i32);

impl std::fmt::Display for UnknownDysonSphereIconLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.serialize_newtype_struct("unknown_dyson_sphere_icon_layout", &self.0)
    }
}

impl std::error::Error for UnknownDysonSphereIconLayout {}

#[derive(Debug)]
pub struct UnknownDysonSphereBuildingType(pub i32);

impl std::fmt::Display for UnknownDysonSphereBuildingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.serialize_newtype_struct("unknown_dyson_sphere_building_type", &self.0)
    }
}

impl std::error::Error for UnknownDysonSphereBuildingType {}

// todo: maybe should be macro for all of these errors...
