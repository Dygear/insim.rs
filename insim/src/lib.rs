#![doc = include_str!("../README.md")]

pub mod codec;
pub mod error;
pub mod v9;
pub mod result;
pub mod network;
pub mod connection;

#[doc(hidden)]
pub mod macros;

pub mod relay;

// pub mod tools;

#[doc(hidden)]
/// Rexport insim_core
pub use insim_core as core;

#[cfg(feature = "game_data")]
/// Report insim_game_data when game_data feature is enabled
pub use insim_game_data as game_data;

#[cfg(feature = "pth")]
/// Report insim_pth when pth feature is enabled
pub use insim_pth as pth;

#[cfg(feature = "smx")]
/// Report insim_smx when smx feature is enabled
pub use insim_smx as smx;
