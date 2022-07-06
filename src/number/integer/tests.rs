// numera::number::integer::tests
//
//!
//

use super::{Integer, Negative, NonNegative, NonPositive, Positive};
use crate::number::Number;

#[test]
fn number_api() {
    // new

    assert_eq![Negative::new(23), Negative::new(-1)];
    assert_eq![NonPositive::new(23), NonPositive::new(0)];
    assert_eq![NonNegative::new(-23), NonNegative::new(0)];
    assert_eq![Positive::new(-23), Positive::new(1)];

    // scope

    assert_eq![Integer::new(i8::MIN), Integer::<i8>::new_min()];
    assert_eq![Integer::new(i8::MAX), Integer::<i8>::new_max()];

    assert_eq![Negative::new(i8::MIN), Negative::<i8>::new_min()];
    assert_eq![Negative::new(-1), Negative::<i8>::new_max()];

    assert_eq![NonPositive::new(i8::MIN), NonPositive::<i8>::new_min()];
    assert_eq![NonPositive::new(0), NonPositive::<i8>::new_max()];

    assert_eq![Positive::new(1), Positive::<i8>::new_min()];
    assert_eq![Positive::new(i8::MAX), Positive::<i8>::new_max()];
    assert_eq![Positive::new(1), Positive::<u8>::new_min()];
    assert_eq![Positive::new(u8::MAX), Positive::<u8>::new_max()];

    assert_eq![NonNegative::new(0), NonNegative::<i8>::new_min()];
    assert_eq![NonNegative::new(i8::MAX), NonNegative::<i8>::new_max()];
    assert_eq![NonNegative::new(0), NonNegative::<u8>::new_min()];
    assert_eq![NonNegative::new(u8::MAX), NonNegative::<u8>::new_max()];

    //
}
