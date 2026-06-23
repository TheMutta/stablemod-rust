pub fn fill_bytes(bytes: &mut [u8]) {
    for b in bytes.iter_mut() {
        *b = 0;
    }
}
