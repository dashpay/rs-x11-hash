#[cfg(windows)]
#[no_mangle]
extern "C" {
    fn x11_hash(input: *const u8, output: *mut u8);
}

#[cfg(not(windows))]
#[allow(unused_attributes)]
#[no_mangle]
extern "C" {
    fn x11_hash(input: *const u8, output: *mut u8);
}

pub fn get_x11_hash<T: AsRef<[u8]>>(input: T) -> [u8; 32] {
    unsafe {
        let mut buffer = [0u8; 32];
        x11_hash(input.as_ref().as_ptr(), buffer.as_mut_ptr());
        buffer
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
        assert_eq!(md.to_vec(), Vec::from_hex("f29c0f286fd8071669286c6987eb941181134ff5f3978bf89f34070000000000").unwrap())
    }

    #[test]
    fn test_x11_hash_2() {
        let input = Vec::from_hex("040000002e3df23eec5cd6a86edd509539028e2c3a3dc05315eb28f2baa43218ca080000b3a56d65316ffdb006163240a4380e94a4c2d8c0f0b3b2c1ddc486fae15ed065ba968054ffff7f2002000000").unwrap();
        let output = get_x11_hash(input);
        assert_eq!("000739d9da507b3acb949f21fe10ad424abbad5b4c46789285b05fe36df5c5b0", output.encode_hex::<String>(), "x11 error");

        let input = Vec::from_hex("040000002e3df23eec5cd6a86edd509539028e2c3a3dc05315eb28f2baa43218ca080000b3a56d65316ffdb006163240a4380e94a4c2d8c0f0b3b2c1ddc486fae15ed065ba968054ffff7f2003000000").unwrap();
        let output = get_x11_hash(input);
        assert_eq!("90ec0543cd91297e7ad3d3141a404fb55f787b3058aca2b45ab0fc20d06409c6", output.encode_hex::<String>(), "x11 error");

        let input = Vec::from_hex("040000002e3df23eec5cd6a86edd509539028e2c3a3dc05315eb28f2baa43218ca080000b3a56d65316ffdb006163240a4380e94a4c2d8c0f0b3b2c1ddc486fae15ed065ba968054ffff7f2004000000").unwrap();
        let output = get_x11_hash(input);
        assert_eq!("eee8ff78056e3b0cd35cd8e267fa871270a183a5d05c764d8c2047b7c3cca014", output.encode_hex::<String>(), "x11 error");
    }
}
