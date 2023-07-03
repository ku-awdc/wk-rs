pub mod bindings {
    pub mod wk {
        #![allow(non_snake_case)]
        #![allow(non_camel_case_types)]
        #![allow(dead_code)]
        use libR_sys::*;
        include!("bindings_wk.rs");
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
