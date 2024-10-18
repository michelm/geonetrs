/** 
 * Location Table (Entry)
 * 
 * Contains logic and data definitions the Location Table (Entry)
 * as defined clause 8.1 of ETSI EN 302 636-4-1 V1.4.1 (2020-01)
 */

use crate::{
    types::gnaddr::GNAddress, 
    types::macaddr::MACAddress, 
    types::station::StationType
};

/// Location Table Entry (LocTE)
pub struct LocTE {
    /// GeoNetworking Address (GN_ADDR)
    pub gnaddr: GNAddress,

    /// Link Layer Address (LL_ADDR, MAC_ADDR, MID)
    pub lladdr: MACAddress,

    /// Station type
    pub station: StationType,

    /// Geonetworking protocol version
    pub version: u8,

    /// Position vector PV, i.e. Long Position Vector LPV (clause 9.5.2), of the ITS-S
    pub lpv: u8,

    /// Flag indicating that a Location Service (LS) (clause 10.2.4) is in progress
    pub ls_pending: bool,

    /// Flag indicating that the GeoAdhoc router is in direct communication range,
    /// i.e. is a neighbour.
    pub neighbour: bool,

    /// Duplicate packet list for source GN_ADDR
    pub dpl: u8,

    /// The timestamp of the last packet from the source GN_ADDR that was 
    /// identified as 'not duplicated'. 
    pub timestamp: u64,

    /// Packet data rate PDR(GN_ADDR) as Exponential Moving Average (EMA) (clause B.2)
    pub pdr: u8,
}
