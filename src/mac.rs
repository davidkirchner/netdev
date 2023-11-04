use std::error;
use std::fmt;
use core::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Structure of MAC address
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MacAddr(pub u8, pub u8, pub u8, pub u8, pub u8, pub u8);

impl MacAddr {
    /// Construct a new `MacAddr` instance.
    pub fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> MacAddr {
        MacAddr(a, b, c, d, e, f)
    }
    /// Construct a new MacAddr instance from the given octets
    pub fn from_octets(octets: [u8; 6]) -> MacAddr {
        MacAddr(
            octets[0], octets[1], octets[2], octets[3], octets[4], octets[5],
        )
    }
    /// Returns an array of MAC address octets
    pub fn octets(&self) -> [u8; 6] {
        [self.0, self.1, self.2, self.3, self.4, self.5]
    }
    /// Return a formatted string of MAC address
    pub fn address(&self) -> String {
        format!(
            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
            self.0, self.1, self.2, self.3, self.4, self.5
        )
    }
    /// Construct an all-zero MacAddr instance
    pub fn zero() -> MacAddr {
        MacAddr(0, 0, 0, 0, 0, 0)
    }
    /// Construct a broadcast `MacAddr` instance.
    pub fn broadcast() -> MacAddr {
        MacAddr(0xff, 0xff, 0xff, 0xff, 0xff, 0xff)
    }
    /// Construct a new MacAddr instance from a colon-separated string of hex format
    pub fn from_hex_format(hex_mac_addr: &str) -> MacAddr {
        if hex_mac_addr.len() != 17 {
            return MacAddr(0, 0, 0, 0, 0, 0);
        }
        let fields: Vec<&str> = hex_mac_addr.split(":").collect();
        let o1: u8 = u8::from_str_radix(&fields[0], 0x10).unwrap_or(0);
        let o2: u8 = u8::from_str_radix(&fields[1], 0x10).unwrap_or(0);
        let o3: u8 = u8::from_str_radix(&fields[2], 0x10).unwrap_or(0);
        let o4: u8 = u8::from_str_radix(&fields[3], 0x10).unwrap_or(0);
        let o5: u8 = u8::from_str_radix(&fields[4], 0x10).unwrap_or(0);
        let o6: u8 = u8::from_str_radix(&fields[5], 0x10).unwrap_or(0);
        MacAddr(o1, o2, o3, o4, o5, o6)
    }
}

impl std::fmt::Display for MacAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let _ = write!(
            f,
            "{:<02x}:{:<02x}:{:<02x}:{:<02x}:{:<02x}:{:<02x}",
            self.0, self.1, self.2, self.3, self.4, self.5
        );
        Ok(())
    }
}

/// Represents an error which occurred whilst parsing a MAC address
#[derive(Copy, Debug, PartialEq, Eq, Clone)]
pub enum ParseMacAddrError {
    /// The MAC address has too many components, eg. 00:11:22:33:44:55:66
    TooManyComponents,
    /// The MAC address has too few components, eg. 00:11
    TooFewComponents,
    /// One of the components contains an invalid value, eg. 00:GG:22:33:44:55
    InvalidComponent,
}

impl error::Error for ParseMacAddrError {}

impl ParseMacAddrError {
    fn description(&self) -> &str {
        match *self {
            ParseMacAddrError::TooManyComponents => "Too many components in a MAC address string",
            ParseMacAddrError::TooFewComponents => "Too few components in a MAC address string",
            ParseMacAddrError::InvalidComponent => "Invalid component in a MAC address string",
        }
    }
}

impl fmt::Display for ParseMacAddrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl FromStr for MacAddr {
    type Err = ParseMacAddrError;
    fn from_str(s: &str) -> Result<MacAddr, ParseMacAddrError> {
        let mut parts = [0u8; 6];
        let splits = s.split(':');
        let mut i = 0;
        for split in splits {
            if i == 6 {
                return Err(ParseMacAddrError::TooManyComponents);
            }
            match u8::from_str_radix(split, 16) {
                Ok(b) if split.len() != 0 => parts[i] = b,
                _ => return Err(ParseMacAddrError::InvalidComponent),
            }
            i += 1;
        }

        if i == 6 {
            Ok(MacAddr(
                parts[0], parts[1], parts[2], parts[3], parts[4], parts[5],
            ))
        } else {
            Err(ParseMacAddrError::TooFewComponents)
        }
    }
}
