pub(crate) fn s(bs: &[u8]) -> &str {
    return str::from_utf8(bs).unwrap();
}