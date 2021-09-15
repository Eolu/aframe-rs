#![doc = include_str!("../README.md")]

#[cfg(test)]
mod tests;

pub mod sys;
pub mod shader;
pub mod component;
pub mod utils;
pub use utils::*;
pub mod entity;
pub use entity::*;
