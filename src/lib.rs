use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn add(_a: i32, _b: i32) -> *mut c_char {
    let result = "TESTSTRING AUS RUST";

    let c_string = CString::new(result).unwrap();

    // In einen rohen Pointer umwandeln (Ownership geht an C# über!)
    c_string.into_raw()
}