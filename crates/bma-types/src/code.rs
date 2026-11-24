//! Refusal codes. One enum, one string per variant.
//!
//! The whole list lives here so that slices working side by side never edit this file. A new code
//! is its own small pull request.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Declares [`Code`] with its wire strings, so a variant and its string cannot drift apart.
macro_rules! codes {
    ($( $(#[$family:meta])* $variant:ident => $text:literal, )+) => {
        /// A refusal code. `Display` prints the `X_` string that goes on the wire.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[non_exhaustive]
        pub enum Code {
            $( $(#[$family])* #[doc = $text] $variant, )+
        }

        impl Code {
            /// Every code, in declaration order.
            pub const ALL: &'static [Code] = &[ $( Code::$variant, )+ ];

            /// The `X_` string of this code.
            #[must_use]
            pub const fn as_str(self) -> &'static str {
                match self {
                    $( Code::$variant => $text, )+
                }
            }
        }
    };
}

codes! {
    // Host: the URL, the binding and the connection to the upstream.
    HostUrlInvalid => "X_HOST_URL_INVALID",
    HostScheme => "X_HOST_SCHEME",
    HostNotBound => "X_HOST_NOT_BOUND",
    HostMethodNotAllowed => "X_HOST_METHOD_NOT_ALLOWED",
    HostPathNotAllowed => "X_HOST_PATH_NOT_ALLOWED",
    HostDns => "X_HOST_DNS",
    HostPrivateAddress => "X_HOST_PRIVATE_ADDRESS",
    HostRedirectUnbound => "X_HOST_REDIRECT_UNBOUND",
    HostRedirectLimit => "X_HOST_REDIRECT_LIMIT",
    HostTls => "X_HOST_TLS",
    HostTimeout => "X_HOST_TIMEOUT",
    HostUnreachable => "X_HOST_UNREACHABLE",
    HostResponseTooLarge => "X_HOST_RESPONSE_TOO_LARGE",
    HostProxyRefused => "X_HOST_PROXY_REFUSED",
    HostProxyUnreachable => "X_HOST_PROXY_UNREACHABLE",

    // Secret: finding, opening and attaching a credential.
    SecretNotFound => "X_SECRET_NOT_FOUND",
    SecretLocked => "X_SECRET_LOCKED",
    SecretRevoked => "X_SECRET_REVOKED",
    SecretExpired => "X_SECRET_EXPIRED",
    SecretNotGranted => "X_SECRET_NOT_GRANTED",
    SecretSourceUnavailable => "X_SECRET_SOURCE_UNAVAILABLE",
    SecretBindingMismatch => "X_SECRET_BINDING_MISMATCH",
    SecretFormatUnsupported => "X_SECRET_FORMAT_UNSUPPORTED",
    SecretVersionConflict => "X_SECRET_VERSION_CONFLICT",
    SecretReferenceInvalid => "X_SECRET_REFERENCE_INVALID",
    SecretDecryptFailed => "X_SECRET_DECRYPT_FAILED",

    // SQL: parsing and running a statement.
    SqlUnparseable => "X_SQL_UNPARSEABLE",
    SqlMultiStatement => "X_SQL_MULTI_STATEMENT",
    SqlReadOnly => "X_SQL_READ_ONLY",
    SqlTimeout => "X_SQL_TIMEOUT",
    SqlDialect => "X_SQL_DIALECT",
    SqlConnect => "X_SQL_CONNECT",

    // Helper: the desktop helper and its link to the control plane.
    HelperOffline => "X_HELPER_OFFLINE",
    HelperRequired => "X_HELPER_REQUIRED",
    HelperRevoked => "X_HELPER_REVOKED",
    HelperTimeout => "X_HELPER_TIMEOUT",
    HelperUpdateRequired => "X_HELPER_UPDATE_REQUIRED",
    HelperUnsupported => "X_HELPER_UNSUPPORTED",
    HelperEnrolmentExpired => "X_HELPER_ENROLMENT_EXPIRED",
    HelperEnrolmentInvalid => "X_HELPER_ENROLMENT_INVALID",
    HelperPaused => "X_HELPER_PAUSED",
    HelperSandboxUnavailable => "X_HELPER_SANDBOX_UNAVAILABLE",
    HelperMcpUnpinned => "X_HELPER_MCP_UNPINNED",
    HelperApprovalRefused => "X_HELPER_APPROVAL_REFUSED",
    HelperGateSignature => "X_HELPER_GATE_SIGNATURE",
    HelperKeyNotPinned => "X_HELPER_KEY_NOT_PINNED",
    HelperPathRefused => "X_HELPER_PATH_REFUSED",
    HelperFrameTooLarge => "X_HELPER_FRAME_TOO_LARGE",
    HelperRunLimit => "X_HELPER_RUN_LIMIT",
    HelperBusy => "X_HELPER_BUSY",
    HelperGateSetBehind => "X_HELPER_GATE_SET_BEHIND",
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A string that is not a known code. It carries nothing: an unknown string is not echoed.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("not a known refusal code")]
pub struct UnknownCode;

impl FromStr for Code {
    type Err = UnknownCode;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::ALL
            .iter()
            .copied()
            .find(|code| code.as_str() == text)
            .ok_or(UnknownCode)
    }
}

impl Serialize for Code {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        text.parse().map_err(serde::de::Error::custom)
    }
}
