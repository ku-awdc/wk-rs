pub mod bindings {

    pub mod wk {
        #![allow(non_snake_case)]
        #![allow(non_camel_case_types)]
        #![allow(dead_code)]
        use libR_sys::*;
        include!("bindings_wk.rs");
    }
    pub mod wk_default {
        use super::wk::*;
        use libR_sys::*;
        include!("bindings_wk_default_impl.rs");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_lib() {
        // this ensures that things are being built and linked properly.
        let a_handler = unsafe { bindings::wk::wk_handler_create() };
    }
}
