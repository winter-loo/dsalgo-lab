pub struct Codec;

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    pub fn new() -> Self {
        Codec {}
    }
	
    // Encodes a list of strings to a single string.
    pub fn encode(&self, strs: Vec<String>) -> String {
        let mut encoded = String::new();
        for s in strs {
            encoded.push_str(&s);
            encoded.push_str("\0");
        }
        encoded
    }
	
    // Decodes a single string to a list of strings.
    pub fn decode(&self, s: String) -> Vec<String> {
        s.split("\0").map(|s| s.to_string()).collect()
    }
}
