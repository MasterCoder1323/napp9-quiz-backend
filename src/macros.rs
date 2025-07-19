#[macro_export]
macro_rules! export_ffi_fn {
    ($name:ident, $handler:ident) => {
        #[no_mangle]
        pub extern "C" fn $name(buffer: *mut ::libc::c_char) {
            crate::runtime::get_runtime().block_on(async {
                crate::handlers::$handler(buffer).await;
            });
        }
    };
}
