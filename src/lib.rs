extern crate libc;

mod ffi;

use libc::c_char;
use std::ffi::*;
use ffi::*;

/// Result of opening a file dialog
pub enum NFDResult {
    /// User pressed okay. `String` is the file path selected
    Okay(String),
    /// User pressed cancel
    Cancel,
    /// Program error. `String` is the error description
    Error(String),
}

enum DialogType {
    SingleFile,
    SaveFile
}

/// Open single file dialog
#[inline(always)]
pub fn open_file_dialog(filter_list: &str, default_path: &str) -> NFDResult {
    open_dialog(filter_list, default_path, &DialogType::SingleFile)
}

/// Open save dialog
#[inline(always)]
pub fn open_save_dialog(filter_list: &str, default_path: &str) -> NFDResult {
    open_dialog(filter_list, default_path, &DialogType::SaveFile)
}

fn open_dialog(filter_list: &str, default_path: &str, dialog_type: &DialogType) -> NFDResult {

    let mut final_cstring = CString::new("").unwrap();
    let out_path = CString::new("").unwrap().into_raw() as *mut *mut c_char;
    let result: nfdresult_t;

    unsafe {
        result = match dialog_type {
            &DialogType::SingleFile => {
                NFD_OpenDialog(
                CString::new(filter_list).unwrap().as_ptr(),
                CString::new(default_path).unwrap().as_ptr(),
                out_path)
            },

            &DialogType::SaveFile => {
                NFD_SaveDialog(
                CString::new(filter_list).unwrap().as_ptr(),
                CString::new(default_path).unwrap().as_ptr(),
                out_path)
            },
        };

        match result {
            nfdresult_t::NFD_OKAY => {
                final_cstring = CString::from_raw(*out_path);
            },
            nfdresult_t::NFD_ERROR => {
                final_cstring = CStr::from_ptr(NFD_GetError()).to_owned();
            }
            _ => {},
        }
    }

    let final_string = final_cstring.to_str().unwrap().to_string();

    match result {
        nfdresult_t::NFD_OKAY => NFDResult::Okay(final_string),
        nfdresult_t::NFD_CANCEL => NFDResult::Cancel,
        nfdresult_t::NFD_ERROR => NFDResult::Error(final_string)
    }
}
