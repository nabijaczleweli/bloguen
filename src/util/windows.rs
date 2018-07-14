use winapi::um::winnls::{LCTYPE, GetLocaleInfoEx};
use winapi::um::winnt::{LPCWSTR, WCHAR};
use winapi::um::winbase::GetUserNameW;
use winapi::shared::minwindef::DWORD;
use winapi::shared::lmcons::UNLEN;
use std::ptr;


// Stolen verbatim from winnls.h and winnt.h.
const LOCALE_NAME_USER_DEFAULT: LPCWSTR = ptr::null();
const LOCALE_NAME_MAX_LENGTH: usize = 85;
const LOCALE_SNAME: LCTYPE = 0x0000005C;


#[inline]
pub fn default_language_impl() -> Option<String> {
    let mut buf = [0 as WCHAR; LOCALE_NAME_MAX_LENGTH];
    let len = unsafe { GetLocaleInfoEx(LOCALE_NAME_USER_DEFAULT, LOCALE_SNAME, buf.as_mut_ptr(), buf.len() as i32) } as usize;
    if len != 0 {
        String::from_utf16(&buf[..len - 1]).ok()
    } else {
        None
    }
}

#[inline]
pub fn current_username_impl() -> Option<String> {
    let mut buf = [0 as WCHAR; UNLEN as usize + 1];
    let mut len = buf.len() as DWORD;
    if unsafe { GetUserNameW(buf[..].as_mut_ptr(), &mut len as *mut DWORD) } != 0 {
        String::from_utf16(&buf[..len as usize - 1]).ok()
    } else {
        None
    }
}
