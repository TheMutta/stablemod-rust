use crate::io;

pub fn errno() -> i32 {
    // Return your kernel's current error variable
    0
}

pub fn error_string(errno: i32) -> crate::string::String {
    crate::format!("OS error {}", errno)
}

pub fn decode_error_kind(errno: i32) -> io::ErrorKind {
    match errno {
        _ => io::ErrorKind::Other,
    }
}

pub fn is_interrupted(e: &io::Error) -> bool {
    false
}
