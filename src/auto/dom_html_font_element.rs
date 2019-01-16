// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::GString;
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
    pub struct DOMHTMLFontElement(Object<ffi::WebKitDOMHTMLFontElement, ffi::WebKitDOMHTMLFontElementClass, DOMHTMLFontElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_font_element_get_type(),
    }
}

pub const NONE_DOMHTML_FONT_ELEMENT: Option<&DOMHTMLFontElement> = None;

pub trait DOMHTMLFontElementExt: 'static {
    fn get_color(&self) -> Option<GString>;

    fn get_face(&self) -> Option<GString>;

    fn get_size(&self) -> Option<GString>;

    fn set_color(&self, value: &str);

    fn set_face(&self, value: &str);

    fn set_size(&self, value: &str);

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_face_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFontElement>> DOMHTMLFontElementExt for O {
    fn get_color(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_font_element_get_color(self.as_ref().to_glib_none().0))
        }
    }

    fn get_face(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_font_element_get_face(self.as_ref().to_glib_none().0))
        }
    }

    fn get_size(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_font_element_get_size(self.as_ref().to_glib_none().0))
        }
    }

    fn set_color(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_font_element_set_color(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_face(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_font_element_set_face(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_size(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_font_element_set_size(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::color\0".as_ptr() as *const _,
                transmute(notify_color_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_face_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::face\0".as_ptr() as *const _,
                transmute(notify_face_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::size\0".as_ptr() as *const _,
                transmute(notify_size_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_color_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFontElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFontElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFontElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_face_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFontElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFontElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFontElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_size_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFontElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFontElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFontElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLFontElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLFontElement")
    }
}
