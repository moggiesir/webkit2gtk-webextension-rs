// This file was generated by gir (24767f3+) from gir-files (???)
// DO NOT EDIT

#[cfg(feature = "v2_2")]
use Frame;
#[cfg(feature = "v2_2")]
use WebPage;
use ffi;
#[cfg(feature = "v2_2")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v2_2")]
use glib_ffi;
#[cfg(feature = "v2_2")]
use std::boxed::Box as Box_;
#[cfg(feature = "v2_2")]
use std::mem::transmute;

glib_wrapper! {
    pub struct ScriptWorld(Object<ffi::WebKitScriptWorld>);

    match fn {
        get_type => || ffi::webkit_script_world_get_type(),
    }
}

impl ScriptWorld {
    #[cfg(feature = "v2_2")]
    pub fn new() -> ScriptWorld {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::webkit_script_world_new())
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn get_default() -> Option<ScriptWorld> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::webkit_script_world_get_default())
        }
    }

    #[cfg(feature = "v2_2")]
    pub fn connect_window_object_cleared<F: Fn(&ScriptWorld, &WebPage, &Frame) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&ScriptWorld, &WebPage, &Frame) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "window-object-cleared",
                transmute(window_object_cleared_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v2_2")]
unsafe extern "C" fn window_object_cleared_trampoline(this: *mut ffi::WebKitScriptWorld, page: *mut ffi::WebKitWebPage, frame: *mut ffi::WebKitFrame, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&ScriptWorld, &WebPage, &Frame) + 'static> = transmute(f);
    f(&from_glib_none(this), &from_glib_none(page), &from_glib_none(frame))
}
