
pub mod without_core {
    pub use ::fmt;
    pub use ::mem;
    pub use ::utility;
    pub use ::core::{
        nonzero,
        ptr,
        marker,
        // mem is already imported
        // fmt is already imported
    };

    pub use ::fmt::Write;  // for println!(...)
    //pub use ::fmt::*;  // for println!(...)
    pub use ::core::nonzero::NonZero;
    pub use ::core::ptr::Shared;
    pub use ::core::mem::{ size_of, size_of_val };
}

pub use self::without_core::*;
#[allow(private_in_public)]
pub use core;
