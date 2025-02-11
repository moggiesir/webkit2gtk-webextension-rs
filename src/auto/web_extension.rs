// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use WebPage;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct WebExtension(Object<webkit2_webextension_sys::WebKitWebExtension, webkit2_webextension_sys::WebKitWebExtensionClass, WebExtensionClass>);

    match fn {
        get_type => || webkit2_webextension_sys::webkit_web_extension_get_type(),
    }
}

pub const NONE_WEB_EXTENSION: Option<&WebExtension> = None;

pub trait WebExtensionExt: 'static {
    fn get_page(&self, page_id: u64) -> Option<WebPage>;

    fn connect_page_created<F: Fn(&Self, &WebPage) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<WebExtension>> WebExtensionExt for O {
    fn get_page(&self, page_id: u64) -> Option<WebPage> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_web_extension_get_page(self.as_ref().to_glib_none().0, page_id))
        }
    }

    fn connect_page_created<F: Fn(&Self, &WebPage) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"page-created\0".as_ptr() as *const _,
                Some(transmute(page_created_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn page_created_trampoline<P, F: Fn(&P, &WebPage) + 'static>(this: *mut webkit2_webextension_sys::WebKitWebExtension, web_page: *mut webkit2_webextension_sys::WebKitWebPage, f: glib_sys::gpointer)
where P: IsA<WebExtension> {
    let f: &F = &*(f as *const F);
    f(&WebExtension::from_glib_borrow(this).unsafe_cast(), &from_glib_borrow(web_page))
}

impl fmt::Display for WebExtension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "WebExtension")
    }
}
