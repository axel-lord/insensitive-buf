#![cfg_attr(not(feature = "std"), no_std)]
#![doc = include_str!("../README.md")]

pub use self::{insensitive_display::InsensitiveDisplay, insensitive_ref::Insensitive};

mod insensitive_ref;

#[cfg(feature = "alloc")]
mod insensitive_buf;

#[cfg(feature = "alloc")]
pub use self::insensitive_buf::InsensitiveBuf;

mod insensitive_display;

pub mod insensitive;
