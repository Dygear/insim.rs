use crate::string::InsimString;
use deku::prelude::*;
use serde::Serialize;

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Serialize)]
#[deku(endian = "little")]
pub struct HostInfo {
    #[deku(bytes = "32")]
    pub hname: InsimString,

    #[deku(bytes = "6")]
    pub track: InsimString,

    #[deku(bytes = "1")]
    pub flags: u8,

    #[deku(bytes = "1")]
    pub numconns: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Serialize)]
#[deku(ctx = "_endian: deku::ctx::Endian")]
pub struct AdminRequest {
    #[deku(bytes = "1", pad_bytes_after = "1")]
    pub reqi: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Serialize)]
#[deku(ctx = "_endian: deku::ctx::Endian")]
pub struct AdminResponse {
    #[deku(bytes = "1")]
    pub reqi: u8,
    #[deku(bytes = "1")]
    pub admin: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Default, Serialize)]
#[deku(ctx = "_endian: deku::ctx::Endian")]
pub struct HostListRequest {
    #[deku(bytes = "1", pad_bytes_after = "1")]
    pub reqi: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Serialize)]
#[deku(ctx = "_endian: deku::ctx::Endian")]
pub struct HostList {
    #[deku(bytes = "1")]
    pub reqi: u8,

    #[deku(bytes = "1")]
    pub numhosts: u8,

    #[deku(count = "numhosts")]
    pub hinfo: Vec<HostInfo>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Default, Serialize)]
#[deku(ctx = "_endian: deku::ctx::Endian")]
pub struct HostSelect {
    #[deku(bytes = "1", pad_bytes_after = "1")]
    pub reqi: u8,

    // zero handled by pad_bytes_after
    #[deku(bytes = "32")]
    pub hname: InsimString,

    #[deku(bytes = "16")]
    pub admin: InsimString,

    #[deku(bytes = "16")]
    pub spec: InsimString,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite, Clone, Serialize)]
#[deku(ctx = "_endian: deku::ctx::Endian")]
pub struct Error {
    #[deku(bytes = "1")]
    pub reqi: u8,

    #[deku(bytes = "1")]
    pub errno: u8,
}