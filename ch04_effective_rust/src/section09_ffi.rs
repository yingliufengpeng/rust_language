extern crate libc;

use libc::size_t;

#[no_mangle]
pub extern "C" fn hello_rust() -> *const u8 {
    "Hello, world!\0".as_ptr()
}

// #[link(name = "snappy")]
// extern {
//     fn snappy_max_compressed_length(source_length: size_t) -> size_t;
// }

#[cfg(test)]
mod tests {
    // use crate::section09_ffi::snappy_max_compressed_length;

    use crate::section09_ffi::hello_rust;

    #[test]
    fn test_001() {
        // let x = unsafe {
        //     snappy_max_compressed_length(100)
        // };
        //
        // println!("max compressed length of a 100 byte buffer:  {}", x);
        //




    }

    #[test]
    fn test_002() {
        let r = hello_rust();
        unsafe {
            // let r = r as &str;
            // println!("{:?}", r);
        }

    }

}