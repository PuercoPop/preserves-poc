use poc::Issue;
use preserves::{de, value};
use std::fs::read_to_string;
use std::fs::File;

#[test]
fn test_deserialize_issue() {
    // Basing myself from:
    // https://gitlab.com/preserves/preserves/-/blob/01d3659216e7e0ec71ae5f6fffc20b8109c25301/implementations/rust/preserves/examples/extensibility.rs
    let file = File::open("./tests/data/issue.json").expect("Could not find file");
    let issue: Issue = de::from_read(&mut file).expect("Danger Will Robinson");
    assert_eq!(issue.key, "UNIQ-2020");
}

#[test]
fn test_deserialize_issue_from_response() {
    let input: String =
        read_to_string("./tests/data/issue.http").expect("Could not read canned HTTP response");
    let res: ureq::Response = input.parse().expect("Could not parse canned HTTP response");
    let issue: Issue = value::de::from_value(res).expect("Danger Will Robinson");
    assert_eq!(issue.key, "UNIQ-2020");
}

// TODO: Add test to parse the `create issue metadata` endpoint
