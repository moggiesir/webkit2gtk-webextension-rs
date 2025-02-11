// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMNodeList(Object<webkit2_webextension_sys::WebKitDOMNodeList, webkit2_webextension_sys::WebKitDOMNodeListClass, DOMNodeListClass>) @extends DOMObject;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_node_list_get_type(),
    }
}

pub const NONE_DOM_NODE_LIST: Option<&DOMNodeList> = None;

pub trait DOMNodeListExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_length(&self) -> libc::c_ulong;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn item(&self, index: libc::c_ulong) -> Option<DOMNode>;

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMNodeList>> DOMNodeListExt for O {
    fn get_length(&self) -> libc::c_ulong {
        unsafe {
            webkit2_webextension_sys::webkit_dom_node_list_get_length(self.as_ref().to_glib_none().0)
        }
    }

    fn item(&self, index: libc::c_ulong) -> Option<DOMNode> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_node_list_item(self.as_ref().to_glib_none().0, index))
        }
    }

    fn connect_property_length_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::length\0".as_ptr() as *const _,
                Some(transmute(notify_length_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_length_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMNodeList, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMNodeList> {
    let f: &F = &*(f as *const F);
    f(&DOMNodeList::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMNodeList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMNodeList")
    }
}
