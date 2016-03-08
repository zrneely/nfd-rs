use libc::c_char;

#[allow(dead_code)]
#[repr(C)]
pub enum nfdresult_t {
    NFD_ERROR,
    NFD_OKAY,
    NFD_CANCEL
}

#[link(name = "nfd")]
extern {
    pub fn NFD_OpenDialog(filter_list: *const c_char, default_path: *const c_char, outPath: *mut *mut c_char) -> nfdresult_t;
    pub fn NFD_SaveDialog(filter_list: *const c_char, default_path: *const c_char, outPath: *mut *mut c_char) -> nfdresult_t;

    pub fn NFD_GetError() -> *const c_char;
}
