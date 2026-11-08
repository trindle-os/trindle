//! The checks made through PAM: a typed password, and Pocketkey's yes from the phone.

use std::ffi::{c_char, c_int, c_void, CStr, CString};

use zeroize::Zeroize;

use crate::common::pocketkey;

const PAM_CONV_ERR: c_int = 19;
const PAM_BUF_ERR: c_int = 5;

const MOST_MESSAGES: c_int = 32;

#[repr(C)]
struct PamMessage {
    style: c_int,
    msg: *const c_char,
}
#[repr(C)]
struct PamResponse {
    resp: *mut c_char,
    retcode: c_int,
}
#[repr(C)]
struct PamConv {
    conv: extern "C" fn(c_int, *mut *const PamMessage, *mut *mut PamResponse, *mut c_void) -> c_int,
    appdata: *mut c_void,
}

#[link(name = "libpam.so.0", kind = "dylib", modifiers = "+verbatim")]
extern "C" {
    fn pam_start(service: *const c_char, user: *const c_char, conv: *const PamConv, handle: *mut *mut c_void) -> c_int;
    fn pam_authenticate(handle: *mut c_void, flags: c_int) -> c_int;
    fn pam_end(handle: *mut c_void, status: c_int) -> c_int;
}

extern "C" fn converse(n: c_int, msgs: *mut *const PamMessage, out: *mut *mut PamResponse, data: *mut c_void) -> c_int {
    if n <= 0 || n > MOST_MESSAGES {
        return PAM_CONV_ERR;
    }
    // SAFETY: PAM promises `msgs` points to `n` message pointers and that

    unsafe {
        let replies = libc::calloc(n as usize, std::mem::size_of::<PamResponse>()) as *mut PamResponse;
        if replies.is_null() {
            return PAM_BUF_ERR;
        }
        let password = data as *const c_char;
        for i in 0..n as usize {
            let m = *msgs.add(i);
            if !m.is_null() && (*m).style == 1 {

                (*replies.add(i)).resp = libc::strdup(password);
            }
        }
        *out = replies;
    }
    0
}

pub(crate) fn password_ok(user: &str, password: &str) -> bool {
    let Ok(pw) = CString::new(password) else { return false };
    let ok = authenticate(user, &PamConv { conv: converse, appdata: pw.as_ptr() as *mut c_void });
    pw.into_bytes_with_nul().zeroize();
    ok
}

unsafe fn is_question(m: *const PamMessage) -> bool {
    // SAFETY: as the caller promised: `m` is valid and its text null or NUL-terminated.
    unsafe { !(*m).msg.is_null() && CStr::from_ptr((*m).msg) == pocketkey::QUESTION }
}

extern "C" fn converse_phone(n: c_int, msgs: *mut *const PamMessage, out: *mut *mut PamResponse, _: *mut c_void) -> c_int {
    if n <= 0 || n > MOST_MESSAGES {
        return PAM_CONV_ERR;
    }
    // SAFETY: PAM promises `msgs` points to `n` message pointers, each either

    unsafe {
        for i in 0..n as usize {
            let m = *msgs.add(i);
            if m.is_null() || ((*m).style == 1 && !is_question(m)) {
                return PAM_CONV_ERR;
            }
        }
        let replies = libc::calloc(n as usize, std::mem::size_of::<PamResponse>()) as *mut PamResponse;
        if replies.is_null() {
            return PAM_BUF_ERR;
        }
        for i in 0..n as usize {
            let m = *msgs.add(i);

            if (*m).style == 2 && is_question(m) || (*m).style == 1 {
                (*replies.add(i)).resp = libc::strdup(c"yes".as_ptr());
            }
        }
        *out = replies;
    }
    0
}

pub(crate) fn phone_ok(user: &str) -> bool {
    authenticate(user, &PamConv { conv: converse_phone, appdata: std::ptr::null_mut() })
}

fn authenticate(user: &str, conv: &PamConv) -> bool {
    let (Ok(service), Ok(user)) = (CString::new("trindle-lock"), CString::new(user)) else {
        return false;
    };
    let mut handle: *mut c_void = std::ptr::null_mut();
    // SAFETY: the CStrings and `conv`, with whatever its appdata points at,

    unsafe {
        if pam_start(service.as_ptr(), user.as_ptr(), conv, &mut handle) != 0 {
            return false;
        }
        let status = pam_authenticate(handle, 0);
        pam_end(handle, status);
        status == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::pocketkey;

    fn ask(msgs: &[(c_int, Option<&CStr>)]) -> (c_int, Vec<Option<String>>) {
        let owned: Vec<PamMessage> =
            msgs.iter().map(|(style, m)| PamMessage { style: *style, msg: m.map_or(std::ptr::null(), |m| m.as_ptr()) }).collect();
        let ptrs: Vec<*const PamMessage> = owned.iter().map(|m| m as *const PamMessage).collect();
        let mut out: *mut PamResponse = std::ptr::null_mut();
        let rc = converse_phone(ptrs.len() as c_int, ptrs.as_ptr() as *mut _, &mut out, std::ptr::null_mut());
        let mut answers = Vec::new();
        if !out.is_null() {
            // SAFETY: on success converse_phone calloc'd one reply per message,

            unsafe {
                for i in 0..msgs.len() {
                    let r = (*out.add(i)).resp;
                    answers.push((!r.is_null()).then(|| CStr::from_ptr(r).to_string_lossy().into_owned()));
                    libc::free(r as *mut c_void);
                }
                libc::free(out as *mut c_void);
            }
        }
        (rc, answers)
    }

    #[test]
    fn the_phone_only_ever_says_yes_to_pocketkey() {
        assert_eq!(ask(&[(2, Some(pocketkey::QUESTION))]), (0, vec![Some("yes".into())]));

        assert_eq!(ask(&[(4, Some(c"Touch your phone")), (2, Some(pocketkey::QUESTION))]), (0, vec![None, Some("yes".into())]));

        assert_eq!(ask(&[(2, Some(c"Trindle Pocketkey: use your phone? (yes/no) "))]), (0, vec![None]));
        assert_eq!(ask(&[(2, Some(c"Username:"))]), (0, vec![None]));
        assert_eq!(ask(&[(2, None)]), (0, vec![None]));

        assert_eq!(ask(&[(1, Some(c"Password:"))]), (PAM_CONV_ERR, vec![]));
        assert_eq!(ask(&[(2, Some(pocketkey::QUESTION)), (1, Some(c"Password:"))]), (PAM_CONV_ERR, vec![]));
        assert_eq!(ask(&[]), (PAM_CONV_ERR, vec![]));
    }

    #[test]
    fn without_the_pocketkey_module_the_lock_stack_is_the_password_alone() {
        let file = std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/system/pam.d/trindle-lock")).unwrap();
        let lines: Vec<&str> = file.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')).collect();

        let (module, rest): (Vec<&str>, Vec<&str>) = lines.into_iter().partition(|l| l.contains("pam_pocketkey.so"));
        assert!(module.iter().all(|l| l.starts_with("-auth ")), "{module:?}");
        assert_eq!(rest, ["auth include login"]);
    }
}
