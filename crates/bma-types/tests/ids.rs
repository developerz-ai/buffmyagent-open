//! Identifiers have one spelling, and a refused string is never echoed.

use bma_types::{ConnectionId, IdError, ItemId, OrgId, ProjectRef, ProjectRefError, RequestId};

const UUID: &str = "0192f0c0-1111-7000-8000-000000000001";

#[test]
fn an_identifier_is_refused_unless_it_is_the_canonical_form() {
    let cases = [
        ("", IdError::Length),
        ("0192f0c0-1111-7000-8000-00000000000", IdError::Length),
        ("0192f0c0-1111-7000-8000-0000000000011", IdError::Length),
        ("0192f0c011117000800000000000000001xx", IdError::Format),
        ("0192F0C0-1111-7000-8000-000000000001", IdError::Format),
        ("0192f0c0_1111-7000-8000-000000000001", IdError::Format),
        ("0192f0c0-1111-7000-8000-00000000000g", IdError::Format),
        ("0192f0c0--111-7000-8000-000000000001", IdError::Format),
        ("{0192f0c0-1111-7000-8000-0000000001}", IdError::Format),
        ("0192f0c0-1111-7000-8000-00000000000é", IdError::Length),
    ];
    for (text, error) in cases {
        assert_eq!(text.parse::<OrgId>(), Err(error), "{text:?}");
    }
}

#[test]
fn an_identifier_round_trips_through_text_bits_and_json() {
    let org: OrgId = UUID.parse().unwrap();
    assert_eq!(org.to_string(), UUID);
    assert_eq!(org.as_u128(), 0x0192_f0c0_1111_7000_8000_0000_0000_0001);
    assert_eq!(OrgId::from_u128(org.as_u128()), org);
    assert_eq!(format!("{org:?}"), format!("OrgId({UUID})"));
    assert_eq!(
        OrgId::from_u128(0).to_string(),
        "00000000-0000-0000-0000-000000000000"
    );
    assert_eq!(
        OrgId::from_u128(u128::MAX).to_string(),
        "ffffffff-ffff-ffff-ffff-ffffffffffff"
    );

    let json = serde_json::to_string(&org).unwrap();
    assert_eq!(json, format!("\"{UUID}\""));
    assert_eq!(serde_json::from_str::<OrgId>(&json).unwrap(), org);

    // Every kind parses the same text and is its own type.
    assert_eq!(UUID.parse::<ConnectionId>().unwrap().to_string(), UUID);
    assert_eq!(UUID.parse::<ItemId>().unwrap().to_string(), UUID);
    assert_eq!(UUID.parse::<RequestId>().unwrap().to_string(), UUID);
}

#[test]
fn a_refused_identifier_is_not_echoed() {
    let error = serde_json::from_str::<ItemId>("\"hunter2-is-not-an-id\"").unwrap_err();
    assert!(!error.to_string().contains("hunter2"));
    assert!(serde_json::from_str::<ItemId>("12").is_err());
}

#[test]
fn a_project_reference_is_refused_unless_it_is_two_slugs() {
    let cases = [
        ("", ProjectRefError::Shape),
        ("acme", ProjectRefError::Shape),
        ("acme/app/extra", ProjectRefError::Shape),
        ("/app", ProjectRefError::Slug),
        ("acme/", ProjectRefError::Slug),
        ("a/app", ProjectRefError::Slug),
        ("Acme/app", ProjectRefError::Slug),
        ("acme/-app", ProjectRefError::Slug),
        ("acme/app-", ProjectRefError::Slug),
        ("acme/my--app", ProjectRefError::Slug),
        ("acme/my_app", ProjectRefError::Slug),
        ("acme/my app", ProjectRefError::Slug),
        ("acme/caf\u{e9}", ProjectRefError::Slug),
        (
            "acme/aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            ProjectRefError::Slug,
        ),
    ];
    for (text, error) in cases {
        assert_eq!(text.parse::<ProjectRef>(), Err(error), "{text:?}");
    }
}

#[test]
fn a_project_reference_round_trips() {
    let longest = "a".repeat(39);
    for text in [
        "acme/app",
        "personal/my-app",
        "a1/b2",
        &format!("{longest}/{longest}"),
    ] {
        let project: ProjectRef = text.parse().unwrap();
        assert_eq!(project.to_string(), text);
        let json = serde_json::to_string(&project).unwrap();
        assert_eq!(serde_json::from_str::<ProjectRef>(&json).unwrap(), project);
    }
    let project: ProjectRef = "acme/my-app".parse().unwrap();
    assert_eq!(project.org(), "acme");
    assert_eq!(project.project(), "my-app");
    assert_eq!(format!("{project:?}"), "ProjectRef(acme/my-app)");

    let error = serde_json::from_str::<ProjectRef>("\"hunter2 / nope\"").unwrap_err();
    assert!(!error.to_string().contains("hunter2"));
}
