//! UUID identifiers. Each kind is its own type, so an org id is never passed where a connection
//! id is meant.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Why a string is not an identifier. It never carries the string.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum IdError {
    /// Not 36 characters.
    #[error("an identifier is 36 characters: 32 hex digits in groups of 8-4-4-4-12")]
    Length,
    /// A hyphen is missing, or a digit is not lowercase hex.
    #[error("an identifier is lowercase hex digits in groups of 8-4-4-4-12")]
    Format,
}

/// The canonical form only: lowercase, hyphenated. One identifier has one spelling, so two
/// systems that hash it get the same bytes.
fn parse_uuid(text: &str) -> Result<u128, IdError> {
    let bytes = text.as_bytes();
    if bytes.len() != 36 {
        return Err(IdError::Length);
    }
    let mut value: u128 = 0;
    for (index, &byte) in bytes.iter().enumerate() {
        let hyphen = matches!(index, 8 | 13 | 18 | 23);
        let digit = match byte {
            b'-' if hyphen => continue,
            b'0'..=b'9' if !hyphen => byte - b'0',
            b'a'..=b'f' if !hyphen => byte - b'a' + 10,
            _ => return Err(IdError::Format),
        };
        value = (value << 4) | u128::from(digit);
    }
    Ok(value)
}

fn write_uuid(value: u128, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        f,
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        value >> 96,
        (value >> 80) & 0xffff,
        (value >> 64) & 0xffff,
        (value >> 48) & 0xffff,
        value & 0xffff_ffff_ffff,
    )
}

/// Declares one identifier type over a UUID.
macro_rules! uuid_id {
    ($(#[$doc:meta])+ $name:ident) => {
        $(#[$doc])+
        #[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub struct $name(u128);

        impl $name {
            /// From the 128 bits of the UUID.
            #[must_use]
            pub const fn from_u128(value: u128) -> Self {
                Self(value)
            }

            /// The 128 bits of the UUID.
            #[must_use]
            pub const fn as_u128(self) -> u128 {
                self.0
            }
        }

        impl FromStr for $name {
            type Err = IdError;

            fn from_str(text: &str) -> Result<Self, Self::Err> {
                parse_uuid(text).map(Self)
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write_uuid(self.0, f)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}({self})", stringify!($name))
            }
        }

        impl Serialize for $name {
            fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.collect_str(self)
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                let text = String::deserialize(deserializer)?;
                text.parse().map_err(serde::de::Error::custom)
            }
        }
    };
}

uuid_id! {
    /// An organisation. Every call is scoped to one.
    OrgId
}

uuid_id! {
    /// A connection: one upstream, with the hosts a credential may reach.
    ConnectionId
}

uuid_id! {
    /// An item in a vault.
    ItemId
}

uuid_id! {
    /// One request through the broker, for logs and audit.
    RequestId
}
