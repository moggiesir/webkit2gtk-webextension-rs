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
    pub struct DOMHTMLAnchorElement(Object<webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, webkit2_webextension_sys::WebKitDOMHTMLAnchorElementClass, DOMHTMLAnchorElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_anchor_element_get_type(),
    }
}

pub const NONE_DOMHTML_ANCHOR_ELEMENT: Option<&DOMHTMLAnchorElement> = None;

pub trait DOMHTMLAnchorElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_charset(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_coords(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_hash(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_host(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_hostname(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_href(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_hreflang(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_name(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_pathname(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_port(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_protocol(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_rel(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_rev(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_search(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_shape(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_target(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_text(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_type_attr(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_charset(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_coords(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_hash(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_host(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_hostname(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_href(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_hreflang(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_name(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_pathname(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_port(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_protocol(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_rel(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_rev(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_search(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_shape(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_target(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_text(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_type_attr(&self, value: &str);

    fn set_property_text(&self, text: Option<&str>);

    fn get_property_type(&self) -> Option<GString>;

    fn set_property_type(&self, type_: Option<&str>);

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_coords_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_host_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hostname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hreflang_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_pathname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rev_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_shape_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLAnchorElement>> DOMHTMLAnchorElementExt for O {
    fn get_charset(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_charset(self.as_ref().to_glib_none().0))
        }
    }

    fn get_coords(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_coords(self.as_ref().to_glib_none().0))
        }
    }

    fn get_hash(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_hash(self.as_ref().to_glib_none().0))
        }
    }

    fn get_host(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_host(self.as_ref().to_glib_none().0))
        }
    }

    fn get_hostname(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_hostname(self.as_ref().to_glib_none().0))
        }
    }

    fn get_href(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_href(self.as_ref().to_glib_none().0))
        }
    }

    fn get_hreflang(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_hreflang(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_pathname(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_pathname(self.as_ref().to_glib_none().0))
        }
    }

    fn get_port(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_port(self.as_ref().to_glib_none().0))
        }
    }

    fn get_protocol(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_protocol(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rel(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_rel(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rev(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_rev(self.as_ref().to_glib_none().0))
        }
    }

    fn get_search(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_search(self.as_ref().to_glib_none().0))
        }
    }

    fn get_shape(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_shape(self.as_ref().to_glib_none().0))
        }
    }

    fn get_target(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_target(self.as_ref().to_glib_none().0))
        }
    }

    fn get_text(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_text(self.as_ref().to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_anchor_element_get_type_attr(self.as_ref().to_glib_none().0))
        }
    }

    fn set_charset(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_charset(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_coords(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_coords(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hash(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_hash(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_host(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_host(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hostname(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_hostname(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_href(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_href(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hreflang(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_hreflang(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_pathname(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_pathname(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_port(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_port(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_protocol(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_protocol(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_rel(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_rel(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_rev(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_rev(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_search(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_search(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_shape(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_shape(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_target(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_target(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_text(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_text(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_anchor_element_set_type_attr(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_property_text(&self, text: Option<&str>) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"text\0".as_ptr() as *const _, Value::from(text).to_glib_none().0);
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

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::charset\0".as_ptr() as *const _,
                Some(transmute(notify_charset_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_coords_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::coords\0".as_ptr() as *const _,
                Some(transmute(notify_coords_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_hash_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::hash\0".as_ptr() as *const _,
                Some(transmute(notify_hash_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_host_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::host\0".as_ptr() as *const _,
                Some(transmute(notify_host_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_hostname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::hostname\0".as_ptr() as *const _,
                Some(transmute(notify_hostname_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::href\0".as_ptr() as *const _,
                Some(transmute(notify_href_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_hreflang_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::hreflang\0".as_ptr() as *const _,
                Some(transmute(notify_hreflang_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                Some(transmute(notify_name_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_pathname_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::pathname\0".as_ptr() as *const _,
                Some(transmute(notify_pathname_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_port_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::port\0".as_ptr() as *const _,
                Some(transmute(notify_port_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::protocol\0".as_ptr() as *const _,
                Some(transmute(notify_protocol_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rel\0".as_ptr() as *const _,
                Some(transmute(notify_rel_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rev_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rev\0".as_ptr() as *const _,
                Some(transmute(notify_rev_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_search_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::search\0".as_ptr() as *const _,
                Some(transmute(notify_search_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_shape_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::shape\0".as_ptr() as *const _,
                Some(transmute(notify_shape_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::target\0".as_ptr() as *const _,
                Some(transmute(notify_target_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text\0".as_ptr() as *const _,
                Some(transmute(notify_text_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                Some(transmute(notify_type_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_charset_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_coords_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_hash_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_host_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_hostname_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_href_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_hreflang_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_name_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_pathname_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_port_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_protocol_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rel_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rev_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_search_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_shape_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_target_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_type_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLAnchorElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLAnchorElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLAnchorElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLAnchorElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLAnchorElement")
    }
}
