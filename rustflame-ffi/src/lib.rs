use std::ffi::CString;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::mem;
use std::os::raw::c_char;
use std::os::windows::ffi::OsStrExt;

#[cfg(test)]
mod tests;

#[no_mangle]
pub extern "cdecl" fn hello_world() {
    println!("Hello from rust!");
}

#[no_mangle]
pub extern "cdecl" fn return_string_utf8() -> *const u16 {
    let mut vec: Vec<u16> = OsStr::new("This is a string from rust!\0")
        .encode_wide()
        .collect();
    let r = vec.as_ptr();
    mem::forget(vec);
    r
}

#[no_mangle]
pub extern "cdecl" fn return_image_rg_24bpp(width: u32, height: u32) -> *const u8 {
    println!(
        "starting the image generation. width: {} height: {}",
        width, height
    );
    let mut v = Vec::<u8>::with_capacity((width * height * 3) as usize);
    for i in 0..width {
        for j in 0..height {
            let r = (i % 255) as u8;
            let g = (j % 255) as u8;
            v.push(r);
            v.push(g);
            v.push(0);
        }
    }
    println!(
        "{} {} {}, {} {} {}, {} {} {}",
        v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8]
    );
    let ret = v.as_ptr();
    mem::forget(v);
    ret
}
