//! Contains [crate::Packet] enum

use insim_core::binrw::{self, binrw};

use crate::identifiers::RequestId;

use crate::insim::*;
use crate::relay::*;

#[binrw]
#[brw(little)]
#[derive(Debug, Clone, from_variants::FromVariants)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
#[non_exhaustive]
/// Enum representing all possible packets receivable via an Insim connection
pub enum Packet {
    #[brw(magic = 1u8)]
    Init(Isi),
    #[brw(magic = 2u8)]
    Version(Version),
    #[brw(magic = 3u8)]
    Tiny(Tiny),
    #[brw(magic = 4u8)]
    Small(Small),
    #[brw(magic = 5u8)]
    State(Sta),
    #[brw(magic = 6u8)]
    SingleCharacter(Sch),
    #[brw(magic = 7u8)]
    StateFlagsPack(Sfp),
    #[brw(magic = 8u8)]
    SetCarCam(Scc),
    #[brw(magic = 9u8)]
    CamPosPack(Cpp),
    #[brw(magic = 10u8)]
    MultiPlayerNotification(Ism),
    #[brw(magic = 11u8)]
    MessageOut(Mso),
    #[brw(magic = 12u8)]
    InsimInfo(Iii),
    #[brw(magic = 13u8)]
    MessageType(Mst),
    #[brw(magic = 14u8)]
    MessageToConnection(Mtc),
    #[brw(magic = 15u8)]
    ScreenMode(Mod),
    #[brw(magic = 16u8)]
    VoteNotification(Vtn),
    #[brw(magic = 17u8)]
    RaceStart(Rst),
    #[brw(magic = 18u8)]
    NewConnection(Ncn),
    #[brw(magic = 19u8)]
    ConnectionLeave(Cnl),
    #[brw(magic = 20u8)]
    ConnectionPlayerRenamed(Cpr),
    #[brw(magic = 21u8)]
    NewPlayer(Npl),
    #[brw(magic = 22u8)]
    PlayerPits(Plp),
    #[brw(magic = 23u8)]
    PlayerLeave(Pll),
    #[brw(magic = 24u8)]
    Lap(Lap),
    #[brw(magic = 25u8)]
    SplitX(Spx),
    #[brw(magic = 26u8)]
    PitStopStart(Pit),
    #[brw(magic = 27u8)]
    PitStopFinish(Psf),
    #[brw(magic = 28u8)]
    PitLane(Pla),
    #[brw(magic = 29u8)]
    CameraChange(Cch),
    #[brw(magic = 30u8)]
    Penalty(Pen),
    #[brw(magic = 31u8)]
    TakeOverCar(Toc),
    #[brw(magic = 32u8)]
    Flag(Flg),
    #[brw(magic = 33u8)]
    PlayerFlags(Pfl),
    #[brw(magic = 34u8)]
    Finished(Fin),
    #[brw(magic = 35u8)]
    Result(Res),
    #[brw(magic = 36u8)]
    Reorder(Reo),
    #[brw(magic = 37u8)]
    NodeLap(Nlp),
    #[brw(magic = 38u8)]
    MultiCarInfo(Mci),
    #[brw(magic = 39u8)]
    MesssageExtended(Msx),
    #[brw(magic = 40u8)]
    MessageLocal(Msl),
    #[brw(magic = 41u8)]
    CarReset(Crs),
    #[brw(magic = 42u8)]
    ButtonFunction(Bfn),
    #[brw(magic = 43u8)]
    AutoXInfo(Axi),
    #[brw(magic = 44u8)]
    AutoXObject(Axo),
    #[brw(magic = 45u8)]
    Button(Btn),
    #[brw(magic = 46u8)]
    ButtonClick(Btc),
    #[brw(magic = 47u8)]
    ButtonType(Btt),
    #[brw(magic = 48u8)]
    ReplayInformation(Rip),
    #[brw(magic = 49u8)]
    ScreenShot(Ssh),
    #[brw(magic = 50u8)]
    Contact(Con),
    #[brw(magic = 51u8)]
    ObjectHit(Obh),
    #[brw(magic = 52u8)]
    HotLapValidity(Hlv),
    #[brw(magic = 53u8)]
    PlayerAllowedCars(Plc),
    #[brw(magic = 54u8)]
    AutoXMultipleObjects(Axm),
    #[brw(magic = 55u8)]
    AdminCommandReport(Acr),
    #[brw(magic = 56u8)]
    Handicaps(Hcp),
    #[brw(magic = 57u8)]
    Nci(Nci),
    #[brw(magic = 58u8)]
    Jrr(Jrr),
    #[brw(magic = 59u8)]
    UserControlObject(Uco),
    #[brw(magic = 60u8)]
    ObjectControl(Oco),
    #[brw(magic = 61u8)]
    TargetToConnection(Ttc),
    #[brw(magic = 62u8)]
    SelectedVehicle(Slc),
    #[brw(magic = 63u8)]
    VehicleStateChanged(Csc),
    #[brw(magic = 64u8)]
    ConnectionInterfaceMode(Cim),
    #[brw(magic = 65u8)]
    ModsAllowed(Mal),
    #[brw(magic = 66u8)]
    Plh(Plh),

    #[brw(magic = 250u8)]
    RelayAdminRequest(AdminRequest),

    #[brw(magic = 251u8)]
    RelayAdminResponse(AdminResponse),

    #[brw(magic = 252u8)]
    RelayHostListRequest(HostListRequest),

    #[brw(magic = 253u8)]
    RelayHostList(HostList),

    #[brw(magic = 254u8)]
    RelayHostSelect(HostSelect),

    #[brw(magic = 255u8)]
    RelayError(RelayError),
}

impl Default for Packet {
    fn default() -> Self {
        Self::Tiny(Tiny::default())
    }
}

impl Packet {
    /// Hint at the possible *minimum* size of a packet, so that when we encode it, it can pre-allocate a
    /// ballpark buffer.
    /// It must not be trusted. An incorrect implementation of size_hint() should not lead to memory safety violations.
    pub fn size_hint(&self) -> usize {
        // TODO: For some of these packets we can be more intelligent.
        // i.e. see RelayHostList
        match self {
            Packet::Init(_) => 44,
            Packet::Version(_) => 20,
            Packet::Small(_) => 8,
            Packet::State(_) => 28,
            Packet::SingleCharacter(_) => 8,
            Packet::StateFlagsPack(_) => 8,
            Packet::SetCarCam(_) => 8,
            Packet::CamPosPack(_) => 32,
            Packet::MultiPlayerNotification(_) => 40,
            Packet::MessageOut(_) => 12,
            Packet::InsimInfo(_) => 12,
            Packet::MessageType(_) => 68,
            Packet::MessageToConnection(_) => 12,
            Packet::ScreenMode(_) => 20,
            Packet::VoteNotification(_) => 8,
            Packet::RaceStart(_) => 28,
            Packet::NewConnection(_) => 56,
            Packet::ConnectionLeave(_) => 8,
            Packet::ConnectionPlayerRenamed(_) => 36,
            Packet::NewPlayer(_) => 76,
            Packet::Lap(_) => 20,
            Packet::SplitX(_) => 16,
            Packet::PitStopStart(_) => 24,
            Packet::PitStopFinish(_) => 12,
            Packet::PitLane(_) => 8,
            Packet::CameraChange(_) => 8,
            Packet::Penalty(_) => 8,
            Packet::TakeOverCar(_) => 8,
            Packet::Flag(_) => 8,
            Packet::PlayerFlags(_) => 8,
            Packet::Finished(_) => 20,
            Packet::Result(_) => 86,
            Packet::Reorder(_) => 44,
            Packet::NodeLap(_) => 10,
            Packet::MultiCarInfo(_) => 32,
            Packet::MesssageExtended(_) => 100,
            Packet::MessageLocal(_) => 132,
            Packet::CarReset(_) => 4,
            Packet::ButtonFunction(_) => 8,
            Packet::AutoXInfo(_) => 40,
            Packet::AutoXObject(_) => 4,
            Packet::Button(_) => 16,
            Packet::ButtonClick(_) => 8,
            Packet::ButtonType(_) => 104,
            Packet::ReplayInformation(_) => 80,
            Packet::ScreenShot(_) => 40,
            Packet::Contact(_) => 40,
            Packet::ObjectHit(_) => 24,
            Packet::HotLapValidity(_) => 16,
            Packet::PlayerAllowedCars(_) => 8,
            Packet::AutoXMultipleObjects(_) => 16,
            Packet::AdminCommandReport(_) => 12,
            Packet::Handicaps(_) => 68,
            Packet::Nci(_) => 16,
            Packet::Jrr(_) => 16,
            Packet::UserControlObject(_) => 28,
            Packet::ObjectControl(_) => 8,
            Packet::TargetToConnection(_) => 8,
            Packet::SelectedVehicle(_) => 8,
            Packet::VehicleStateChanged(_) => 20,
            Packet::ConnectionInterfaceMode(_) => 8,
            Packet::ModsAllowed(_) => 12,
            Packet::RelayHostList(i) => 4 + (i.hinfo.len() * 40),
            Packet::RelayHostSelect(_) => 68,
            _ => {
                // a sensible default for everything else
                4
            }
        }
    }

    /// Does this packet indicate that we should send a ping reply back?
    #[tracing::instrument]
    pub fn maybe_pong(&self) -> Option<Self> {
        match self {
            Packet::Tiny(Tiny {
                subt: TinyType::None,
                reqi: RequestId(0),
            }) => Some(Self::Tiny(Tiny {
                reqi: RequestId(0),
                subt: TinyType::None,
            })),
            _ => None,
        }
    }

    /// Does this packet contain the version of the Insim server, and can we verify it?
    #[tracing::instrument]
    pub fn maybe_verify_version(&self) -> crate::result::Result<bool> {
        match self {
            Packet::Version(Version { insimver, .. }) => {
                if *insimver != crate::VERSION {
                    return Err(crate::error::Error::IncompatibleVersion(*insimver));
                }

                Ok(true)
            }
            _ => Ok(false),
        }
    }
}
