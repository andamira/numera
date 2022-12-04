// numera::integer::z::impl_integer
//
//!
//
// TOC
//
// - macro
// - impls

use super::{Integer128, Integer16, Integer32, Integer64, Integer8};
use crate::number::integer::Integer;

macro_rules! impl_integer {
    (many: $($t:ident),+) => {
        $( impl_integer![$t]; )+
    };
    ($t:ident) => {
        impl Integer for $t {
            fn is_even(&self) -> bool {
                self.0.is_even()
            }
            fn is_multiple_of(&self, other: &Self) -> bool {
                self.0.is_multiple_of(&other.0)
            }
        }
    };
}

impl_integer![many: Integer8, Integer16, Integer32, Integer64, Integer128];
