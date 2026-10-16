use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic;
use engine::run_query;


#[no_mangle]
pub extern "C" fn execute_sql(sql: *const c_char) -> *mut c_char{
    let result = if sql.is_null() {
        "Error: no query given".to_string()
    } else {
        // SAFETY: the caller passes a valid NUL-terminated string that stays
        // alive for the duration of this call.
        match unsafe { CStr::from_ptr(sql) }.to_str() {
            // A panic must not unwind into the Go caller: that aborts the
            // whole process, server included.
            Ok(query) => panic::catch_unwind(|| run_query(query))
                .unwrap_or_else(|_| "Error: internal engine error".to_string()),
            Err(_) => "Error: query is not valid UTF-8".to_string(),
        }
    };

    to_c_string(result).into_raw()
}

#[no_mangle]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(ptr);
    }
}

// A C string cannot contain NUL bytes, so drop any instead of failing.
fn to_c_string(text: String) -> CString {
    CString::new(text).unwrap_or_else(|err| {
        let mut bytes = err.into_vec();
        bytes.retain(|&b| b != 0);
        CString::new(bytes).expect("NUL bytes were removed")
    })
}
