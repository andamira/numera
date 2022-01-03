///! utility macros

/// Implements Overloadable operators traits on a newtype.
///
/// Just the following arithmetic operators:
///
/// Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign,
/// RemAssign
///
#[macro_export]
#[doc(hidden)]
macro_rules! impl_ops [
    // implements the arithmetic operators.
    ($inner:ident, $tname:tt, $type:ty, $rhs:ty, $($bound:tt)* ) => {
        crate::impl_ops![op_refs; Add, add, $inner, $tname, $type, $rhs, $($bound)* ];
        crate::impl_ops![op_refs; Sub, sub, $inner, $tname, $type, $rhs, $($bound)* ];
        crate::impl_ops![op_refs; Mul, mul, $inner, $tname, $type, $rhs, $($bound)* ];
        crate::impl_ops![op_refs; Div, div, $inner, $tname, $type, $rhs, $($bound)* ];
        crate::impl_ops![op_refs; Rem, rem, $inner, $tname, $type, $rhs, $($bound)* ];
        // crate::impl_ops![op_refs; BitAnd, bitand, $inner, $tname, $type, $rhs, $($bound)* ];
        // crate::impl_ops![op_refs; BitOr, bitor, $inner, $tname, $type, $rhs, $($bound)* ];
        // crate::impl_ops![op_refs; BitXor, bitxor, $inner, $tname, $type, $rhs, $($bound)* ];
        // crate::impl_ops![op_refs; Shl, shl, $inner, $tname, $type, $rhs, $($bound)* ];
        // crate::impl_ops![op_refs; Shr, shr, $inner, $tname, $type, $rhs, $($bound)* ];

        crate::impl_ops![op_refs_a; AddAssign, add_assign, $inner, $tname, $type, $rhs, $($bound)* ];
        crate::impl_ops![op_refs_a; SubAssign, sub_assign, $inner, $tname, $type, $rhs, $($bound)* ];
        crate::impl_ops![op_refs_a; MulAssign, mul_assign, $inner, $tname, $type, $rhs, $($bound)* ];
        crate::impl_ops![op_refs_a; DivAssign, div_assign, $inner, $tname, $type, $rhs, $($bound)* ];
        crate::impl_ops![op_refs_a; RemAssign, rem_assign, $inner, $tname, $type, $rhs, $($bound)* ];
        // crate::impl_ops![op_refs_a; BitAndAssign, bitand_assign, $inner, $tname, $type, $rhs, $($bound)* ];
        // crate::impl_ops![op_refs_a; BitOrAssign, bitor_assign, $inner, $tname, $type, $rhs, $($bound)* ];
        // crate::impl_ops![op_refs_a; BitXorAssign, bitxor_assign, $inner, $tname, $type, $rhs, $($bound)* ];
        // crate::impl_ops![op_refs_a; ShlAssign, shl_assign, $inner, $tname, $type, $rhs, $($bound)* ];
        // crate::impl_ops![op_refs_a; ShrAssign, shr_assign, $inner, $tname, $type, $rhs, $($bound)* ];
    };

    // implements all the variants of a single operator. (non-Assign version)
    (op_refs; $op:tt, $fn:ident, $inner:ident, $tname:tt, $type:ty, $rhs:ty, $($bound:tt)* ) => {
        crate::impl_ops![op; $op, $fn, $inner, $tname, $type, $rhs, $($bound)* ];
        crate::impl_ops![op; $op, $fn, $inner, $tname, $type, &'b $rhs, $($bound)* ];
        crate::impl_ops![op; $op, $fn, $inner, $tname, $type, &'b mut $rhs, $($bound)* ];
        crate::impl_ops![op; $op, $fn, $inner, $tname, &'a $type, $rhs, $($bound)* ];
        crate::impl_ops![op; $op, $fn, $inner, $tname, &'a $type, &'b $rhs, $($bound)* ];
        crate::impl_ops![op; $op, $fn, $inner, $tname, &'a $type, &'b mut $rhs, $($bound)* ];
        crate::impl_ops![op; $op, $fn, $inner, $tname, &'a mut $type, $rhs, $($bound)* ];
        crate::impl_ops![op; $op, $fn, $inner, $tname, &'a mut $type, &'b $rhs, $($bound)* ];
        crate::impl_ops![op; $op, $fn, $inner, $tname, &'a mut $type, &'b mut $rhs, $($bound)* ];
    };

    // implements all the variants of a single operator. (Assign version)
    (op_refs_a; $op:tt, $fn:ident, $inner:ident, $tname:tt, $type:ty, $rhs:ty, $($bound:tt)* ) => {
        crate::impl_ops![op_a; $op, $fn, $inner, $tname, $type, $rhs, $($bound)* ];
        crate::impl_ops![op_a; $op, $fn, $inner, $tname, $type, &'b $rhs, $($bound)* ];
        crate::impl_ops![op_a; $op, $fn, $inner, $tname, $type, &'b mut $rhs, $($bound)* ];
        crate::impl_ops![op_a; $op, $fn, $inner, $tname, &'a mut $type, $rhs, $($bound)* ];
        crate::impl_ops![op_a; $op, $fn, $inner, $tname, &'a mut $type, &'b $rhs, $($bound)* ];
        crate::impl_ops![op_a; $op, $fn, $inner, $tname, &'a mut $type, &'b mut $rhs, $($bound)* ];
    };

    // implements a single operator. (non-Assign version)
    //
    // # Supports
    //
    // Add, Sub, Mul, Div, BitAnd, BitOr, BitXor, Shl, Shr, Rem
    //
    // # Arguments
    //
    // - $op:    the operator trait
    // - $fn:    the operator function
    // - $inner: the inner type
    // - $tname: the name of the type of the implementation, must be owned.
    // - $type:  the main type for the implementation, can be a reference.
    // - $rhs:   the other type needed for the operation, can be a reference.
    // - $bound: the trait bounds (not including the current operator).
    //
    (op; $op:tt, $fn: ident, $inner:ident, $tname:tt, $type:ty, $rhs:ty, $($bound:tt)* ) => {
        impl<'a, 'b, $inner> core::ops::$op<$rhs> for $type
        where $inner: $($bound)* + core::ops::$op<Output = $inner>
        {
            type Output = $tname<$inner>;
            fn $fn(self, rhs: $rhs) -> Self::Output {
                $tname(self.0.clone().$fn(rhs.0.clone()))
            }
        }
    };

    // implements a single operator. (Assign version)
    //
    // # Supports
    //
    // AddAssign, SubAssign, MulAssign, DivAssign, BitAndAssign,
    // BitOrAssign, BitXorAssign, ShlAssign, ShrAssign, RemAssign
    //
    // # Arguments
    //
    // the same as `op;`
    //
    (op_a; $op:tt, $fn: ident, $inner:ident, $tname:tt, $type:ty, $rhs:ty, $($bound:tt)* ) => {
        impl<'a, 'b, $inner> core::ops::$op<$rhs> for $type
        where $inner: $($bound)* + core::ops::$op
        {
            fn $fn(&mut self, rhs: $rhs) {
                self.0.$fn(rhs.0.clone())
            }
        }
    };

];
