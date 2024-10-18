
/// Protocol major release version.
/// 
/// This constant is also used in the 
/// header of a GeoNetworking frame.
pub const VMAJOR: u8 = 1;

/// Protocol minor release version.
pub const VMINOR: u8 = 3;

/// Protocol patch (bugfix) version.
pub const VPATCH: u8 = 1;

/// Protocol version as a byte array.
pub const VERSION: [u8; 3] = [VMAJOR, VMINOR, VPATCH];

/// Ethernet frame identifier for a GeoNet
/// packet in an Ethernet frame. 
pub const ETHERNET_FRAME_ID: u16 = 0x8947;

/// Maximum Ethernet frame payload size 
/// in bytes.
pub const ETHERNET_MTU_SIZE: usize = 1500;

/// Maximum GeoNet payload size in bytes; i.e. MTU 
/// size minus 200 bytes for the overhead.
pub const PAYLOAD_SIZE: usize = ETHERNET_MTU_SIZE - 200;
