// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMHTMLMenuElement(Object<ffi::WebKitDOMHTMLMenuElement, ffi::WebKitDOMHTMLMenuElementClass, DOMHTMLMenuElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_menu_element_get_type(),
    }
}

pub const NONE_DOMHTML_MENU_ELEMENT: Option<&DOMHTMLMenuElement> = None;

pub trait DOMHTMLMenuElementExt: 'static {
    fn get_compact(&self) -> bool;

    fn set_compact(&self, value: bool);

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLMenuElement>> DOMHTMLMenuElementExt for O {
    fn get_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_menu_element_get_compact(self.as_ref().to_glib_none().0))
        }
    }

    fn set_compact(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_menu_element_set_compact(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn connect_property_compact_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::compact\0".as_ptr() as *const _,
                transmute(notify_compact_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_compact_trampoline<P>(this: *mut ffi::WebKitDOMHTMLMenuElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLMenuElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLMenuElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLMenuElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLMenuElement")
    }
}
