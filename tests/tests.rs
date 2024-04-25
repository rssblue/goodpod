use goodpod::*;
use pretty_assertions::assert_eq;

#[test]
fn serialize_simple_feed() {
    let rss = Rss {
        channel: Channel {
            title: "My title".to_string(),
            description: "My description".to_string(),
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
