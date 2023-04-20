// numera::number::integer::prime::impl_traits
//
//!
//

use super::{is_prime_brute, Prime16, Prime32, Prime8};
use crate::{
    error::{NumeraError as Error, NumeraResult as Result},
    number::traits::{
        Bound, ConstLowerBounded, ConstUpperBounded, Count, Ident, LowerBounded, NonNegOne, NonOne,
        NonZero, Number, Sign, Unsigned, UpperBounded,
    },
};

/* Prime8 */

impl Bound for Prime8 {
    fn is_lower_bounded(&self) -> bool {
        true
    }
    fn is_upper_bounded(&self) -> bool {
        true
    }
    fn lower_bound(&self) -> Option<Self> {
        Some(Prime8(2))
    }
    fn upper_bound(&self) -> Option<Self> {
        Some(Prime8(251))
    }
}
impl LowerBounded for Prime8 {
    fn new_min() -> Self {
        Prime8(2)
    }
}
impl UpperBounded for Prime8 {
    fn new_max() -> Self {
        Prime8(251)
    }
}
impl ConstLowerBounded for Prime8 {
    const MIN: Self = Prime8(2);
}
impl ConstUpperBounded for Prime8 {
    const MAX: Self = Prime8(251);
}

impl Count for Prime8 {
    fn is_countable(&self) -> bool {
        true
    }
}
// impl Countable for Prime8 { // ←TODO
//     fn next(&self) -> bool { todo![] }
//     fn previous(&self) -> bool { todo![] }
// }

impl Ident for Prime8 {
    fn can_zero(&self) -> bool {
        false
    }
    fn can_one(&self) -> bool {
        false
    }
    fn can_neg_one(&self) -> bool {
        false
    }
    fn is_zero(&self) -> bool {
        false
    }
    fn is_one(&self) -> bool {
        false
    }
    fn is_neg_one(&self) -> bool {
        false
    }
}
impl NonZero for Prime8 {}
impl NonOne for Prime8 {}
impl NonNegOne for Prime8 {}

impl Sign for Prime8 {
    fn can_positive(&self) -> bool {
        true
    }
    fn can_negative(&self) -> bool {
        false
    }
    fn is_positive(&self) -> bool {
        true
    }
    fn is_negative(&self) -> bool {
        false
    }
}
impl Unsigned for Prime8 {}

impl Number for Prime8 {
    type Inner = u8;
    fn new(value: Self::Inner) -> Result<Self> {
        if is_prime_brute(value.into()) {
            Ok(Prime8(value))
        } else {
            Err(Error::Other("Not a prime."))
        }
    }
    unsafe fn new_unchecked(value: Self::Inner) -> Self {
        Self(value)
    }
    fn into_inner(self) -> Self::Inner {
        self.0
    }
    fn ref_inner(&self) -> &Self::Inner {
        &self.0
    }
}

/* Prime16 */

impl Bound for Prime16 {
    fn is_lower_bounded(&self) -> bool {
        true
    }
    fn is_upper_bounded(&self) -> bool {
        true
    }
    fn lower_bound(&self) -> Option<Self> {
        Some(Prime16(2))
    }
    fn upper_bound(&self) -> Option<Self> {
        Some(Prime16(65_521))
    }
}
impl LowerBounded for Prime16 {
    fn new_min() -> Self {
        Prime16(2)
    }
}
impl UpperBounded for Prime16 {
    fn new_max() -> Self {
        Prime16(65_512)
    }
}
impl ConstLowerBounded for Prime16 {
    const MIN: Self = Prime16(2);
}
impl ConstUpperBounded for Prime16 {
    const MAX: Self = Prime16(65_521);
}

impl Count for Prime16 {
    fn is_countable(&self) -> bool {
        true
    }
}

impl Ident for Prime16 {
    fn can_zero(&self) -> bool {
        false
    }
    fn can_one(&self) -> bool {
        false
    }
    fn can_neg_one(&self) -> bool {
        false
    }
    fn is_zero(&self) -> bool {
        false
    }
    fn is_one(&self) -> bool {
        false
    }
    fn is_neg_one(&self) -> bool {
        false
    }
}
impl NonZero for Prime16 {}
impl NonOne for Prime16 {}
impl NonNegOne for Prime16 {}

impl Sign for Prime16 {
    fn can_positive(&self) -> bool {
        true
    }
    fn can_negative(&self) -> bool {
        false
    }
    fn is_positive(&self) -> bool {
        true
    }
    fn is_negative(&self) -> bool {
        false
    }
}
impl Unsigned for Prime16 {}

impl Number for Prime16 {
    type Inner = u16;
    fn new(value: Self::Inner) -> Result<Self> {
        if is_prime_brute(value.into()) {
            Ok(Prime16(value))
        } else {
            Err(Error::Other("Not a prime."))
        }
    }
    unsafe fn new_unchecked(value: Self::Inner) -> Self {
        Self(value)
    }
    fn into_inner(self) -> Self::Inner {
        self.0
    }
    fn ref_inner(&self) -> &Self::Inner {
        &self.0
    }
}

/* Prime32 */

impl Bound for Prime32 {
    fn is_lower_bounded(&self) -> bool {
        true
    }
    fn is_upper_bounded(&self) -> bool {
        true
    }
    fn lower_bound(&self) -> Option<Self> {
        Some(Prime32(2))
    }
    fn upper_bound(&self) -> Option<Self> {
        Some(Prime32(4_294_967_279))
    }
}
impl LowerBounded for Prime32 {
    fn new_min() -> Self {
        Prime32(2)
    }
}
impl UpperBounded for Prime32 {
    fn new_max() -> Self {
        Prime32(4_294_967_291)
    }
}
impl ConstLowerBounded for Prime32 {
    const MIN: Self = Prime32(2);
}
impl ConstUpperBounded for Prime32 {
    const MAX: Self = Prime32(4_294_967_291);
}

impl Count for Prime32 {
    fn is_countable(&self) -> bool {
        true
    }
}

impl Ident for Prime32 {
    fn can_zero(&self) -> bool {
        false
    }
    fn can_one(&self) -> bool {
        false
    }
    fn can_neg_one(&self) -> bool {
        false
    }
    fn is_zero(&self) -> bool {
        false
    }
    fn is_one(&self) -> bool {
        false
    }
    fn is_neg_one(&self) -> bool {
        false
    }
}
impl NonZero for Prime32 {}
impl NonOne for Prime32 {}
impl NonNegOne for Prime32 {}

impl Sign for Prime32 {
    fn can_positive(&self) -> bool {
        true
    }
    fn can_negative(&self) -> bool {
        false
    }
    fn is_positive(&self) -> bool {
        true
    }
    fn is_negative(&self) -> bool {
        false
    }
}
impl Unsigned for Prime32 {}

impl Number for Prime32 {
    type Inner = u32;
    fn new(value: Self::Inner) -> Result<Self> {
        if is_prime_brute(value) {
            Ok(Prime32(value))
        } else {
            Err(Error::Other("Not a prime."))
        }
    }
    unsafe fn new_unchecked(value: Self::Inner) -> Self {
        Self(value)
    }
    fn into_inner(self) -> Self::Inner {
        self.0
    }
    fn ref_inner(&self) -> &Self::Inner {
        &self.0
    }
}
