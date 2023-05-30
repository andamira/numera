// numera::number::integer::z::ops::sqrt
//
//! Implement the substraction operations.
//

use crate::number::{integer::*, traits::Sign};
use devela::paste;

macro_rules! impl_integer_sub {
    // impl square root ops for multiple integer types
    //
    // # Args
    // $a: integer base abbreviation. e.g. Z
    // $t: integer base name. e.g. Integer
    // $p: inner primitive base name. e.g. i
    // $b: integer and primitive bitsize. e.g. 8
    ( $($a:ident | $t:ident + $p:ident + $b:literal, cast: $bcast:literal);+ ) => {
        $(
            impl_integer_sub![sqrt: $a | $t + $p + $b];
        )+
    };

    // substraction operations
    //
    // impl variants:
    // - sqrt_floor
    // - sqrt_ceil
    // - sqrt_round
    (sqrt: $a: ident | $t:ident + $p:ident + $b:literal) => { paste! {
        /// # Square root
        impl [<$t$b>] {
            /// Returns `true` if self is a perfect square,
            /// meaning the square root of self is an integer.
            ///
            /// Returns `false` otherwise, which includes all negative values.
            ///
            /// # Algorithm
            /// $$
            /// \text{is\textunderscore square}(n) = \begin{cases}
            /// \text{true} & \text{if } \left(\lfloor \sqrt{n} \rfloor\right)^2 = n \cr
            /// \text{false} & \text{if } \left(\lfloor \sqrt{n} \rfloor\right)^2 \neq n
            /// \end{cases}
            /// $$
            ///
            /// # Examples
            /// ```
            #[doc="use numera::all::" [<$t$b>] ";"]
            ///
            #[doc="assert_eq!["[<$t$b>]"(12).is_square(), false];"]
            #[doc="assert_eq!["[<$t$b>]"(13).is_square(), false];"]
            #[doc="assert_eq!["[<$t$b>]"(16).is_square(), true];"]
            #[doc="assert_eq!["[<$t$b>]"(20).is_square(), false];"]
            #[doc="assert_eq!["[<$t$b>]"(21).is_square(), false];"]
            /// ```
            #[inline]
            #[must_use]
            pub fn is_square(self) -> bool {
                if let Some(sqrt) = self.sqrt_floor() {
                    sqrt * sqrt == self
                } else {
                    false
                }
            }

            /// Returns the floored integer square root.
            ///
            /// Returns `None` if self is negative.
            ///
            /// # Algorithm
            /// $$ \left\lfloor \sqrt{x} \right\rfloor = n_{k} $$
            ///
            /// Where $n_{k}$ is the result of a sequence of estimates that
            /// starts with an initial $n_{0} = x/2$ which is updated using
            /// [*Heron's method*](
            /// https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Heron's_method):
            ///
            /// $$
            /// n_{i+1} = n_{i} - ( n_{i}^{2} - x) / 2n_{i},
            /// \quad \small\text{for} \quad i = 0, 1, \ldots, k,
            /// $$
            ///
            /// Where $n_{i}$ is the current estimate, $n_{i+1}$ is the next
            /// estimate, $x$ is self, and $k$ is the number of iterations
            /// needed to converge to a solution, on the order of the number of
            /// bits of self, about $O(\log_2 b)$, which for e.g. 128 bits would
            /// be $ ±7 $ iterations.
            ///
            /// Hence, the function continues updating the estimate until
            /// reaching $n_{k}$, which provides the largest integer less than
            /// or equal to the square root of `x`.
            ///
            /// # Examples
            /// ```
            #[doc="use numera::all::" [<$t$b>] ";"]
            ///
            #[doc="assert_eq!["[<$t$b>]"(12).sqrt_floor(), Some("[<$t$b>]"(3))];"]
            #[doc="assert_eq!["[<$t$b>]"(13).sqrt_floor(), Some("[<$t$b>]"(3))];"]
            #[doc="assert_eq!["[<$t$b>]"(16).sqrt_floor(), Some("[<$t$b>]"(4))];"]
            #[doc="assert_eq!["[<$t$b>]"(20).sqrt_floor(), Some("[<$t$b>]"(4))];"]
            #[doc="assert_eq!["[<$t$b>]"(21).sqrt_floor(), Some("[<$t$b>]"(4))];"]
            /// ```
            #[inline]
            #[must_use]
            pub fn sqrt_floor(self) -> Option<[<$t$b>]> {
                const _2: [<$t$b>] = [<$t$b>](2);

                if self.is_negative() {
                    None
                } else if self < _2 {
                    Some(self)
                } else {
                    let mut x = self;
                    let mut y = (x + self / x) / _2;
                    while y < x {
                        x = y;
                        y = (x + self / x) / _2;
                    }
                    Some(x)
                }
            }

            /// Returns the ceiled integer square root.
            ///
            /// Returns `None` if self is negative.
            ///
            /// # Algorithm
            /// $$
            /// \begin{align}
            /// \notag \left\lceil \sqrt{x} \thinspace\right\rceil = \begin{cases}
            /// n & \text{if } n^2 = x \cr
            /// n+1 & \text{if } n^2 < x \end{cases} \cr
            /// \notag \small\text{where } n = \lfloor \sqrt{x} \rfloor &
            /// \end{align}
            /// $$
            ///
            /// # Examples
            /// ```
            #[doc="use numera::all::" [<$t$b>] ";"]
            ///
            #[doc="assert_eq!["[<$t$b>]"(12).sqrt_ceil(), Some("[<$t$b>]"(4))];"]
            #[doc="assert_eq!["[<$t$b>]"(13).sqrt_ceil(), Some("[<$t$b>]"(4))];"]
            #[doc="assert_eq!["[<$t$b>]"(16).sqrt_ceil(), Some("[<$t$b>]"(4))];"]
            #[doc="assert_eq!["[<$t$b>]"(20).sqrt_ceil(), Some("[<$t$b>]"(5))];"]
            #[doc="assert_eq!["[<$t$b>]"(21).sqrt_ceil(), Some("[<$t$b>]"(5))];"]
            /// ```
            #[inline]
            #[must_use]
            pub fn sqrt_ceil(self) -> Option<[<$t$b>]> {
                const _1: [<$t$b>] = [<$t$b>](1);

                if let Some(floor) = self.sqrt_floor() {
                    if floor * floor == self {
                        Some(floor)
                    } else {
                        Some(floor + _1)
                    }
                } else {
                    None
                }
            }

            /// Returns the rounded integer square root.
            ///
            /// Returns `None` if self is negative.
            ///
            /// # Algorithm
            /// $$
            /// \begin{align}
            /// \notag \left\lfloor\sqrt{x} \thinspace\right\rceil = \begin{cases}
            /// n & \text{if } x - n^2 < (n+1)^2 - x \cr
            /// n+1 & \text{if } x - n^2 \geq (n+1)^2 - x \end{cases} \cr
            /// \notag \small\text{where } n = \lfloor \sqrt{x} \rfloor &
            /// \end{align}
            /// $$
            ///
            /// [`sqrt_floor`]: #method.sqrt_floor
            ///
            /// # Examples
            /// ```
            #[doc="use numera::all::" [<$t$b>] ";"]
            ///
            #[doc="assert_eq!["[<$t$b>]"(12).sqrt_round(), Some("[<$t$b>]"(3))];"]
            #[doc="assert_eq!["[<$t$b>]"(13).sqrt_round(), Some("[<$t$b>]"(4))];"]
            #[doc="assert_eq!["[<$t$b>]"(16).sqrt_round(), Some("[<$t$b>]"(4))];"]
            #[doc="assert_eq!["[<$t$b>]"(20).sqrt_round(), Some("[<$t$b>]"(4))];"]
            #[doc="assert_eq!["[<$t$b>]"(21).sqrt_round(), Some("[<$t$b>]"(5))];"]
            /// ```
            #[inline]
            #[must_use]
            pub fn sqrt_round(self) -> Option<[<$t$b>]> {
                const _1: [<$t$b>] = [<$t$b>](1);
                const _2: [<$t$b>] = [<$t$b>](2);

                if self.is_negative() {
                    None
                } else if self < _2 {
                    Some(self)
                } else {
                    // sqrt_floor
                    let mut x = self;
                    let mut y = (x + self / x) / _2;
                    while y < x {
                        x = y;
                        y = (x + self / x) / _2;
                    }

                    // do we have to round up?
                    if self - x * x >= (x + _1) * (x + _1) - self {
                        Some(x + _1)
                    } else {
                        Some(x)
                    }
                }
            }

            /// Returns the checked floored integer square root, as a tuple containing
            /// [`sqrt_floor`](#method.is_floor) and [`is_square`](#method.is_square).
            ///
            /// Returns `None` if self is negative.
            ///
            /// # Examples
            /// ```
            #[doc="use numera::all::" [<$t$b>] ";"]
            ///
            #[doc="assert_eq!["[<$t$b>]"(12).checked_sqrt_floor(), Some(("[<$t$b>]"(3), false))];"]
            #[doc="assert_eq!["[<$t$b>]"(13).checked_sqrt_floor(), Some(("[<$t$b>]"(3), false))];"]
            #[doc="assert_eq!["[<$t$b>]"(16).checked_sqrt_floor(), Some(("[<$t$b>]"(4), true))];"]
            #[doc="assert_eq!["[<$t$b>]"(20).checked_sqrt_floor(), Some(("[<$t$b>]"(4), false))];"]
            #[doc="assert_eq!["[<$t$b>]"(21).checked_sqrt_floor(), Some(("[<$t$b>]"(4), false))];"]
            /// ```
            #[inline]
            #[must_use]
            pub fn checked_sqrt_floor(self) -> Option<([<$t$b>], bool)> {
                const _2: [<$t$b>] = [<$t$b>](2);

                if self.is_negative() {
                    None
                } else if self < _2 {
                    Some((self, true))
                } else {
                    let mut x = self;
                    let mut y = (x + self / x) / _2;
                    while y < x {
                        x = y;
                        y = (x + self / x) / _2;
                    }
                    Some((x, x * x == self))
                }
            }

            /// Returns the checked ceiled integer square root, as a tuple containing
            /// [`sqrt_ceil`](#method.is_ceil) and [`is_square`](#method.is_square).
            ///
            /// Returns `None` if self is negative.
            ///
            /// # Examples
            /// ```
            #[doc="use numera::all::" [<$t$b>] ";"]
            ///
            #[doc="assert_eq!["[<$t$b>]"(12).checked_sqrt_ceil(), Some(("[<$t$b>]"(4), false))];"]
            #[doc="assert_eq!["[<$t$b>]"(13).checked_sqrt_ceil(), Some(("[<$t$b>]"(4), false))];"]
            #[doc="assert_eq!["[<$t$b>]"(16).checked_sqrt_ceil(), Some(("[<$t$b>]"(4), true))];"]
            #[doc="assert_eq!["[<$t$b>]"(20).checked_sqrt_ceil(), Some(("[<$t$b>]"(5), false))];"]
            #[doc="assert_eq!["[<$t$b>]"(21).checked_sqrt_ceil(), Some(("[<$t$b>]"(5), false))];"]
            /// ```
            #[inline]
            #[must_use]
            pub fn checked_sqrt_ceil(self) -> Option<([<$t$b>], bool)> {
                const _1: [<$t$b>] = [<$t$b>](1);

                if let Some(floor) = self.sqrt_floor() {
                    if floor * floor == self {
                        Some((floor, true))
                    } else {
                        Some((floor + _1, false))
                    }
                } else {
                    None
                }
            }

            /// Returns the checked rounded integer square root, as a tuple containing
            /// [`sqrt_round`](#method.sqrt_round) and [`is_square`](#method.is_square).
            ///
            /// Returns `None` if self is negative.
            ///
            /// # Examples
            /// ```
            #[doc="use numera::all::" [<$t$b>] ";"]
            ///
            #[doc="assert_eq!["[<$t$b>]"(12).checked_sqrt_round(), Some(("[<$t$b>]"(3), false))];"]
            #[doc="assert_eq!["[<$t$b>]"(13).checked_sqrt_round(), Some(("[<$t$b>]"(4), false))];"]
            #[doc="assert_eq!["[<$t$b>]"(16).checked_sqrt_round(), Some(("[<$t$b>]"(4), true))];"]
            #[doc="assert_eq!["[<$t$b>]"(20).checked_sqrt_round(), Some(("[<$t$b>]"(4), false))];"]
            #[doc="assert_eq!["[<$t$b>]"(21).checked_sqrt_round(), Some(("[<$t$b>]"(5), false))];"]
            /// ```
            #[inline]
            #[must_use]
            pub fn checked_sqrt_round(self) -> Option<([<$t$b>], bool)> {
                const _1: [<$t$b>] = [<$t$b>](1);
                const _2: [<$t$b>] = [<$t$b>](2);

                if self.is_negative() {
                    None
                } else if self < _2 {
                    Some((self, true))
                } else {
                    let mut x = self;
                    let mut y = (x + self / x) / _2;
                    while y < x {
                        x = y;
                        y = (x + self / x) / _2;
                    }
                    let square = x * x;
                    if self - square >= (x + _1) * (x + _1) - self {
                        Some((x + _1, false))
                    } else {
                        Some((x, square == self))
                    }
                }
            }
        }
    }};
}

impl_integer_sub![
    Z|Integer+i+8, cast:16;
    Z|Integer+i+16, cast:32;
    Z|Integer+i+32, cast:64;
    Z|Integer+i+64, cast:128;
    Z|Integer+i+128, cast:128
];
