// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#[cfg(any(feature = "v2_16", feature = "dox"))]
use DOMDOMTokenList;
use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use DOMStyleSheet;
use ffi;
use glib::GString;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMHTMLLinkElement(Object<ffi::WebKitDOMHTMLLinkElement, ffi::WebKitDOMHTMLLinkElementClass, DOMHTMLLinkElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_link_element_get_type(),
    }
}

pub const NONE_DOMHTML_LINK_ELEMENT: Option<&DOMHTMLLinkElement> = None;

pub trait DOMHTMLLinkElementExt: 'static {
    fn get_charset(&self) -> Option<GString>;

    fn get_disabled(&self) -> bool;

    fn get_href(&self) -> Option<GString>;

    fn get_hreflang(&self) -> Option<GString>;

    fn get_media(&self) -> Option<GString>;

    fn get_rel(&self) -> Option<GString>;

    fn get_rev(&self) -> Option<GString>;

    fn get_sheet(&self) -> Option<DOMStyleSheet>;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_sizes(&self) -> Option<DOMDOMTokenList>;

    fn get_target(&self) -> Option<GString>;

    fn get_type_attr(&self) -> Option<GString>;

    fn set_charset(&self, value: &str);

    fn set_disabled(&self, value: bool);

    fn set_href(&self, value: &str);

    fn set_hreflang(&self, value: &str);

    fn set_media(&self, value: &str);

    fn set_rel(&self, value: &str);

    fn set_rev(&self, value: &str);

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_sizes(&self, value: &str);

    fn set_target(&self, value: &str);

    fn set_type_attr(&self, value: &str);

    fn get_property_type(&self) -> Option<GString>;

    fn set_property_type<'a, P: Into<Option<&'a str>>>(&self, type_: P);

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_hreflang_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rev_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_property_sizes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLLinkElement>> DOMHTMLLinkElementExt for O {
    fn get_charset(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_charset(self.as_ref().to_glib_none().0))
        }
    }

    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_link_element_get_disabled(self.as_ref().to_glib_none().0))
        }
    }

    fn get_href(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_href(self.as_ref().to_glib_none().0))
        }
    }

    fn get_hreflang(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_hreflang(self.as_ref().to_glib_none().0))
        }
    }

    fn get_media(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_media(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rel(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_rel(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rev(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_rev(self.as_ref().to_glib_none().0))
        }
    }

    fn get_sheet(&self) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_sheet(self.as_ref().to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn get_sizes(&self) -> Option<DOMDOMTokenList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_sizes(self.as_ref().to_glib_none().0))
        }
    }

    fn get_target(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_target(self.as_ref().to_glib_none().0))
        }
    }

    fn get_type_attr(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_link_element_get_type_attr(self.as_ref().to_glib_none().0))
        }
    }

    fn set_charset(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_charset(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_disabled(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_href(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_href(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_hreflang(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_hreflang(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_media(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_media(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_rel(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_rel(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_rev(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_rev(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn set_sizes(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_sizes(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_target(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_link_element_set_type_attr(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_type<'a, P: Into<Option<&'a str>>>(&self, type_: P) {
        let type_ = type_.into();
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"type\0".as_ptr() as *const _, Value::from(type_).to_glib_none().0);
        }
    }

    fn connect_property_charset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::charset\0".as_ptr() as *const _,
                transmute(notify_charset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::disabled\0".as_ptr() as *const _,
                transmute(notify_disabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_href_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::href\0".as_ptr() as *const _,
                transmute(notify_href_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_hreflang_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::hreflang\0".as_ptr() as *const _,
                transmute(notify_hreflang_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_media_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::media\0".as_ptr() as *const _,
                transmute(notify_media_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rel_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::rel\0".as_ptr() as *const _,
                transmute(notify_rel_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rev_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::rev\0".as_ptr() as *const _,
                transmute(notify_rev_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_sheet_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::sheet\0".as_ptr() as *const _,
                transmute(notify_sheet_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_16", feature = "dox"))]
    fn connect_property_sizes_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::sizes\0".as_ptr() as *const _,
                transmute(notify_sizes_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::target\0".as_ptr() as *const _,
                transmute(notify_target_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_charset_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_disabled_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_href_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_hreflang_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_media_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rel_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rev_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_sheet_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

#[cfg(any(feature = "v2_16", feature = "dox"))]
unsafe extern "C" fn notify_sizes_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_target_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLLinkElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLLinkElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLLinkElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLLinkElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLLinkElement")
    }
}
