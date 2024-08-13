use crate::fields::MacAddress;
use crate::std::*;
#[cfg(feature = "serde")]
use serde::{Serialize, Serializer, Deserialize, Deserializer, de::Error as DeError};


#[derive(Debug, thiserror_no_std::Error)]
pub enum PpeAddressParseError {
    #[error("invalid ppe address: `{0}`")]
    InvalidPpeAddress(String),
}


#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum PpeAddress {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
    Mac(MacAddress),
    Usize(usize),
    // String(String),
}


impl PpeAddress {
    #[inline]
    pub fn is_ipv4(&self) -> bool {
        if let Self::V4(_) = self {true} else {false}
    }

    #[inline]
    pub fn is_ipv6(&self) -> bool {
        if let Self::V6(_) = self {true} else {false}
    }

    #[inline]
    pub fn is_mac(&self) -> bool {
        if let Self::Mac(_) = self {true} else {false}
    }

    #[inline]
    pub fn is_usize(&self) -> bool {
        if let Self::Usize(_) = self {true} else {false}
    }

    // #[inline]
    // pub fn is_string(&self) -> bool {
    //     if let Self::String(_) = self {true} else {false}
    // }

    #[inline]
    pub fn is_broadcast(&self) -> bool {
        match self {
    
            Self::V4(v) => v.is_broadcast(),
    
            Self::Mac(v) => v.is_broadcast(),
            _ => false,
        }
    }

    #[inline]
    pub fn is_multicast(&self) -> bool {
        match self {
    
            Self::V4(v) => v.is_multicast(),
    
            Self::V6(v) => v.is_multicast(),
            _ => false,
        }
    }
}


impl Default for PpeAddress {
    fn default() -> Self {
        Self::Usize(0)
    }
}


impl FromStr for PpeAddress {
    type Err = PpeAddressParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            // Ipv6
            if let Ok(v) = Ipv6Addr::from_str(s) {
                return Ok(Self::V6(v));
            }

            if let Ok(v) = MacAddress::from_str(s) {
                return Ok(Self::Mac(v));
            }
        }
        else if s.contains('.') {
            // Ipv4
            if let Ok(v) = Ipv4Addr::from_str(s) {
                return Ok(Self::V4(v));
            }
        }
        else if let Ok(v) = s.parse::<usize>() {
            return Ok(Self::Usize(v));
        }

        // Ok(Self::String(s.to_string()))

        Err(Self::Err::InvalidPpeAddress(s.to_string()))
    }
}


impl ToString for PpeAddress {
    fn to_string(&self) -> String {
        match self {
            Self::V4(v) => v.to_string(),
            Self::V6(v) => v.to_string(),
            Self::Usize(v) => v.to_string(),
            Self::Mac(v) => v.to_string(),
            // Self::String(v) => v.to_string(),
        }
    }
}


#[cfg(feature = "serde")]
impl Serialize for PpeAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(&self.to_string())
    }
}


#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for PpeAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let value = Deserialize::deserialize(deserializer)?;

        PpeAddress::from_str(value).map_err(D::Error::custom)
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;
    #[cfg(feature = "serde")]
    use serde::{Deserialize, Serialize};
    use super::*;

    #[test]
    fn test_ppe_address_parse() {
        assert_eq!(PpeAddress::from_str("192.168.1.1").unwrap(), PpeAddress::V4(Ipv4Addr::from_str("192.168.1.1").unwrap()));
        assert_eq!(PpeAddress::from_str("fe80::4159:f7b2:b9ed:968a").unwrap(), PpeAddress::V6(Ipv6Addr::from_str("fe80::4159:f7b2:b9ed:968a").unwrap()));
        assert_eq!(PpeAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap(), PpeAddress::Mac(MacAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap()));
        assert_eq!(PpeAddress::from_str("12").unwrap(), PpeAddress::Usize(12));

        assert_eq!(PpeAddress::from_str("").is_err(), true);
        assert_eq!(PpeAddress::from_str(":").is_err(), true);
        assert_eq!(PpeAddress::from_str("aa:bb:cc:dd:ee:").is_err(), true);
        assert_eq!(PpeAddress::from_str("aa:bb:cc:dd:ee:f").is_err(), true);
        assert_eq!(PpeAddress::from_str("aa:bb:cc:dd:ee:ff:").is_err(), true);
        assert_eq!(PpeAddress::from_str("aa:bb:cc:dd:ee:fff").is_err(), true);

        assert_eq!(PpeAddress::from_str("192.168.1.1345").is_err(), true);
        assert_eq!(PpeAddress::from_str("192.168.1.").is_err(), true);
        assert_eq!(PpeAddress::from_str("a").is_err(), true);
    }

    #[cfg(feature = "serde")]
    #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
    struct Example {
        pub addr: PpeAddress,
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_ppe_address_serde() {
        // PpeAddress
        let example = Example {addr: PpeAddress::from_str("aa:bb:cc:dd:ee:ff").unwrap()};

        let example_string = serde_json::to_string(&example).unwrap();
        assert_eq!(example_string, "{\"addr\":\"aa:bb:cc:dd:ee:ff\"}");
        let example_new: Example = serde_json::from_str(&example_string).unwrap();
        assert_eq!(example, example_new);
    }
}
