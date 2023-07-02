//! This is equivalent to code written in `wk-v1-impl.c`.
//!
use crate::bindings::wk::*;
use libR_sys::*;

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

#[no_mangle]
pub unsafe extern "C" fn wk_handler_create() -> *mut wk_handler_t {
    let layout = std::alloc::Layout::new::<wk_handler_t>();
    let handler = std::alloc::alloc(layout) as *mut wk_handler_t;
    if handler.is_null() {
        Rf_error("Failed to alloc handler".as_ptr() as _);
        // suggestion:
        // std::alloc::handle_alloc_error(layout);
    }
    (*handler).api_version = 1;
    (*handler).dirty = 0;
    (*handler).handler_data = std::ptr::null_mut();

    (*handler).initialize = Some(wk_default_handler_initialize);
    (*handler).vector_start = Some(wk_default_handler_vector_start);
    (*handler).vector_end = Some(wk_default_handler_vector_end);

    (*handler).feature_start = Some(wk_default_handler_feature);
    (*handler).null_feature = Some(wk_default_handler_null_feature);
    (*handler).feature_end = Some(wk_default_handler_feature);

    (*handler).geometry_start = Some(wk_default_handler_geometry);
    (*handler).geometry_end = Some(wk_default_handler_geometry);

    (*handler).ring_start = Some(wk_default_handler_ring);
    (*handler).ring_end = Some(wk_default_handler_ring);

    (*handler).coord = Some(wk_default_handler_coord);

    //FIXME: this field is not mentioned in header of `wk_handler_t`.
    // (*handler).error = Some(wk_default_handler_error);
    (*handler).deinitialize = Some(wk_default_handler_finalizer);
    (*handler).finalizer = Some(wk_default_handler_finalizer);

    handler
}

#[no_mangle]
pub unsafe extern "C" fn wk_handler_destroy(handler: *mut wk_handler_t) {
    if !handler.is_null() {
        (((*handler).finalizer).unwrap())((*handler).handler_data);
        std::alloc::dealloc(handler as _, std::alloc::Layout::new::<wk_handler_t>());
    }
}

#[no_mangle]
pub unsafe extern "C" fn wk_handler_destroy_xptr(xptr: SEXP) {
    wk_handler_destroy(R_ExternalPtrAddr(xptr) as _);
}

#[no_mangle]
pub unsafe extern "C" fn wk_handler_create_xptr(
    handler: *mut wk_handler_t,
    tag: SEXP,
    prot: SEXP,
) -> SEXP {
    let xptr = R_MakeExternalPtr(handler as _, tag, prot);
    R_RegisterCFinalizerEx(xptr, Some(wk_handler_destroy_xptr), Rboolean_FALSE);
    xptr
}

#[repr(C)]
pub struct wk_handler_run_data {
    pub read_fun: ::std::option::Option<
        unsafe extern "C" fn(read_data: SEXP, handler: *mut wk_handler_t) -> SEXP,
    >,
    pub read_data: SEXP,
    pub handler: *mut wk_handler_t,
}

#[no_mangle]
pub unsafe extern "C" fn wk_handler_run_cleanup(data: *mut ::std::os::raw::c_void) {
    // let run_data = data as *mut wk_handler_run_data;
    let run_data = data.cast::<wk_handler_run_data>();
    let handler = *(*run_data).handler;
    (handler.deinitialize.unwrap())(handler.handler_data);
}

#[no_mangle]
pub unsafe extern "C" fn wk_handler_run_internal(data: *mut ::std::os::raw::c_void) -> SEXP {
    let run_data = data.cast::<wk_handler_run_data>();
    let api_version = (*(*run_data).handler).api_version;
    if api_version != 1 {
        Rf_error(format!("Can't run a wk_handler with api_version {api_version}").as_ptr() as _)
    }

    (((*(*run_data).handler).initialize).unwrap())(
        &mut (*(*run_data).handler).dirty,
        (*(*run_data).handler).handler_data,
    );

    ((*run_data).read_fun.unwrap())((*run_data).read_data, (*run_data).handler)
}

#[no_mangle]
pub unsafe extern "C" fn wk_handler_run_xptr(
    read_fun: ::std::option::Option<
        unsafe extern "C" fn(read_data: SEXP, handler: *mut wk_handler_t) -> SEXP,
    >,
    read_data: SEXP,
    xptr: SEXP,
) -> SEXP {
    use std::os::raw::c_void;
    let handler = R_ExternalPtrAddr(xptr).cast::<wk_handler_t>();
    let mut run_data = wk_handler_run_data {
        read_fun,
        read_data,
        handler,
    };
    R_ExecWithCleanup(
        Some(wk_handler_run_internal),
        (&mut run_data as *mut _) as *mut c_void,
        Some(wk_handler_run_cleanup),
        (&mut run_data as *mut _) as *mut c_void,
    )
}

extern "C" {
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
