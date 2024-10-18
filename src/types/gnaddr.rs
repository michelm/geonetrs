//!
//! #GeoNetworking address
//! 
//! Contains logic and data definitions the GeoNetworking addresses
//! as defined clause 6 of ETSI EN 302 636-4-1 V1.4.1 (2020-01)
//!

use crate::types::macaddr::MACAddress;
use crate::types::station::StationType;

#[derive(Eq, Debug)]
pub struct GNAddress {
    manual: bool,
    station_type: StationType,
    mac_address: MACAddress,
}

impl PartialEq for GNAddress {
    fn eq(&self, other: &Self) -> bool {
        self.manual == other.manual &&
        self.station_type == other.station_type &&
        self.mac_address.get() == other.mac_address.get()
    }
}

impl Default for GNAddress {
    fn default() -> Self {
        GNAddress {
            manual: false,
            station_type: StationType::Unknown,
            mac_address: MACAddress::default(),
        }
    }
}

impl GNAddress {
    pub fn is_manual(&self) -> bool {
        self.manual
    }

    pub fn set_manual(&mut self, manual: bool) {
        self.manual = manual;
    }

    pub fn get_station_type(&self) -> StationType {
        self.station_type
    }

    pub fn set_station_type(&mut self, station_type: StationType) {
        self.station_type = station_type;
    }
    
    pub fn get_mac_address(&self) -> MACAddress {
        self.mac_address
    }

    pub fn set_mac_address(&mut self, mac_address: MACAddress) {
        self.mac_address = mac_address;
    }

    pub fn encode(&self) -> [u8;8] {
        let mut buf: [u8;8] = [0;8];
        let st = self.station_type as u8;

        buf[0] = st << 1;

        if self.manual {
            buf[0] |= 1 << 7;
        }

        buf[2..8].copy_from_slice(self.mac_address.get());

        buf
    }

    pub fn decode(buf: [u8;8]) -> GNAddress {
        let mut gn_addr = GNAddress::default();

        gn_addr.set_manual((buf[0] & 0x80) != 0);

        let st = (buf[0] & 0x7E) >> 1;

        gn_addr.station_type = match StationType::try_from(st) {
            Ok(st) => st,
            Err(_) => StationType::Unknown,
        };

        gn_addr.mac_address = MACAddress::new(buf[2..8].try_into().unwrap());

        gn_addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gnaddr_new() {
        let gnaddr = GNAddress::default();
        assert!(!gnaddr.is_manual());
        assert_eq!(gnaddr.get_station_type(), StationType::Unknown);
        assert_eq!(gnaddr.get_mac_address().get(), &[0; 6]);
    }

    #[test]
    fn gnaddr_set_manual() {
        let mut gnaddr = GNAddress::default();
        assert!(!gnaddr.is_manual());
        gnaddr.set_manual(true);
        assert!(gnaddr.is_manual());
    }

    #[test]
    fn gnaddr_set_station_type() {
        let mut gnaddr = GNAddress::default();
        assert_eq!(gnaddr.get_station_type(), StationType::Unknown);
        gnaddr.set_station_type(StationType::Motorcycle);
        assert_eq!(gnaddr.get_station_type(), StationType::Motorcycle);
    }

    #[test]
    fn gnaddr_set_mac_address() {
        let mut gnaddr = GNAddress::default();
        assert_eq!(gnaddr.get_mac_address().get(), &[0; 6]);

        let m: [u8; 6] = [1,2,3,4,5,6];
        let mac = MACAddress::new(&m);
        gnaddr.set_mac_address(mac);
        assert_eq!(gnaddr.get_mac_address(), mac);
        assert_eq!(gnaddr.get_mac_address().get(), &m);
    }

    #[test]
    fn gnaddr_encode_decode() {
        let mut gnaddr1 = GNAddress::default();
        let mut buf = gnaddr1.encode();
        let mut gnaddr2 = GNAddress::decode(buf);
        assert_eq!(gnaddr1, gnaddr2);
        
        gnaddr1.set_manual(true);
        gnaddr1.set_station_type(StationType::Motorcycle);
        gnaddr1.set_mac_address(MACAddress::new(&[2,1,4,3,6,5]));

        buf = gnaddr1.encode();
        gnaddr2 = GNAddress::decode(buf);
        assert_eq!(gnaddr1, gnaddr2);
    }
}
