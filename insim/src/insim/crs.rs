use insim_core::{
    identifiers::{PlayerId, RequestId},
    binrw::{self, binrw}
};

#[cfg(feature = "serde")]
use serde::Serialize;

#[binrw]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "serde", derive(Serialize))]
/// Car Reset packet indicates a vehicle has been reset or that a vehicle should be reset by the
/// server.
pub struct Crs {
    pub reqi: RequestId,
    pub plid: PlayerId,
}
