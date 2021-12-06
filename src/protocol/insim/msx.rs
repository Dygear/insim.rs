use crate::string::ICodepageString;
use deku::prelude::*;
use serde::Serialize;

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Serialize)]
#[deku(ctx = "_endian: deku::ctx::Endian")]
/// Extended Message (like [Mst](super::Mst), but longer)
pub struct Msx {
    #[deku(pad_bytes_after = "1")]
    pub reqi: u8,

    #[deku(bytes = "96")]
    pub msg: ICodepageString,
}
