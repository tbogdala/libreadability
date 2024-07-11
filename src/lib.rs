use std::ffi::{CString, CStr};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn parse_html(input: *const c_char) -> *mut c_char {
    // Convert the input pointer to a Rust string
    let input_str = unsafe { CStr::from_ptr(input) }.to_string_lossy().into_owned();

    let mut readability = readable_readability::Readability::new();
    let (node, _metadata) = readability.parse(&input_str);
    let output_str = node.to_string();
    
    // Convert the output string to a C string and return its pointer
    let output_c_str = CString::new(output_str).unwrap();
    output_c_str.into_raw()
}

#[no_mangle]
pub extern "C" fn free_parsed_html(input: *mut c_char) -> i32 {
    if !input.is_null() {
        unsafe {
            drop(CString::from_raw(input));
        }
    }
    return 0;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
