use std::{
    fmt,
    fmt::{Debug, Display},
    str::FromStr
};

/// The interface for a generic derivation path.
pub trait DerivationPath: Clone + Debug + Display + FromStr + Send + Sync + 'static + Eq + Sized {}

#[derive(Debug, Fail)]
pub enum DerivationPathError {

    #[fail(display = "expected hardened path")]
    ExpectedHardenedPath,

    #[fail(display = "expected normal path")]
    ExpectedNormalPath,

    #[fail(display = "invalid child number: {}", _0)]
    InvalidChildNumber(u32),

    #[fail(display = "invalid child number format")]
    InvalidChildNumberFormat,

    #[fail(display = "invalid derivation path: {}", _0)]
    InvalidDerivationPath(String),
}

/// Represents a child index for a derivation path
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChildIndex {
    // A non-hardened index: Normal(n) == n in path notation
    Normal(u32),
    // A hardened index: Hardened(n) == n + (1 << 31) == n' in path notation
    Hardened(u32),
}

impl ChildIndex {
    /// Returns [`Normal`] from an index, or errors if the index is not within [0, 2^31 - 1].
    pub fn from_normal(index: u32) -> Result<Self, DerivationPathError> {
        if index & (1 << 31) == 0 {
            Ok(ChildIndex::Normal(index))
        } else {
            Err(DerivationPathError::InvalidChildNumber(index))
        }
    }

    /// Returns [`Hardened`] from an index, or errors if the index is not within [0, 2^31 - 1].
    pub fn from_hardened(index: u32) -> Result<Self, DerivationPathError> {
        if index & (1 << 31) == 0 {
            Ok(ChildIndex::Hardened(index))
        } else {
            Err(DerivationPathError::InvalidChildNumber(index))
        }
    }

    /// Returns `true` if the child index is a [`Normal`] value.
    pub fn is_normal(&self) -> bool {
        !self.is_hardened()
    }

    /// Returns `true` if the child index is a [`Hardened`] value.
    pub fn is_hardened(&self) -> bool {
        match *self {
            ChildIndex::Hardened(_) => true,
            ChildIndex::Normal(_) => false,
        }
    }
}

impl From<u32> for ChildIndex {
    fn from(number: u32) -> Self {
        if number & (1 << 31) != 0 {
            ChildIndex::Hardened(number ^ (1 << 31))
        } else {
            ChildIndex::Normal(number)
        }
    }
}

impl From<ChildIndex> for u32 {
    fn from(index: ChildIndex) -> Self {
        match index {
            ChildIndex::Normal(number) => number,
            ChildIndex::Hardened(number) => number | (1 << 31),
        }
    }
}

impl FromStr for ChildIndex {
    type Err = DerivationPathError;

    fn from_str(inp: &str) -> Result<Self, Self::Err> {
        Ok(match inp.chars().last().map_or(false, |l| l == '\'' || l == 'h') {
            true => Self::from_hardened(
                inp[0..inp.len() - 1].parse().map_err(|_| DerivationPathError::InvalidChildNumberFormat)?
            )?,
            false => Self::from_normal(
                inp.parse().map_err(|_| DerivationPathError::InvalidChildNumberFormat)?
            )?,
        })
    }
}

impl fmt::Display for ChildIndex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ChildIndex::Hardened(number) => write!(f, "{}'", number),
            ChildIndex::Normal(number) => write!(f, "{}", number),
        }
    }
}
