// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMDOMWindow(Object<ffi::WebKitDOMDOMWindow>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_dom_window_get_type(),
    }
}

impl DOMDOMWindow {
    pub fn webkit_message_handlers_post_message(&self, handler: &str, message: &str) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_dom_window_webkit_message_handlers_post_message(self.to_glib_none().0, handler.to_glib_none().0, message.to_glib_none().0))
        }
    }
}
