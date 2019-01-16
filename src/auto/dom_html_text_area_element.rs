// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;
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
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMHTMLTextAreaElement(Object<ffi::WebKitDOMHTMLTextAreaElement, ffi::WebKitDOMHTMLTextAreaElementClass, DOMHTMLTextAreaElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_text_area_element_get_type(),
    }
}

pub const NONE_DOMHTML_TEXT_AREA_ELEMENT: Option<&DOMHTMLTextAreaElement> = None;

pub trait DOMHTMLTextAreaElementExt: 'static {
    fn get_area_type(&self) -> Option<GString>;

    fn get_autofocus(&self) -> bool;

    fn get_cols(&self) -> libc::c_long;

    fn get_default_value(&self) -> Option<GString>;

    fn get_disabled(&self) -> bool;

    fn get_form(&self) -> Option<DOMHTMLFormElement>;

    fn get_name(&self) -> Option<GString>;

    fn get_read_only(&self) -> bool;

    fn get_rows(&self) -> libc::c_long;

    fn get_selection_end(&self) -> libc::c_long;

    fn get_selection_start(&self) -> libc::c_long;

    fn get_value(&self) -> Option<GString>;

    fn get_will_validate(&self) -> bool;

    fn is_edited(&self) -> bool;

    fn select(&self);

    fn set_autofocus(&self, value: bool);

    fn set_cols(&self, value: libc::c_long);

    fn set_default_value(&self, value: &str);

    fn set_disabled(&self, value: bool);

    fn set_name(&self, value: &str);

    fn set_read_only(&self, value: bool);

    fn set_rows(&self, value: libc::c_long);

    fn set_selection_end(&self, value: libc::c_long);

    fn set_selection_range(&self, start: libc::c_long, end: libc::c_long, direction: &str);

    fn set_selection_start(&self, value: libc::c_long);

    fn set_value(&self, value: &str);

    fn get_property_type(&self) -> Option<GString>;

    fn connect_property_autofocus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cols_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_default_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selection_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_selection_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_will_validate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLTextAreaElement>> DOMHTMLTextAreaElementExt for O {
    fn get_area_type(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_text_area_element_get_area_type(self.as_ref().to_glib_none().0))
        }
    }

    fn get_autofocus(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_text_area_element_get_autofocus(self.as_ref().to_glib_none().0))
        }
    }

    fn get_cols(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_text_area_element_get_cols(self.as_ref().to_glib_none().0)
        }
    }

    fn get_default_value(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_text_area_element_get_default_value(self.as_ref().to_glib_none().0))
        }
    }

    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_text_area_element_get_disabled(self.as_ref().to_glib_none().0))
        }
    }

    fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_text_area_element_get_form(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_text_area_element_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_read_only(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_text_area_element_get_read_only(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rows(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_text_area_element_get_rows(self.as_ref().to_glib_none().0)
        }
    }

    fn get_selection_end(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_text_area_element_get_selection_end(self.as_ref().to_glib_none().0)
        }
    }

    fn get_selection_start(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_text_area_element_get_selection_start(self.as_ref().to_glib_none().0)
        }
    }

    fn get_value(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_text_area_element_get_value(self.as_ref().to_glib_none().0))
        }
    }

    fn get_will_validate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_text_area_element_get_will_validate(self.as_ref().to_glib_none().0))
        }
    }

    fn is_edited(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_text_area_element_is_edited(self.as_ref().to_glib_none().0))
        }
    }

    fn select(&self) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_select(self.as_ref().to_glib_none().0);
        }
    }

    fn set_autofocus(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_autofocus(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_cols(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_cols(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_default_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_default_value(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_disabled(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_read_only(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_read_only(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_rows(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_rows(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_selection_end(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_selection_end(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_selection_range(&self, start: libc::c_long, end: libc::c_long, direction: &str) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_selection_range(self.as_ref().to_glib_none().0, start, end, direction.to_glib_none().0);
        }
    }

    fn set_selection_start(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_selection_start(self.as_ref().to_glib_none().0, value);
        }
    }

    fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_text_area_element_set_value(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn get_property_type(&self) -> Option<GString> {
        unsafe {
            let mut value = Value::from_type(<GString as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"type\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_autofocus_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::autofocus\0".as_ptr() as *const _,
                transmute(notify_autofocus_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_cols_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::cols\0".as_ptr() as *const _,
                transmute(notify_cols_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_default_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::default-value\0".as_ptr() as *const _,
                transmute(notify_default_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_disabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::disabled\0".as_ptr() as *const _,
                transmute(notify_disabled_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_form_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::form\0".as_ptr() as *const _,
                transmute(notify_form_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::read-only\0".as_ptr() as *const _,
                transmute(notify_read_only_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::rows\0".as_ptr() as *const _,
                transmute(notify_rows_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_selection_end_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::selection-end\0".as_ptr() as *const _,
                transmute(notify_selection_end_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_selection_start_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::selection-start\0".as_ptr() as *const _,
                transmute(notify_selection_start_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::type\0".as_ptr() as *const _,
                transmute(notify_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_value_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::value\0".as_ptr() as *const _,
                transmute(notify_value_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_will_validate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::will-validate\0".as_ptr() as *const _,
                transmute(notify_will_validate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_autofocus_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_cols_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_default_value_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_disabled_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_form_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_read_only_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rows_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_selection_end_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_selection_start_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_type_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_value_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_will_validate_trampoline<P>(this: *mut ffi::WebKitDOMHTMLTextAreaElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLTextAreaElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLTextAreaElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLTextAreaElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLTextAreaElement")
    }
}
