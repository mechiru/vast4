#![allow(deprecated)]
#![allow(clippy::deprecated_semver, clippy::needless_late_init)]

//! An implementation of [`VAST 4.2 FINAL`].
//!
//! [`VAST 4.2 FINAL`]: https://iabtechlab.com/wp-content/uploads/2019/06/VAST_4.2_final_june26.pdf

mod macros;

// functions

pub fn to_string<T: hard_xml::XmlWrite>(value: &T) -> hard_xml::XmlResult<String> {
    T::to_string(value)
}

pub fn from_str<'a, T: hard_xml::XmlRead<'a>>(s: &'a str) -> hard_xml::XmlResult<T> {
    T::from_str(s)
}

// common types
mod duration;
pub use duration::Duration;

mod error;
pub use error::*;

// 3.2
mod vast;
pub use vast::*;

// 3.3
mod ad;
pub use ad::*;

// 3.4
mod in_line;
pub use in_line::*;

// 3.5
mod viewable;
pub use viewable::*;

// 3,6, 3.7
mod creative;
pub use creative::*;

// 3.8
mod linear;
pub use linear::*;

// 3.9
mod media_file;
pub use media_file::*;

// 3.10
mod click;
pub use click::*;

// 3.11
mod icon;
pub use icon::*;

// 3.12
mod non_linear;
pub use non_linear::*;

// 3.13
mod companion_ad;
pub use companion_ad::*;

// 3.14
mod tracking;
pub use tracking::*;

// 3.15
mod creative_resource;
pub use creative_resource::*;

// 3.16, 3.17
mod verification;
pub use verification::*;

// 3.18
mod extension;
pub use extension::*;

// 3.19
mod wrapper;
pub use wrapper::*;
