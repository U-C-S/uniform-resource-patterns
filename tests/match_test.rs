use uri_globbing::{is_match, to_regex_str};

#[test]
fn simple_exp() {
    // let result = is_match("world-big-cat", String::from("world-*-cat"));
    // assert_eq!(result, true);

    let test_gen_map = [
        ["world-big-cat", "world-*-cat"],
        ["/meow/h/ja/ddd/ada/dad", "/**"],
    ];

    for [sample, pattern] in test_gen_map {
        assert_eq!(is_match(sample, pattern), true)
    }
}

#[test]
fn escape_char_test() {
    assert_eq!(to_regex_str(r"meow\?"), String::from("^meow?$"));

    // assert_eq!("meow\\?".len(), 7)
}

#[test]
fn test_range_parsing() {
    assert_eq!(to_regex_str("[a-z]*"), String::from("^[a-z].*$"));

    assert_eq!(to_regex_str("[0-9]?"), String::from("^[0-9].$"));

    assert_eq!(to_regex_str("file[abc].txt"), String::from("^file[abc].txt$"));

    // Malformed range should panic
    let result = std::panic::catch_unwind(|| to_regex_str("[a-z"));
    assert!(result.is_err());
}

#[test]
fn test_lists() {
    assert_eq!(
        to_regex_str("{super,spider,iron}man"),
        "^(?:super|spider|iron)man$"
    );
    assert_eq!(is_match("superman", "{super,spider,iron}man$"), true)
}
