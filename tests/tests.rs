use chrono::prelude::*;
use goodpod::*;
use pretty_assertions::assert_eq;

#[test]
fn serialize_simple_feed() {
    let rss = Rss {
        channel: Channel {
            title: Some("My title".to_string()),
            description: Some("My description".to_string()),
            generator: Some("RSS Blue v1.0".to_string()),
            language: Some(Language::English(LanguageEnglish::UnitedStates)),

            ..Default::default()
        },
    };

    let expected = include_str!("../tests/data/simple-feed.xml").trim();
    let actual = goodpod::ser::to_string_with_config(
        &rss,
        &goodpod::ser::Config {
            perform_indent: true,
            indent_string: Some("  ".to_string()),
            write_document_declaration: true,
        },
    )
    .unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn serialize_publisher_feed() {
    let rss = Rss {
        channel: Channel {
            title: Some("John Doe".to_string()),
            description: Some("John Doe is a fictional artist.".to_string()),
            last_build_date: Some(
                chrono::prelude::Utc
                    .with_ymd_and_hms(2024, 4, 20, 0, 5, 50)
                    .unwrap(),
            ),
            itunes_image: Some(itunes::Image {
                href: "https://example.com/john-doe.jpg".to_string(),
            }),
            podcast_guid: Some(
                uuid::Uuid::parse_str("4136005d-a5b9-534d-b32a-e88894e1ae4a").unwrap(),
            ),
            podcast_medium: Some(podcast::Medium::Publisher),
            podcast_remote_items: vec![
                podcast::RemoteItem {
                    feed_guid: uuid::Uuid::parse_str("b9f0b2e5-6a56-5e73-83e4-af1c33769e73")
                        .unwrap(),
                    item_guid: None,
                    medium: Some(podcast::Medium::Music),
                },
                podcast::RemoteItem {
                    feed_guid: uuid::Uuid::parse_str("cb6e27c5-6d11-54a1-bf60-40537c0336e3")
                        .unwrap(),
                    item_guid: None,
                    medium: Some(podcast::Medium::Music),
                },
            ],
            ..Default::default()
        },
    };

    let expected = include_str!("../tests/data/publisher-feed.xml").trim();
    let actual = goodpod::ser::to_string_with_config(
        &rss,
        &goodpod::ser::Config {
            perform_indent: true,
            indent_string: Some("  ".to_string()),
            write_document_declaration: true,
        },
    )
    .unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn serialize_complex_feed() {
    let rss = Rss {
        channel: Channel {
            title: Some("My <strong>title</strong>".to_string()),
            description: Some("My description".to_string()),
            generator: Some("RSS Blue v1.0".to_string()),
            last_build_date: None,

            podcast_funding: vec![
                podcast::Funding {
                    url: url::Url::parse("https://www.example.com/donations").unwrap(),
                    display_text: "Support the show!".to_string(),
                },
                podcast::Funding {
                    url: url::Url::parse("https://www.example.com/members").unwrap(),
                    display_text: "Become a member!".to_string(),
                },
            ],
            podcast_locked: Some(podcast::Locked {
                value: false,
                owner_email: Some("john@example.com".to_string()),
            }),
            ..Default::default()
        },
    };

    let expected = include_str!("../tests/data/complex-feed.xml").trim();
    let actual = goodpod::ser::to_string_with_config(
        &rss,
        &goodpod::ser::Config {
            perform_indent: true,
            indent_string: Some("  ".to_string()),
            write_document_declaration: true,
        },
    )
    .unwrap();

    assert_eq!(expected, actual);
}
