use libc::{c_char, size_t, c_int};
use std::ffi::CStr;
use std::env;


include!(concat!(env!("OUT_DIR"), "/errno-data/errno.rs"));

extern "C" {
    fn getlogin_r(buf: *mut c_char, bufsize: size_t) -> c_int;
}


#[inline]
pub fn default_language_impl() -> Option<String> {
    for v in &["LANG", "LANGUAGE", "LC_NAME"] {
        let out = env::var(v).ok().and_then(|l| {
            let main = if let Some(i) = l.find('.') {
                &l[..i]
            } else {
                &l
            };

            if !["", "C", "POSIX"].contains(&main) {
                Some(main.replace('_', "-"))
            } else {
                None
            }
        });
        if out.is_some() {
            return out;
        }
    }

    None
}

#[inline]
pub fn current_username_impl() -> Option<String> {
    let mut len = 16;
    while len <= 256 {
        let mut buf = vec![0 as c_char; len + 1];
        match unsafe { getlogin_r(buf[..].as_mut_ptr(), buf.len()) } {
            0 => {
                match unsafe { CStr::from_ptr(buf.as_ptr()) }.to_str() {
                    Ok(s) => return Some(s.to_string()),
                    Err(_) => break,
                }
            }
            ERANGE => len *= 2,
            _ => break,
        }
    }

    env::var("USER").ok()
}
