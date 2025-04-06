use encode_and_decode_strings::Codec;

#[test]
fn test_example_1() {
    let codec = Codec::new();
    let strs = vec!["Hello".to_string(), "World".to_string()];
    let encoded = codec.encode(strs.clone());
    let decoded = codec.decode(encoded);
    assert_eq!(decoded, strs);
}

#[test]
fn test_example_2() {
    let codec = Codec::new();
    let strs = vec!["".to_string()];
    let encoded = codec.encode(strs.clone());
    let decoded = codec.decode(encoded);
    assert_eq!(decoded, strs);
}

#[test]
fn test_empty_list() {
    let codec = Codec::new();
    let strs: Vec<String> = vec![];
    let encoded = codec.encode(strs.clone());
    let decoded = codec.decode(encoded);
    assert_eq!(decoded, strs);
}

#[test]
fn test_special_characters() {
    let codec = Codec::new();
    let strs = vec![
        "Hello\nWorld".to_string(),
        "Special#Characters".to_string(),
        "With,Comma".to_string(),
        "With:Colon".to_string(),
    ];
    let encoded = codec.encode(strs.clone());
    let decoded = codec.decode(encoded);
    assert_eq!(decoded, strs);
}

#[test]
fn test_multiple_empty_strings() {
    let codec = Codec::new();
    let strs = vec!["".to_string(), "".to_string(), "".to_string()];
    let encoded = codec.encode(strs.clone());
    let decoded = codec.decode(encoded);
    assert_eq!(decoded, strs);
}

#[test]
fn test_mixed_strings() {
    let codec = Codec::new();
    let strs = vec![
        "".to_string(),
        "Hello".to_string(),
        "".to_string(),
        "World".to_string(),
        "".to_string(),
    ];
    let encoded = codec.encode(strs.clone());
    let decoded = codec.decode(encoded);
    assert_eq!(decoded, strs);
}
