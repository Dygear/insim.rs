use deku::prelude::*;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::protocol::identifiers::RequestId;

#[derive(Debug, DekuRead, DekuWrite, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[deku(
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Little",
    endian = "endian"
)]
/// Send Single Character
pub struct Sch {
    #[deku(pad_bytes_after = "1")]
    pub reqi: RequestId,

    pub charb: u8,

    #[deku(pad_bytes_after = "2")]
    pub flags: u8, // bit 0: SHIFT / bit 1: CTRL
}