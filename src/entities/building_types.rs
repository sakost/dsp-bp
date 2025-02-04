/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

#![allow(dead_code)]

use crate::errors::UnknownDysonSphereBuildingType;

macro_rules! define_building_type_enum {
    ($name:ident, $($variant:ident = $value:expr),* $(,)?) => {
        use ::strum::IntoStaticStr;


        #[repr(i32)]
        #[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
        pub enum $name {
            $($variant = $value),*
        }

        impl TryFrom<i32> for $name {
            type Error = UnknownDysonSphereBuildingType;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok($name::$variant),)*
                    _ => Err(UnknownDysonSphereBuildingType(value)),
                }
            }
        }
        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                match self {
                    $($name::$variant => write!(f, "{}", stringify!($variant)),)*
                }
            }
        }
    }
}

define_building_type_enum!(
    BuildingType,
    None = 0,
    Miner = 1,
    Splitter = 2,
    Storage = 3,
    Tank = 4,
    Assembler = 5,
    Inserter = 6,
    Ejector = 7,
    Lab = 8,
    Station = 9,
    Dispenser = 10,
    Turret = 11,
    Gamma = 12,
    Exchanger = 13,
    Belt = 14,
    Monitor = 15,
    Silo = 16,
    ArtifacialStar = 17,
    BattleBase = 18,
    Geothermal = 19,
    Marker = 20,
    Other = 99,
);
