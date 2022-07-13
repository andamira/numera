// numera::integer::impl_ops
//
//! implements the arithmetic operations for all integer types.
//!
//! - core ops are only implemented when self and output have the same type.
// - try ops can fail but support more different types of ops.
//

/* core ops */

mod add;
mod div;
mod mul; // TODO
mod neg; // REVIEW
mod rem; // TODO
mod sub; // TODO

// mod try;
