mod backend;
mod error;

pub use einops_macros::einops;

pub use backend::Backend;
pub use error::EinopsError;

/// Specifies the operation used to reduce an axis
#[derive(Copy, Clone, Debug)]
pub enum Operation {
    /// Take the minimum value
    Min,
    /// Take the maximum value
    Max,
    /// Add all elements
    Sum,
    /// Take the average
    Mean,
    /// Multiply all elements
    Prod,
}
