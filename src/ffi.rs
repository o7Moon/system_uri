// Copyright 2018 MaidSafe.net limited.
//
// This SAFE Network Software is licensed to you under the MIT license <LICENSE-MIT
// http://opensource.org/licenses/MIT> or the Modified BSD license <LICENSE-BSD
// https://opensource.org/licenses/BSD-3-Clause>, at your option. This file may not be copied,
// modified, or distributed except according to those terms. Please review the Licences for the
// specific language governing permissions and limitations relating to use of the SAFE Network
// Software.

#![allow(unsafe_code)]

use super::errors::Error;
use super::{install as rust_install, open as rust_open, App};
use ffi_utils::{
    catch_unwind_cb, vec_clone_from_raw_parts, ErrorCode, FfiResult, FFI_RESULT_OK,
};

use libc::c_char;
use std::ffi::CStr;
use std::os::raw::c_void;

/// Open the given URI on this system.
#[no_mangle]
pub unsafe extern "C" fn open_uri(
    uri: *const c_char,
    user_data: *mut c_void,
    o_cb: extern "C" fn(*mut c_void, *const FfiResult),
) {
    catch_unwind_cb(user_data, o_cb, || -> Result<(), Error> {
        let uri = CStr::from_ptr(uri).to_str()?.to_owned();
        rust_open(uri)?;
        o_cb(user_data, FFI_RESULT_OK);
        Ok(())
    })
}

/// Install the given App definition for each scheme URI on the system.
/// Schemes are a comma delimited list of schemes.
#[no_mangle]
pub unsafe extern "C" fn install(
    bundle: *const c_char,
    vendor: *const c_char,
    name: *const c_char,
    exec_args: *const *const c_char,
    exec_args_len: usize,
    icon: *const c_char,
    schemes: *const c_char,
    user_data: *mut c_void,
    o_cb: extern "C" fn(*mut c_void, *const FfiResult),
) {
    catch_unwind_cb(user_data, o_cb, || -> Result<(), Error> {
        let mut exec = String::new();
        let args = vec_clone_from_raw_parts(exec_args, exec_args_len);
        for arg in args {
            let arg_str = format!("\"{}\" ", CStr::from_ptr(arg).to_str()?);
            exec.push_str(&arg_str);
        }
        let app = App::new(
            CStr::from_ptr(bundle).to_str()?.to_owned(),
            CStr::from_ptr(vendor).to_str()?.to_owned(),
            CStr::from_ptr(name).to_str()?.to_owned(),
            exec.trim_end().to_owned(),
            Some(CStr::from_ptr(icon).to_str()?.to_owned()),
        );

        let schemes_str = CStr::from_ptr(schemes).to_str()?.to_owned();

        rust_install(
            &app,
            &schemes_str
                .split(',')
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        )?;
        o_cb(user_data, FFI_RESULT_OK);
        Ok(())
    })
}

impl ErrorCode for Error {
    fn error_code(&self) -> i32 {
        -1
    }
}
