#![no_std]

mod contract;
mod storage;
mod test;
mod types;

#[cfg(test)]
pub use crate::contract::TokenContract;
