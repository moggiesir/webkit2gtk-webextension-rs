// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use Error;
use glib::GString;
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
use std::ptr;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMHTMLTableSectionElement(Object<webkit2_webextension_sys::WebKitDOMHTMLTableSectionElement, webkit2_webextension_sys::WebKitDOMHTMLTableSectionElementClass, DOMHTMLTableSectionElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_table_section_element_get_type(),
    }
}

pub const NONE_DOMHTML_TABLE_SECTION_ELEMENT: Option<&DOMHTMLTableSectionElement> = None;

pub trait DOMHTMLTableSectionElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn delete_row(&self, index: libc::c_long) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_align(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_ch(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_ch_off(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_rows(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_v_align(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn insert_row(&self, index: libc::c_long) -> Result<DOMHTMLElement, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_align(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_ch(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_ch_off(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_v_align(&self, value: &str);

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ch_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_ch_off_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_v_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLTableSectionElement>> DOMHTMLTableSectionElementExt for O {
    fn delete_row(&self, index: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_html_table_section_element_delete_row(self.as_ref().to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_align(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_section_element_get_align(self.as_ref().to_glib_none().0))
        }
    }

    fn get_ch(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_section_element_get_ch(self.as_ref().to_glib_none().0))
        }
    }

    fn get_ch_off(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_section_element_get_ch_off(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rows(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_section_element_get_rows(self.as_ref().to_glib_none().0))
        }
    }

    fn get_v_align(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_section_element_get_v_align(self.as_ref().to_glib_none().0))
        }
    }

    fn insert_row(&self, index: libc::c_long) -> Result<DOMHTMLElement, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_html_table_section_element_insert_row(self.as_ref().to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_section_element_set_align(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_ch(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_section_element_set_ch(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_ch_off(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_section_element_set_ch_off(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_v_align(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_section_element_set_v_align(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::align\0".as_ptr() as *const _,
                Some(transmute(notify_align_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_ch_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::ch\0".as_ptr() as *const _,
                Some(transmute(notify_ch_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_ch_off_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::ch-off\0".as_ptr() as *const _,
                Some(transmute(notify_ch_off_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rows\0".as_ptr() as *const _,
                Some(transmute(notify_rows_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_v_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::v-align\0".as_ptr() as *const _,
                Some(transmute(notify_v_align_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_align_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableSectionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableSectionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableSectionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_ch_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableSectionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableSectionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableSectionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_ch_off_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableSectionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableSectionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableSectionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rows_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableSectionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableSectionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableSectionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_v_align_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableSectionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableSectionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableSectionElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLTableSectionElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLTableSectionElement")
    }
}
