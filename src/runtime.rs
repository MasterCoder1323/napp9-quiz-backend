use tokio::runtime::{Builder, Runtime};
use std::sync::OnceLock;

// Global static Tokio runtime, initialized once and reused everywhere
pub static TOKIO_RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// Get a reference to the global multi-threaded Tokio runtime.
/// Creates the runtime on first call.
pub fn get_runtime() -> &'static Runtime {
    TOKIO_RUNTIME.get_or_init(|| {
        Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
}