//! # MAC Address
//! 
//! Contains logic for handling MAC address identifiers
//! 
//! Also known as:
//! 
//! - MID        = MAC ID,
//! - LL_ADDR    = Link Layer Address
//! - MAC_ADDR   = MAC Address
//! 
//! # Examples:
//! 
//! ```
//! use geonetrs::types::macaddr::MACAddress;
//! let mac = MACAddress::new(&[1, 2, 3, 4, 5, 6]);
//! // convert to Hexadecimal string, column separated
//! let s = format!("{}", mac);
//! assert_eq!(s, "01:02:03:04:05:06");
//! ```

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct MACAddress([u8; 6]);

impl fmt::Display for MACAddress {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = format!("{:02x}", self.0[0]);

        for i in 1..6 {
            s.push_str(&format!(":{:02x}", self.0[i]));
        }

        write!(f, "{}", s)
    }
}

impl MACAddress {
    pub fn new(mac: &[u8; 6]) -> MACAddress {
        let mut m: [u8; 6] = [0; 6];
        m.copy_from_slice(mac);
        MACAddress(m)
    }
    
    pub fn get(&self) -> &[u8; 6] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mac_address_new() {
        let m: [u8; 6] = [6, 1, 5, 2, 4, 3];
        let mac = MACAddress::new(&m);
        assert!(mac.0 == m);
    }

    #[test]
    fn mac_address_eq() {
        let m: [u8; 6] = [6, 1, 5, 2, 4, 3];
        let mac1 = MACAddress::new(&m);
        let mac2 = MACAddress::new(&m);
        assert!(mac1 == mac2);
        assert!(mac1.0 == mac2.0);
    }

    #[test]
    fn mac_address_get() {
        let m1: [u8; 6] = [6, 1, 5, 2, 4, 3];
        let mac = MACAddress::new(&m1);
        let m2 = mac.get();
        assert!(&m1 == m2);
        assert!(m1 == *m2);
    }

    #[test]
    fn mac_address_display() {
        let mac1 = MACAddress::default();
        assert_eq!(format!("{}", mac1), "00:00:00:00:00:00");

        let mac2 = MACAddress::new(&[1, 2, 3, 4, 5, 6]);
        assert_eq!(format!("{}", mac2), "01:02:03:04:05:06");
    }
}
