extern crate libc;

use libc::{c_char, c_int, c_ulong};
use std::ffi::CStr;
use std::path::Path;

extern "C" {
    pub fn OpenSSL_version_num() -> c_ulong;
    pub fn OPENSSL_info(t: c_int) -> *const c_char;
}

#[test]
fn version_works() {
    unsafe {
        println!("{:#x}", OpenSSL_version_num());
        assert!(OpenSSL_version_num() > 0);
    }
}

#[test]
fn compiled_in_directories_are_outside_the_build_directory() {
    // OPENSSL_INFO_CONFIG_DIR, OPENSSL_INFO_ENGINES_DIR and
    // OPENSSL_INFO_MODULES_DIR from <openssl/crypto.h>.
    let directories = [(1001, "OPENSSLDIR"), (1002, "ENGINESDIR"), (1003, "MODULESDIR")];
    // OpenSSL is built and installed inside this crate's OUT_DIR.
    let out_dir = Path::new(env!("OUT_DIR"));
    for &(t, name) in directories.iter() {
        let dir = unsafe { OPENSSL_info(t) };
        assert!(!dir.is_null(), "OPENSSL_info gives no {}", name);
        let dir = unsafe { CStr::from_ptr(dir) }.to_str().unwrap();
        println!("{}: {}", name, dir);
        assert!(
            !Path::new(dir).starts_with(out_dir),
            "{} is {}, inside the build directory {}",
            name,
            dir,
            out_dir.display()
        );
    }
}
