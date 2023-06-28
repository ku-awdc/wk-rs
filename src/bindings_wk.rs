/* automatically generated by rust-bindgen 0.66.1 */

pub type R_xlen_t = isize;
pub type SEXP = *mut SEXPREC;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SEXPREC {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wk_meta_t {
    pub geometry_type: u32,
    pub flags: u32,
    pub srid: u32,
    pub size: u32,
    pub precision: f64,
    pub bounds_min: [f64; 4usize],
    pub bounds_max: [f64; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wk_vector_meta_t {
    pub geometry_type: u32,
    pub flags: u32,
    pub size: R_xlen_t,
    pub bounds_min: [f64; 4usize],
    pub bounds_max: [f64; 4usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wk_handler_t {
    pub api_version: ::std::os::raw::c_int,
    pub dirty: ::std::os::raw::c_int,
    pub handler_data: *mut ::std::os::raw::c_void,
    pub initialize: ::std::option::Option<
        unsafe extern "C" fn(
            dirty: *mut ::std::os::raw::c_int,
            handler_data: *mut ::std::os::raw::c_void,
        ),
    >,
    pub vector_start: ::std::option::Option<
        unsafe extern "C" fn(
            meta: *const wk_vector_meta_t,
            handler_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub feature_start: ::std::option::Option<
        unsafe extern "C" fn(
            meta: *const wk_vector_meta_t,
            feat_id: R_xlen_t,
            handler_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub null_feature: ::std::option::Option<
        unsafe extern "C" fn(handler_data: *mut ::std::os::raw::c_void) -> ::std::os::raw::c_int,
    >,
    pub geometry_start: ::std::option::Option<
        unsafe extern "C" fn(
            meta: *const wk_meta_t,
            part_id: u32,
            handler_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub ring_start: ::std::option::Option<
        unsafe extern "C" fn(
            meta: *const wk_meta_t,
            size: u32,
            ring_id: u32,
            handler_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub coord: ::std::option::Option<
        unsafe extern "C" fn(
            meta: *const wk_meta_t,
            coord: *const f64,
            coord_id: u32,
            handler_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub ring_end: ::std::option::Option<
        unsafe extern "C" fn(
            meta: *const wk_meta_t,
            size: u32,
            ring_id: u32,
            handler_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub geometry_end: ::std::option::Option<
        unsafe extern "C" fn(
            meta: *const wk_meta_t,
            part_id: u32,
            handler_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub feature_end: ::std::option::Option<
        unsafe extern "C" fn(
            meta: *const wk_vector_meta_t,
            feat_id: R_xlen_t,
            handler_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub vector_end: ::std::option::Option<
        unsafe extern "C" fn(
            meta: *const wk_vector_meta_t,
            handler_data: *mut ::std::os::raw::c_void,
        ) -> SEXP,
    >,
    pub Rf_error: ::std::option::Option<
        unsafe extern "C" fn(
            message: *const ::std::os::raw::c_char,
            handler_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub deinitialize:
        ::std::option::Option<unsafe extern "C" fn(handler_data: *mut ::std::os::raw::c_void)>,
    pub finalizer:
        ::std::option::Option<unsafe extern "C" fn(handler_data: *mut ::std::os::raw::c_void)>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct wk_trans_t {
    pub api_version: ::std::os::raw::c_int,
    pub trans_data: *mut ::std::os::raw::c_void,
    pub use_z: ::std::os::raw::c_int,
    pub use_m: ::std::os::raw::c_int,
    pub xyzm_out_min: [f64; 4usize],
    pub xyzm_out_max: [f64; 4usize],
    pub trans: ::std::option::Option<
        unsafe extern "C" fn(
            feature_id: R_xlen_t,
            xyzm_in: *const f64,
            xyzm_out: *mut f64,
            trans_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
    >,
    pub vector_end:
        ::std::option::Option<unsafe extern "C" fn(trans_data: *mut ::std::os::raw::c_void)>,
    pub finalizer:
        ::std::option::Option<unsafe extern "C" fn(trans_data: *mut ::std::os::raw::c_void)>,
}
pub const WK_CONTINUE: u32 = 0;
pub const WK_ABORT: u32 = 1;
pub const WK_ABORT_FEATURE: u32 = 2;
pub const WK_FLAG_HAS_BOUNDS: u32 = 1;
pub const WK_FLAG_HAS_Z: u32 = 2;
pub const WK_FLAG_HAS_M: u32 = 4;
pub const WK_FLAG_DIMS_UNKNOWN: u32 = 8;
pub const WK_PRECISION_NONE: f64 = 0.0;
pub const WK_PART_ID_NONE: u32 = 4294967295;
pub const WK_SIZE_UNKNOWN: u32 = 4294967295;
pub const WK_VECTOR_SIZE_UNKNOWN: i32 = -1;
pub const WK_SRID_NONE: u32 = 4294967295;
#[repr(i32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum wk_geometery_type_enum {
    WK_GEOMETRY = 0,
    WK_POINT = 1,
    WK_LINESTRING = 2,
    WK_POLYGON = 3,
    WK_MULTIPOINT = 4,
    WK_MULTILINESTRING = 5,
    WK_MULTIPOLYGON = 6,
    WK_GEOMETRYCOLLECTION = 7,
}
extern "C" {
    pub fn wk_handler_create() -> *mut wk_handler_t;
}
extern "C" {
    pub fn wk_handler_create_xptr(handler: *mut wk_handler_t, tag: SEXP, prot: SEXP) -> SEXP;
}
extern "C" {
    pub fn wk_handler_destroy(handler: *mut wk_handler_t);
}
extern "C" {
    pub fn wk_handler_run_xptr(
        read_fun: ::std::option::Option<
            unsafe extern "C" fn(read_data: SEXP, handler: *mut wk_handler_t) -> SEXP,
        >,
        read_data: SEXP,
        xptr: SEXP,
    ) -> SEXP;
}
extern "C" {
    pub fn wk_trans_create() -> *mut wk_trans_t;
}
extern "C" {
    pub fn wk_trans_create_xptr(trans: *mut wk_trans_t, tag: SEXP, prot: SEXP) -> SEXP;
}
extern "C" {
    pub fn wk_trans_destroy(trans: *mut wk_trans_t);
}
