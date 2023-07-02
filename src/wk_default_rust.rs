//! This is equivalent to code written in `wk-v1-impl.c`.
//!
use crate::bindings::wk::*;
use libR_sys::*;

#[repr(C)]
pub struct wk_handler_run_data {
    pub read_fun: ::std::option::Option<
        unsafe extern "C" fn(read_data: SEXP, handler: *mut wk_handler_t) -> SEXP,
    >,
    pub read_data: SEXP,
    pub handler: *mut wk_handler_t,
}

#[no_mangle]
pub unsafe extern "C" fn wk_default_handler_initialize(
    dirty: *mut ::std::os::raw::c_int,
    _handler_data: *mut ::std::os::raw::c_void,
) {
    if *dirty != 0 {
        Rf_error("Can't re-use this wk_handler".as_ptr() as _);
    }
    *dirty = 1;
}
#[no_mangle]
pub extern "C" fn wk_default_handler_vector_start(
    _meta: *const wk_vector_meta_t,
    _handler_data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    WK_CONTINUE
}

#[no_mangle]
pub unsafe extern "C" fn wk_default_handler_vector_end(
    _meta: *const wk_vector_meta_t,
    _handler_data: *mut ::std::os::raw::c_void,
) -> SEXP {
    R_NilValue
}

#[no_mangle]
pub unsafe extern "C" fn wk_default_handler_feature(
    _meta: *const wk_vector_meta_t,
    _feat_id: R_xlen_t,
    _handler_data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    WK_CONTINUE
}

#[no_mangle]
pub extern "C" fn wk_default_handler_null_feature(
    _handler_data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    WK_CONTINUE
}

#[no_mangle]
pub extern "C" fn wk_default_handler_geometry(
    _meta: *const wk_meta_t,
    _part_id: u32,
    _handler_data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    WK_CONTINUE
}

#[no_mangle]
pub extern "C" fn wk_default_handler_ring(
    _meta: *const wk_meta_t,
    _size: u32,
    _ring_id: u32,
    _handler_data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    WK_CONTINUE
}

#[no_mangle]
pub extern "C" fn wk_default_handler_coord(
    _meta: *const wk_meta_t,
    _coord: *const f64,
    _coord_id: u32,
    _handler_data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    WK_CONTINUE
}

#[no_mangle]
pub unsafe extern "C" fn wk_default_handler_error(
    message: *const ::std::os::raw::c_char,
    _handler_data: *mut ::std::os::raw::c_void,
) -> ::std::os::raw::c_int {
    // never ending, thus no need to return a value
    Rf_error(message);
    // WK_ABORT
}
#[no_mangle]
pub unsafe extern "C" fn wk_default_handler_finalizer(_handler_data: *mut ::std::os::raw::c_void) {}

extern "C" {
    pub fn wk_handler_create() -> *mut wk_handler_t;
    pub fn wk_handler_destroy(handler: *mut wk_handler_t);
    pub fn wk_handler_destroy_xptr(xptr: SEXP);
    pub fn wk_handler_create_xptr(handler: *mut wk_handler_t, tag: SEXP, prot: SEXP) -> SEXP;
    pub fn wk_handler_run_cleanup(data: *mut ::std::os::raw::c_void);
    pub fn wk_handler_run_internal(data: *mut ::std::os::raw::c_void) -> SEXP;
    pub fn wk_handler_run_xptr(
        read_fun: ::std::option::Option<
            unsafe extern "C" fn(read_data: SEXP, handler: *mut wk_handler_t) -> SEXP,
        >,
        read_data: SEXP,
        xptr: SEXP,
    ) -> SEXP;
    pub fn wk_default_trans_trans(
        feature_id: R_xlen_t,
        xyzm_in: *const f64,
        xyzm_out: *mut f64,
        trans_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn wk_default_trans_finalizer(trans_data: *mut ::std::os::raw::c_void);
    pub fn wk_default_trans_vector(trans_data: *mut ::std::os::raw::c_void);
    pub fn wk_trans_create() -> *mut wk_trans_t;
    pub fn wk_trans_destroy(trans: *mut wk_trans_t);
    pub fn wk_trans_destroy_xptr(trans_xptr: SEXP);
    pub fn wk_trans_create_xptr(trans: *mut wk_trans_t, tag: SEXP, prot: SEXP) -> SEXP;
}
