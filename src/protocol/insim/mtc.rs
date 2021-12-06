use crate::string::ICodepageString;
use deku::ctx::Size;
use deku::prelude::*;
use serde::Serialize;

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Serialize)]
#[deku(ctx = "_endian: deku::ctx::Endian")]
/// Message to Connection - Send a message to a specific connection, restricted to hosts only
pub struct Mtc {
    pub reqi: u8,

    pub sound: u8,

    pub ucid: u8,

    #[deku(pad_bytes_after = "2")]
    pub plid: u8,

    #[deku(reader = "ICodepageString::read(deku::rest, Size::Bytes(deku::rest.len() / 8))")]
    pub msg: ICodepageString,
}
