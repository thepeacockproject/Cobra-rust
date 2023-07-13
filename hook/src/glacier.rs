use std::os::raw::c_char;

#[repr(C)]
pub struct ZString {
    pub m_length: u32,
    pub m_chars: *const c_char,
}
