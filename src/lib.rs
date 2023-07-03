pub mod bindings {
    pub mod wk {
        #![allow(non_snake_case)]
        #![allow(non_camel_case_types)]
        #![allow(dead_code)]
        use libR_sys::*;
        include!("bindings_wk.rs");

        // region: manual impl

        impl wk_meta_t {
            /// Equivalent to `WK_META_RESET` function-like macro in `wk-v1.h`.
            pub fn reset(&mut self, geometry_type: wk_geometery_type_enum) {
                self.geometry_type = geometry_type as _;
                self.flags = 0;
                self.precision = WK_PRECISION_NONE;
                self.srid = WK_SRID_NONE;
                self.size = WK_SIZE_UNKNOWN;
            }
        }

        impl wk_vector_meta_t {
            /// Equivalent to `WK_VECTOR_META_RESET` function-like macro in `wk-v1.h`.
            fn reset(&mut self, geometry_type: wk_geometery_type_enum) {
                self.geometry_type = geometry_type as _;
                self.flags = 0;
                //FIXME: (in bindings) these should be i32 instead of isize;
                self.size = WK_VECTOR_SIZE_UNKNOWN as _;
            }
        }

        // endregion
    }
    pub mod wk_default_rust;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_lib() {
        // this ensures that things are being built and linked properly.
        let b_handler = unsafe { bindings::wk_default_rust::wk_handler_create() };
    }
}
