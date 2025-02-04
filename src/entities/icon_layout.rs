/*
 * Copyright (c) 2025. sakost aka Konstantin Sazhenov
 * All rights reserved.
 */

#![allow(dead_code)]

use crate::errors::UnknownDysonSphereIconLayout;

macro_rules! define_icon_enum {
    ($name:ident, $($variant:ident = $value:expr),* $(,)?) => {
        use ::strum::IntoStaticStr;


        #[repr(i32)]
        #[derive(Debug, PartialEq, Eq, Clone, Copy, IntoStaticStr)]
        pub enum $name {
            $($variant = $value),*
        }

        impl TryFrom<i32> for $name {
            type Error = UnknownDysonSphereIconLayout;

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $($value => Ok($name::$variant),)*
                    _ => Err(UnknownDysonSphereIconLayout(value)),
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

define_icon_enum!(
    IconLayout,
    None = 0,
    NoIcon = 1,
    OneIcon = 10,
    OneIconSmall = 11,
    TwoIcon46 = 20,
    TwoIcon53 = 21,
    TwoIcon59 = 22,
    TwoIcon57 = 23,
    TwoIcon51 = 24,
    ThreeIcon813 = 30,
    ThreeIcon279 = 31,
    ThreeIcon573 = 32,
    ThreeIcon591 = 33,
    FourIcon7913 = 40,
    FourIcon8462 = 41,
    FiveIcon57913 = 50,
    FiveIconPenta = 51,
);
