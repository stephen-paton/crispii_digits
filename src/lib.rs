//! Super simple, no frills, digit enum types
//! 
//! (a low-level dependency of crispii, though the types may offer value as standalone imports in your own projects as well)
//! 
//! The intention is for these to be used as safe building blocks for function/method args, really for two key reasons:
//! 1. It makes it far more explicit to the consumer of your library, what type you expect them to use e.g. if a colour channel arg is to represent 0x00 to 0xff, technically a u8 could be used, but there's no way to enforce that hex notation 0x is used
//! 2. To eschew the need for or to simplify input validation e.g. a single hex digit would technically be represented by a u4, but no such type exists in vanilla Rust, so assuming you ever did have a situation where you only wanted to represent a
//! single hex digit, you'd need to ensure that the 4x leftmost bits of your u8 were set to zero before use, whereas using the Hex type, there's no such need
//! 
//! # Examples
//! ```
//! use crispii_digits::Hex;
//! 
//! fn pass_single_hex_digit(digit: Hex) {
//!     println!("I can safely convert 'digit' to '{}' without fear of it being an illegal value!", u8::from(digit));
//! }
//! ``` 
mod bin;
pub use bin::Bin;

mod dec;
pub use dec::Dec;

mod hex;
pub use hex::Hex;
