//! Result storage
use spl_math::precise_number::PreciseNumber;

#[derive(Debug)]
/// Encodes all results of swapping from a source token to a destination token
pub struct SwapResult {
    /// Invariant expressed as k
    pub k: PreciseNumber,
    /// New source amount expressed as x_new
    pub x_new: PreciseNumber,
    /// New destination amount expressed as y_new
    pub y_new: PreciseNumber,
    /// Amount of source token swapped expressed as delta_x
    pub delta_x: PreciseNumber,
    /// Amount of destination token swapped expressed as delta_x
    pub delta_y: PreciseNumber,
}