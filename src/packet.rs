/** 
 * GeoNetworking packet
 * 
 * Contains logic and data definitions the GeoNetworking packet
 * as defined clause 9 of ETSI EN 302 636-4-1 V1.4.1 (2020-01)
 */

 pub struct GNPacket {
    pub al_header: u8,
    pub gn_header: GNHeader,
    pub payload: Option<Vec<u8>>,
}

pub struct GNHeader {
    pub basic: GNBasicHeader,
    pub common: GNCommonHeader,
    pub extended: GNExtendedHeader,
}

pub enum GNNextHeader {
    Any     = 0,
    Common  = 1,
    Secured = 2,
}

pub struct GNBasicHeader {
    pub version: u8,
    pub next_header: GNNextHeader,
    pub lifetime: u8,
    pub remaining_hop_limit: u8,
}

pub struct GNCommonHeader {
}

pub struct GNExtendedHeader {
}
