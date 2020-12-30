use mbedtls_sys::debug_set_threshold;

pub fn debug_set_level(level: i32) {
    unsafe {
        debug_set_threshold(level);
    }
}

