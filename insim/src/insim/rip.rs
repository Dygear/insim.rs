use std::time::Duration;

use insim_core::{
    identifiers::RequestId, binrw::{binrw, self},
    duration::{binrw_parse_u32_duration, binrw_write_u32_duration},
    string::{binrw_parse_codepage_string, binrw_write_codepage_string}
};

#[cfg(feature = "serde")]
use serde::Serialize;

#[binrw]
#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[repr(u8)]
#[brw(repr(u8))]
#[non_exhaustive]
pub enum RipError {
    #[default]
    Ok = 0,

    Already = 1,

    Dedicated = 2,

    WrongMode = 3,

    NotReplay = 4,

    Corrupted = 5,

    NotFound = 6,

    Unloadable = 7,

    DestOOB = 8,

    Unknown = 9,

    User = 10,

    OOS = 11,
}

bitflags::bitflags! {
    /// Bitwise flags used within the [Sta] packet
    #[binrw]
    #[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy, Default)]
    #[cfg_attr(feature = "serde", derive(Serialize))]
    #[br(map = Self::from_bits_truncate)]
    #[bw(map = |&x: &Self| x.bits())]
    pub struct RipOptions: u8 {
        /// Replay will loop
        const LOOP = (1 << 0);

        /// Download missing skins
        const SKINS = (1 << 1);

        /// Use full physics
        const FULL_PHYS = (1 << 2);
    }
}

#[binrw]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
/// Replay Information
pub struct Rip {
    pub reqi: RequestId,
    pub error: RipError,

    // FIXME should be bool
    pub mpr: u8,
    // FIXME should be bool
    pub paused: u8,
    #[brw(pad_after = 1)]
    pub options: RipOptions,

    #[br(parse_with = binrw_parse_u32_duration::<_>)]
    #[bw(write_with = binrw_write_u32_duration::<_>)]
    pub ctime: Duration,

    #[br(parse_with = binrw_parse_u32_duration::<_>)]
    #[bw(write_with = binrw_write_u32_duration::<_>)]
    pub ttime: Duration,

    #[br(parse_with = binrw_parse_codepage_string::<64, _>)]
    #[bw(write_with = binrw_write_codepage_string::<64, _>)]
    pub rname: String,
}
