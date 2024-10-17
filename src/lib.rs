use std::fmt;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum StationType {
    Unknown         = 0,
    Pedestrian      = 1,
    Cyclist         = 2,
    Moped           = 3,
    Motorcycle      = 4,
    PassengerCar    = 5, 
    Bus             = 6,
    LightTruck      = 7,
    HeavyTruck      = 8,
    Trailer         = 9,
    SpecialVehicle  = 10,
    Tram            = 11,
    RoadSideUnit    = 15, 
}

impl std::convert::TryFrom<u8> for StationType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(StationType::Unknown),
            1 => Ok(StationType::Pedestrian),
            2 => Ok(StationType::Cyclist),
            3 => Ok(StationType::Moped),
            4 => Ok(StationType::Motorcycle),
            5 => Ok(StationType::PassengerCar),
            6 => Ok(StationType::Bus),
            7 => Ok(StationType::LightTruck),
            8 => Ok(StationType::HeavyTruck),
            9 => Ok(StationType::Trailer),
            10 => Ok(StationType::SpecialVehicle),
            11 => Ok(StationType::Tram),
            15 => Ok(StationType::RoadSideUnit),
            _ => Err(()),
        }
    }
}

// MID (=MAC ID), LL_ADDR (=Link Layer Address)
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct MACAddress([u8;6]);

impl fmt::Display for MACAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{:02x}", self.0[0]);

        for i in 1..6 {
            s.push_str(&format!(":{:02x}", self.0[i]));
        }

        write!(f, "{}", s)
    }
}

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
        self.mac_address.0 == other.mac_address.0
    }
}

impl Default for GNAddress {
    fn default() -> Self {
        GNAddress {
            manual: false,
            station_type: StationType::Unknown,
            mac_address: MACAddress([0;6]),
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

    pub fn get_mac_address(&self) -> &MACAddress {
        &self.mac_address
    }

    pub fn set_mac_address(&mut self, mac_address: &MACAddress) {
        self.mac_address.0.copy_from_slice(&mac_address.0);
    }

    pub fn encode(&self) -> [u8;8] {
        let mut buf: [u8;8] = [0;8];
        let st = self.station_type as u8;

        buf[0] = st << 1;

        if self.manual {
            buf[0] |= 1 << 7;
        }

        buf[2..8].copy_from_slice(&self.mac_address.0);

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

        gn_addr.mac_address.0.copy_from_slice(&buf[2..8]);

        gn_addr
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_address_display() {
        let mac1 = MACAddress([0;6]);
        assert_eq!(format!("{}", mac1), "00:00:00:00:00:00");

        let mac2 = MACAddress([1,2,3,4,5,6]);
        assert_eq!(format!("{}", mac2), "01:02:03:04:05:06");
    }

    #[test]
    fn gnaddr_new() {
        let gnaddr = GNAddress::default();
        assert!(!gnaddr.is_manual());
        assert_eq!(gnaddr.get_station_type(), StationType::Unknown);
        assert_eq!(gnaddr.get_mac_address(), &MACAddress([0; 6]));
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
        let mac_address = MACAddress([1,2,3,4,5,6]);
        assert_eq!(gnaddr.get_mac_address(), &MACAddress([0; 6]));
        gnaddr.set_mac_address(&mac_address);
        assert_eq!(gnaddr.get_mac_address(), &mac_address);
        gnaddr.set_mac_address(&mac_address);
    }

    #[test]
    fn gnaddr_codec() {
        let mut gnaddr1 = GNAddress::default();
        let mut buf = gnaddr1.encode();
        let mut gnaddr2 = GNAddress::decode(buf);
        assert_eq!(gnaddr1, gnaddr2);
        
        gnaddr1.set_manual(true);
        gnaddr1.set_station_type(StationType::Motorcycle);
        gnaddr1.set_mac_address(&MACAddress([2,1,4,3,6,5]));

        buf = gnaddr1.encode();
        gnaddr2 = GNAddress::decode(buf);
        assert_eq!(gnaddr1, gnaddr2);
    }
}
