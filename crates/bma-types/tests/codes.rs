//! The code list is a wire contract: every string is an `X_` code, and no two codes share one.

use std::collections::BTreeSet;

use bma_types::{Code, UnknownCode};

/// `^X_[A-Z0-9_]+$`, written out: the workspace has no regex crate.
fn is_code_string(text: &str) -> bool {
    text.strip_prefix("X_").is_some_and(|rest| {
        !rest.is_empty()
            && rest
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
    })
}

#[test]
fn the_shape_check_refuses_what_is_not_a_code() {
    for bad in [
        "",
        "X_",
        "HOST_DNS",
        "x_host_dns",
        "X_host",
        "X_HOST-DNS",
        "X_HOST DNS",
        " X_A",
    ] {
        assert!(!is_code_string(bad), "{bad:?} must not pass");
    }
    assert!(is_code_string("X_HOST_DNS"));
    assert!(is_code_string("X_A1"));
}

#[test]
fn every_code_is_an_x_string() {
    for code in Code::ALL {
        let text = code.as_str();
        assert!(text.starts_with("X_"), "{text} must start with X_");
        assert!(is_code_string(text), "{text} must match ^X_[A-Z0-9_]+$");
    }
}

#[test]
fn every_code_string_is_unique() {
    let strings: BTreeSet<&str> = Code::ALL.iter().map(|code| code.as_str()).collect();
    assert_eq!(strings.len(), Code::ALL.len());
    let variants: BTreeSet<Code> = Code::ALL.iter().copied().collect();
    assert_eq!(variants.len(), Code::ALL.len());
}

#[test]
fn the_list_holds_every_family() {
    let count = |prefix: &str| {
        Code::ALL
            .iter()
            .filter(|c| c.as_str().starts_with(prefix))
            .count()
    };
    assert_eq!(count("X_HOST_"), 15);
    assert_eq!(count("X_SECRET_"), 11);
    assert_eq!(count("X_SQL_"), 6);
    assert_eq!(count("X_HELPER_"), 19);
    assert_eq!(Code::ALL.len(), 51);
}

#[test]
fn display_parse_and_json_agree() {
    assert_eq!(Code::HostNotBound.to_string(), "X_HOST_NOT_BOUND");
    for code in Code::ALL {
        let text = code.to_string();
        assert_eq!(text, code.as_str());
        assert_eq!(text.parse::<Code>(), Ok(*code));
        let json = serde_json::to_string(code).unwrap();
        assert_eq!(json, format!("\"{text}\""));
        assert_eq!(serde_json::from_str::<Code>(&json).unwrap(), *code);
    }
}

#[test]
fn an_unknown_string_is_refused_and_not_echoed() {
    for bad in ["", "X_NOPE", "x_host_dns", "X_HOST_DNS ", "HostDns"] {
        assert_eq!(bad.parse::<Code>(), Err(UnknownCode));
    }
    let error = serde_json::from_str::<Code>("\"X_SECRET_VALUE_hunter2\"").unwrap_err();
    assert!(!error.to_string().contains("hunter2"));
    assert!(serde_json::from_str::<Code>("7").is_err());
}
