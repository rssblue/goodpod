use chrono::prelude::*;
use goodpod::*;
use pretty_assertions::assert_eq;

#[test]
fn serialize_simple_feed() {
    let rss = Rss {
        channel: Channel {
            title: Some("My <strong>title</strong>".to_string()),
            description: Some("My description".to_string()),
            generator: Some("RSS Blue v1.0".to_string()),
            last_build_date: None,
            ..Default::default()
        },
    };

    let expected = include_str!("../tests/data/simple-feed.xml").trim();
    let actual = yaserde::ser::to_string_with_config(
        &rss,
        &yaserde::ser::Config {
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
                // Construct using year, month, day, hour, minute, second
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
            podcast_medium: Some("publisher".to_string()),
            podcast_remote_items: vec![
                podcast::RemoteItem {
                    feed_guid: uuid::Uuid::parse_str("b9f0b2e5-6a56-5e73-83e4-af1c33769e73")
                        .unwrap(),
                    item_guid: None,
                    medium: Some("music".to_string()),
                },
                podcast::RemoteItem {
                    feed_guid: uuid::Uuid::parse_str("cb6e27c5-6d11-54a1-bf60-40537c0336e3")
                        .unwrap(),
                    item_guid: None,
                    medium: Some("music".to_string()),
                },
            ],
            ..Default::default()
        },
    };

    let expected = include_str!("../tests/data/publisher-feed.xml").trim();
    let actual = yaserde::ser::to_string_with_config(
        &rss,
        &yaserde::ser::Config {
            perform_indent: true,
            indent_string: Some("  ".to_string()),
            write_document_declaration: true,
        },
    )
    .unwrap();

    assert_eq!(expected, actual);
}
