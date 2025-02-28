#[cfg(feature = "scalar_field")]
pub mod fr;
#[cfg(feature = "scalar_field")]
pub use self::fr::*;

#[cfg(feature = "curve")]
pub mod fq;
#[cfg(feature = "curve")]
pub use self::fq::*;

#[cfg(feature = "curve")]
pub mod fq3;
#[cfg(feature = "curve")]
pub use self::fq3::*;
#[cfg(feature = "curve")]
pub mod fq6;
#[cfg(feature = "curve")]
pub use self::fq6::*;

#[cfg(all(feature = "curve", test))]
mod tests;
