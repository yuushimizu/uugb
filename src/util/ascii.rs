pub fn from_bytes(bytes: &[u8]) -> String {
    bytes.iter().map(|x| *x as char).collect()
}
