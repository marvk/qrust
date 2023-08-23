use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParseQrVersionError {
    OutOfRange(u8),
}

impl Display for ParseQrVersionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { ParseQrVersionError::OutOfRange(value) => { write!(f, "qr version is out of range, was {} but supposed to be in 1..=40", value) } }
    }
}

impl Error for ParseQrVersionError {}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct QrVersion(u8);

impl TryFrom<u8> for QrVersion {
    type Error = ParseQrVersionError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=40).contains(&value) {
            Err(ParseQrVersionError::OutOfRange(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<QrVersion> for u8 {
    fn from(value: QrVersion) -> Self {
        value.0
    }
}

impl QrVersion {
    pub fn to_u8(self) -> u8 {
        self.0
    }

    pub fn side_length(&self) -> usize {
        (((self.0 as i32 - 1) * 4) + 21) as usize
    }

    pub const V1: Self = Self(1);
    pub const V2: Self = Self(2);
    pub const V3: Self = Self(3);
    pub const V4: Self = Self(4);
    pub const V5: Self = Self(5);
    pub const V6: Self = Self(6);
    pub const V7: Self = Self(7);
    pub const V8: Self = Self(8);
    pub const V9: Self = Self(9);
    pub const V10: Self = Self(10);
    pub const V11: Self = Self(11);
    pub const V12: Self = Self(12);
    pub const V13: Self = Self(13);
    pub const V14: Self = Self(14);
    pub const V15: Self = Self(15);
    pub const V16: Self = Self(16);
    pub const V17: Self = Self(17);
    pub const V18: Self = Self(18);
    pub const V19: Self = Self(19);
    pub const V20: Self = Self(20);
    pub const V21: Self = Self(21);
    pub const V22: Self = Self(22);
    pub const V23: Self = Self(23);
    pub const V24: Self = Self(24);
    pub const V25: Self = Self(25);
    pub const V26: Self = Self(26);
    pub const V27: Self = Self(27);
    pub const V28: Self = Self(28);
    pub const V29: Self = Self(29);
    pub const V30: Self = Self(30);
    pub const V31: Self = Self(31);
    pub const V32: Self = Self(32);
    pub const V33: Self = Self(33);
    pub const V34: Self = Self(34);
    pub const V35: Self = Self(35);
    pub const V36: Self = Self(36);
    pub const V37: Self = Self(37);
    pub const V38: Self = Self(38);
    pub const V39: Self = Self(39);
    pub const V40: Self = Self(40);
}
