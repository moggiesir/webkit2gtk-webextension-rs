// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;
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
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMHTMLOptionElement(Object<webkit2_webextension_sys::WebKitDOMHTMLOptionElement, webkit2_webextension_sys::WebKitDOMHTMLOptionElementClass, DOMHTMLOptionElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_option_element_get_type(),
    }
}

pub const NONE_DOMHTML_OPTION_ELEMENT: Option<&DOMHTMLOptionElement> = None;

pub trait DOMHTMLOptionElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_default_selected(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_disabled(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_index(&self) -> libc::c_long;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_label(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_selected(&self) -> bool;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_text(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_value(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_default_selected(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_disabled(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_label(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_selected(&self, value: bool);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_value(&self, value: &str);

    fn connect_property_default_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLOptionElement>> DOMHTMLOptionElementExt for O {
    fn get_default_selected(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_html_option_element_get_default_selected(self.as_ref().to_glib_none().0))
        }
    }

    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_html_option_element_get_disabled(self.as_ref().to_glib_none().0))
        }
    }

    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_html_option_element_get_form(self.as_ref().to_glib_none().0))
        }
    }

    fn get_index(&self) -> libc::c_long {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_option_element_get_index(self.as_ref().to_glib_none().0)
        }
    }

    fn get_label(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_option_element_get_label(self.as_ref().to_glib_none().0))
        }
    }

    fn get_selected(&self) -> bool {
        unsafe {
            from_glib(webkit2_webextension_sys::webkit_dom_html_option_element_get_selected(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_option_element_get_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_option_element_get_value(self.as_ref().to_glib_none().0))
        }
    }

    fn set_default_selected(&self, value: bool) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_option_element_set_default_selected(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_option_element_set_disabled(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_label(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_option_element_set_label(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_selected(&self, value: bool) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_option_element_set_selected(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_value(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_option_element_set_value(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_default_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::default-selected\0".as_ptr() as *const _,
                Some(transmute(notify_default_selected_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::disabled\0".as_ptr() as *const _,
                Some(transmute(notify_disabled_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::form\0".as_ptr() as *const _,
                Some(transmute(notify_form_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_index_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::index\0".as_ptr() as *const _,
                Some(transmute(notify_index_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::label\0".as_ptr() as *const _,
                Some(transmute(notify_label_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_selected_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::selected\0".as_ptr() as *const _,
                Some(transmute(notify_selected_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text\0".as_ptr() as *const _,
                Some(transmute(notify_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_default_selected_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLOptionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_disabled_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLOptionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_form_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLOptionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_index_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLOptionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_label_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLOptionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_selected_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLOptionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLOptionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLOptionElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLOptionElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLOptionElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLOptionElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLOptionElement")
    }
}
