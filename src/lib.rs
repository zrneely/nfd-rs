/*
Copyright (c) 2016 Saurav Sachidanand

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
*/

extern crate libc;

mod ffi;

use ffi::*;
use libc::c_char;
use std::ffi::*;

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
pub fn open_file_dialog(filter_list: Option<&str>, default_path: Option<&str>) -> NFDResult {
    open_dialog(filter_list, default_path, &DialogType::SingleFile)
}

/// Open save dialog
#[inline(always)]
pub fn open_save_dialog(filter_list: Option<&str>, default_path: Option<&str>) -> NFDResult {
    open_dialog(filter_list, default_path, &DialogType::SaveFile)
}

fn open_dialog(filter_list: Option<&str>, default_path: Option<&str>, dialog_type: &DialogType) -> NFDResult {
    let result: nfdresult_t;
    let result_cstring;

    let filter_list_cstring;
    let filter_list_ptr = match filter_list {
        Some(fl_str) => {
            filter_list_cstring = CString::new(fl_str).unwrap();
            filter_list_cstring.as_ptr()
        }
        None => std::ptr::null()
    };

    let default_path_cstring;
    let default_path_ptr = match default_path {
        Some(dp_str) => {
            default_path_cstring = CString::new(dp_str).unwrap();
            default_path_cstring.as_ptr()
        }
        None => std::ptr::null()
    };

    let mut out_path: *mut c_char = std::ptr::null_mut();
    let ptr_out_path = &mut out_path as *mut *mut c_char;

    unsafe {
        result = match dialog_type {
            &DialogType::SingleFile => {
                NFD_OpenDialog(filter_list_ptr, default_path_ptr, ptr_out_path)
            },

            &DialogType::SaveFile => {
                NFD_SaveDialog(filter_list_ptr, default_path_ptr, ptr_out_path)
            },
        };

        result_cstring = match result {
            nfdresult_t::NFD_OKAY => CStr::from_ptr(out_path).to_owned(),
            nfdresult_t::NFD_ERROR => CStr::from_ptr(NFD_GetError()).to_owned(),
            _ => CString::new("").unwrap()
        }
    }

    let result_string = result_cstring.to_str().unwrap().to_string();

    match result {
        nfdresult_t::NFD_OKAY => NFDResult::Okay(result_string),
        nfdresult_t::NFD_CANCEL => NFDResult::Cancel,
        nfdresult_t::NFD_ERROR => NFDResult::Error(result_string)
    }
}
