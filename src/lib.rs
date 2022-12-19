#[cfg(windows)]
#[no_mangle]
extern "C" {
    fn x11_hash(input: *const std::os::raw::c_char, output: *mut std::os::raw::c_char);
}

#[cfg(not(windows))]
#[allow(unused_attributes)]
#[no_mangle]
extern "C" {
    fn x11_hash(input: *const std::os::raw::c_char, output: *mut std::os::raw::c_char);
}

pub fn get_x11_hash<T: AsRef<[u8]>>(input: T) -> Vec<u8> {
    assert_eq!(input.as_ref().len(), 80);
    unsafe {
        let input_str = std::ffi::CStr::from_bytes_with_nul_unchecked(input.as_ref());
        let buffer = [0u8; 32].to_vec();
        let ptr = std::ffi::CString::from_vec_unchecked(buffer).into_raw();
        x11_hash(input_str.as_ptr(), ptr);
        let data = std::ffi::CString::from_raw(ptr);
        let bytes = data.as_bytes();
        let len = std::cmp::min(32, bytes.len());
        let mut result = [0u8; 32].to_vec();
        result[0..len].clone_from_slice(&bytes[0..len]);
        result
    }
}



#[cfg(test)]
mod tests {
    use hex::{FromHex, ToHex};
    use crate::get_x11_hash;

    #[test]
    fn test_x11_hash() {
        let x11 = "020000002cc0081be5039a54b686d24d5d8747ee9770d9973ec1ace02e5c0500000000008d7139724b11c52995db4370284c998b9114154b120ad3486f1a360a1d4253d310d40e55b8f70a1be8e32300";
        let x11_vec = Vec::from_hex(x11).unwrap();
        let md = get_x11_hash(x11_vec);
        println!("input: {}", x11);
        println!("output: {:?}", md.encode_hex::<String>());
        assert_eq!(md, Vec::from_hex("f29c0f286fd8071669286c6987eb941181134ff5f3978bf89f34070000000000").unwrap())
    }

}
