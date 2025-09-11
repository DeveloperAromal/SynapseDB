use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use engine::run_query;


#[no_mangle]
pub extern "C" fn execute_sql(sql: *const c_char) -> *mut c_char{
    let c_char = unsafe {CStr::from_ptr(sql)};
    let query = c_char.to_str().unwrap();

    let result = run_query(query);
    CString::new(result).unwrap().into_raw()
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

