#![allow(non_snake_case)]

extern crate objc;
extern crate objc_id;
extern crate objc_foundation;
extern crate uikit;

mod app;

#[doc(hidden)]
pub mod extern_fns {
    pub use app::{
        RustApplicationDelegateCreate,
        RustApplicationDelegateDestroy,
        RustApplicationDelegateCreateRootViewController,
        RustApplicationDelegateDidFinishLaunching,
    };
}

pub use app::{application_main, ApplicationDelegate};
