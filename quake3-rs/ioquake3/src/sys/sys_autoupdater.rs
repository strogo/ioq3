use ::libc;
/*
The code in this file is in the public domain. The rest of ioquake3
is licensed under the GPLv2. Do not mingle code, please!
*/
#[no_mangle]

pub unsafe extern "C" fn Sys_LaunchAutoupdater(mut argc: i32, mut argv: *mut *mut i8) {
    /* possibly unused. Pacify compilers. */
}
