use libc::*;

use *;

extern "C" {
    pub fn OBJ_nid2ln(nid: c_int) -> *const c_char;
    pub fn OBJ_nid2sn(nid: c_int) -> *const c_char;
    pub fn OBJ_nid2obj(n: c_int) -> *mut ASN1_OBJECT;
    pub fn OBJ_obj2nid(o: *const ASN1_OBJECT) -> c_int;
    pub fn OBJ_obj2txt(
        buf: *mut c_char,
        buf_len: c_int,
        a: *const ASN1_OBJECT,
        no_name: c_int,
    ) -> c_int;

    pub fn OBJ_find_sigid_algs(signid: c_int, pdig_nid: *mut c_int, ppkey_nid: *mut c_int)
        -> c_int;
    pub fn OBJ_sn2nid(sn: *const libc::c_char) -> libc::c_int;
    pub fn OBJ_txt2obj(s: *const libc::c_char, no_name: libc::c_int) -> *mut ASN1_OBJECT;
}