/**
 * ITS Station logic
 *
 * Contains ITS station types and logic
 */
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
            _ => Ok(StationType::Unknown),
        }
    }
}

impl fmt::Display for StationType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ItsStation {
    pub id: u32,
    pub stype: StationType
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::vec::Vec;

    #[test]
    fn station_type_from_u8() {
        let sets: Vec<(u8, StationType)> = vec![
            (0, StationType::Unknown),
            (1, StationType::Pedestrian),
            (2, StationType::Cyclist),
            (3, StationType::Moped),
            (4, StationType::Motorcycle),
            (5, StationType::PassengerCar),
            (6, StationType::Bus),
            (7, StationType::LightTruck),
            (8, StationType::HeavyTruck),
            (9, StationType::Trailer),
            (10, StationType::SpecialVehicle),
            (11, StationType::Tram),
            (15, StationType::RoadSideUnit),
        ];

        for (val, expected) in sets {
            assert_eq!(StationType::try_from(val).unwrap(), expected);
        }
    }

    #[test]
    fn station_type_display() {
        let types: Vec<StationType> = vec![
            StationType::Unknown,
            StationType::Pedestrian,
            StationType::Cyclist,
            StationType::Moped,
            StationType::Motorcycle,
            StationType::PassengerCar,
            StationType::Bus,
            StationType::LightTruck,
            StationType::HeavyTruck,
            StationType::Trailer,
            StationType::SpecialVehicle,
            StationType::Tram,
            StationType::RoadSideUnit,
        ];

        for t in types {
            assert_eq!(format!("{t}"), format!("{t:?}"));
            assert_eq!(format!("{t}"), t.to_string());
        }
    }
}
