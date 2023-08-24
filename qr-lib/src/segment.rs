use std::error::Error;
use std::fmt::{Display, Formatter};
use bitvec::macros::internal::funty::Fundamental;
use bitvec::prelude::*;
use crate::QrVersion;

#[derive(Debug)]
pub enum EncodeQrSegmentError {
    InvalidAlphanumericCharacter(char),
}

impl Display for EncodeQrSegmentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { EncodeQrSegmentError::InvalidAlphanumericCharacter(char) => { write!(f, "invalid alphanumeric character {}, must be one of '0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ $%*+-./:'", char) } }
    }
}

impl Error for EncodeQrSegmentError {}

pub enum QrSegmentMode {
    Numeric,
    Alphanumeric,
    Byte,
    Kanji,
}

impl QrSegmentMode {
    pub const fn mode_indicator(&self) -> u8 {
        match self {
            QrSegmentMode::Numeric => 0b0001,
            QrSegmentMode::Alphanumeric => 0b0001,
            QrSegmentMode::Byte => 0b0100,
            QrSegmentMode::Kanji => 0b1000,
        }
    }

    pub const fn chunk_length(&self) -> usize {
        match self {
            QrSegmentMode::Numeric => 10,
            QrSegmentMode::Alphanumeric => 11,
            QrSegmentMode::Byte => unimplemented!(),
            QrSegmentMode::Kanji => unimplemented!(),
        }
    }

    pub const fn character_count_indicator_length(&self, version: QrVersion) -> usize {
        if version.to_u8() <= QrVersion::V9.to_u8() {
            match self {
                QrSegmentMode::Numeric => 10,
                QrSegmentMode::Alphanumeric => 9,
                QrSegmentMode::Byte => 8,
                QrSegmentMode::Kanji => 8,
            }
        } else if version.to_u8() <= QrVersion::V26.to_u8() {
            match self {
                QrSegmentMode::Numeric => 12,
                QrSegmentMode::Alphanumeric => 11,
                QrSegmentMode::Byte => 16,
                QrSegmentMode::Kanji => 10,
            }
        } else {
            match self {
                QrSegmentMode::Numeric => 14,
                QrSegmentMode::Alphanumeric => 13,
                QrSegmentMode::Byte => 16,
                QrSegmentMode::Kanji => 12,
            }
        }
    }
}

pub struct QrSegment {
    pub mode: QrSegmentMode,
    bit_vec: BitVec,
}

const ALPHANUMERIC_CHUNK_BIT_COUNT: usize = 11;

impl QrSegment {
    fn from_string(string: &str) {}

    pub fn encode_alphanumeric(version: QrVersion, string: &str) -> Result<QrSegment, EncodeQrSegmentError> {
        let mode = QrSegmentMode::Alphanumeric;

        let mut vec = BitVec::new();

        // Segment mode
        vec.extend_from_bitslice(&mode.mode_indicator().view_bits::<Msb0>()[4..]);

        // Segment count
        vec.extend_from_bitslice(&(string.len().as_u16()).view_bits::<Msb0>()[(16 - mode.character_count_indicator_length(version))..]);

        // Segment data
        string
            .chars()
            .map(Self::try_as_alphanumeric_codepoint)
            .collect::<Result<Vec<_>, _>>()?
            .chunks(2)
            .map(Self::encode_alphanumeric_pair)
            .for_each(|(b, n_bits)| {
                vec.extend_from_bitslice(&b.view_bits::<Msb0>()[(16 - n_bits)..]);
            });

        // Terminator
        vec.extend_from_bitslice(&0_u8.view_bits::<Msb0>()[4..]);

        // Bit padding
        vec.extend_from_bitslice(&0_u8.view_bits::<Msb0>()[(vec.len() % 8)..]);


        Self { mode, bit_vec: vec };

        todo!()
    }

    fn encode_alphanumeric_pair(bytes: &[u8]) -> (u16, usize) {
        let b0 = bytes[0] as u16;
        if let Some(&b1) = bytes.get(1) {
            (b0 * 45 + (b1 as u16), QrSegmentMode::Alphanumeric.chunk_length())
        } else {
            (b0, 6)
        }
    }

    pub fn encode_numeric(string: &str) -> Result<QrSegment, EncodeQrSegmentError> {
        unimplemented!()
    }

    fn try_as_alphanumeric_codepoint(c: char) -> Result<u8, EncodeQrSegmentError> {
        if !c.is_ascii() {
            Err(EncodeQrSegmentError::InvalidAlphanumericCharacter(c))
        } else {
            let b = c.as_u8();
            match b {
                b'0'..=b'9' => Ok(b - b'0'),
                b'A'..=b'Z' => Ok(b - b'A' + 10),
                b' ' => Ok(36),
                b'$' => Ok(37),
                b'%' => Ok(38),
                b'*' => Ok(39),
                b'+' => Ok(40),
                b'-' => Ok(41),
                b'.' => Ok(42),
                b'/' => Ok(43),
                b':' => Ok(44),
                _ => Err(EncodeQrSegmentError::InvalidAlphanumericCharacter(c))
            }
        }
    }
}






S



