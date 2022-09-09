use deku::prelude::*;
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::protocol::identifiers::{ConnectionId, PlayerId, RequestId};

#[derive(Debug, DekuRead, DekuWrite, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[deku(
    ctx = "endian: deku::ctx::Endian",
    ctx_default = "deku::ctx::Endian::Little",
    endian = "endian"
)]
/// Take Over Car
pub struct Toc {
    pub reqi: RequestId,

    pub plid: PlayerId,

    pub olducid: ConnectionId,

    #[deku(pad_bytes_after = "2")]
    pub newucid: ConnectionId,
}