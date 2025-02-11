// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use webkit2_webextension_sys;

glib_wrapper! {
    pub struct DOMHTMLParamElement(Object<webkit2_webextension_sys::WebKitDOMHTMLParamElement, webkit2_webextension_sys::WebKitDOMHTMLParamElementClass, DOMHTMLParamElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_param_element_get_type(),
    }
}

pub const NONE_DOMHTML_PARAM_ELEMENT: Option<&DOMHTMLParamElement> = None;

pub trait DOMHTMLParamElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_name(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_type_attr(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_value(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_value_type(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_name(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_type_attr(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_value(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_value_type(&self, value: &str);

    fn get_property_type(&self) -> Option<GString>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLParamElement>> DOMHTMLParamElementExt for O {
    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_param_element_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_param_element_get_type_attr(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_param_element_get_value(self.as_ref().to_glib_none().0))
        }
    }

    fn get_value_type(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_param_element_get_value_type(self.as_ref().to_glib_none().0))
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_param_element_set_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_param_element_set_type_attr(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_value(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_param_element_set_value(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_value_type(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_param_element_set_value_type(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_type(&self, type_: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"type\0".as_ptr() as *const _, Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute(notify_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                Some(transmute(notify_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                Some(transmute(notify_value_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_value_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::value-type\0".as_ptr() as *const _,
                Some(transmute(notify_value_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLParamElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLParamElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLParamElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLParamElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLParamElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLParamElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLParamElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLParamElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLParamElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLParamElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLParamElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLParamElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLParamElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLParamElement")
    }
}
