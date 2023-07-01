/* automatically generated by rust-bindgen 0.66.1 */

#[repr(C)]
pub struct wk_handler_run_data {
    pub read_fun: ::std::option::Option<
        unsafe extern "C" fn(read_data: SEXP, handler: *mut wk_handler_t) -> SEXP,
    >,
    pub read_data: SEXP,
    pub handler: *mut wk_handler_t,
}
extern "C" {
    pub fn wk_default_handler_initialize(
        dirty: *mut ::std::os::raw::c_int,
        handler_data: *mut ::std::os::raw::c_void,
    );
    pub fn wk_default_handler_vector_start(
        meta: *const wk_vector_meta_t,
        handler_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn wk_default_handler_vector_end(
        meta: *const wk_vector_meta_t,
        handler_data: *mut ::std::os::raw::c_void,
    ) -> SEXP;
    pub fn wk_default_handler_feature(
        meta: *const wk_vector_meta_t,
        feat_id: R_xlen_t,
        handler_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn wk_default_handler_null_feature(
        handler_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn wk_default_handler_geometry(
        meta: *const wk_meta_t,
        part_id: u32,
        handler_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn wk_default_handler_ring(
        meta: *const wk_meta_t,
        size: u32,
        ring_id: u32,
        handler_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn wk_default_handler_coord(
        meta: *const wk_meta_t,
        coord: *const f64,
        coord_id: u32,
        handler_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn wk_default_handler_error(
        message: *const ::std::os::raw::c_char,
        handler_data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
    pub fn wk_default_handler_finalizer(handler_data: *mut ::std::os::raw::c_void);
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
