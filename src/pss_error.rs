// GNU General Public License v3.0 Only
// Copyright (C) 2026 0x1F980

extern crate alloc;

use alloc::string::String;
use core::fmt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PssError {
    InvalidK,
    InvalidN,
    InsufficientShares,
    InconsistentShares,
    FileTooShort,
    CorpusSearchFailed,
    KeylessViolation,
    Io(String),
    Other(&'static str),
}

impl fmt::Display for PssError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidK => write!(f, "k must be at least 1"),
            Self::InvalidN => write!(f, "n must be >= k"),
            Self::InsufficientShares => write!(f, "need at least k shares"),
            Self::InconsistentShares => write!(f, "shares fail k+1 consistency check"),
            Self::FileTooShort => write!(f, "carrier file too short for transpose offset"),
            Self::CorpusSearchFailed => write!(f, "corpus search found no matching carriers"),
            Self::KeylessViolation => write!(f, "output directory must contain exactly n files"),
            Self::Io(msg) => write!(f, "io error: {msg}"),
            Self::Other(msg) => write!(f, "{msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for PssError {}

pub type PssResult<T> = Result<T, PssError>;
