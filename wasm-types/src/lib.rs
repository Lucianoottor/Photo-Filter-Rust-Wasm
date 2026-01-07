#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::inline_always)]



use parity_scale_codec::{Decode, Encode};


#[derive(Debug, Clone, Encode, Decode, PartialEq, Eq)]
pub struct Picture {
    pub width: u32,
    pub height: u32,
    pub data_ptr: u32,
}