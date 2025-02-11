// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMHTMLElement;
use DOMHTMLTableCaptionElement;
use DOMHTMLTableSectionElement;
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
    pub struct DOMHTMLTableElement(Object<webkit2_webextension_sys::WebKitDOMHTMLTableElement, webkit2_webextension_sys::WebKitDOMHTMLTableElementClass, DOMHTMLTableElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || webkit2_webextension_sys::webkit_dom_html_table_element_get_type(),
    }
}

pub const NONE_DOMHTML_TABLE_ELEMENT: Option<&DOMHTMLTableElement> = None;

pub trait DOMHTMLTableElementExt: 'static {
    #[cfg_attr(feature = "v2_22", deprecated)]
    fn create_caption(&self) -> Option<DOMHTMLElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn create_t_foot(&self) -> Option<DOMHTMLElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn create_t_head(&self) -> Option<DOMHTMLElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn delete_caption(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn delete_row(&self, index: libc::c_long) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn delete_t_foot(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn delete_t_head(&self);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_align(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_bg_color(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_border(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_caption(&self) -> Option<DOMHTMLTableCaptionElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_cell_padding(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_cell_spacing(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_rows(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_rules(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_summary(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_t_bodies(&self) -> Option<DOMHTMLCollection>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_t_foot(&self) -> Option<DOMHTMLTableSectionElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_t_head(&self) -> Option<DOMHTMLTableSectionElement>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn get_width(&self) -> Option<GString>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn insert_row(&self, index: libc::c_long) -> Result<DOMHTMLElement, Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_align(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_bg_color(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_border(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_caption<P: IsA<DOMHTMLTableCaptionElement>>(&self, value: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_cell_padding(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_cell_spacing(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_rules(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_summary(&self, value: &str);

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_t_foot<P: IsA<DOMHTMLTableSectionElement>>(&self, value: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_t_head<P: IsA<DOMHTMLTableSectionElement>>(&self, value: &P) -> Result<(), Error>;

    #[cfg_attr(feature = "v2_22", deprecated)]
    fn set_width(&self, value: &str);

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_caption_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_cell_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_summary_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_t_bodies_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_t_foot_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_t_head_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLTableElement>> DOMHTMLTableElementExt for O {
    fn create_caption(&self) -> Option<DOMHTMLElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_html_table_element_create_caption(self.as_ref().to_glib_none().0))
        }
    }

    fn create_t_foot(&self) -> Option<DOMHTMLElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_html_table_element_create_t_foot(self.as_ref().to_glib_none().0))
        }
    }

    fn create_t_head(&self) -> Option<DOMHTMLElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_html_table_element_create_t_head(self.as_ref().to_glib_none().0))
        }
    }

    fn delete_caption(&self) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_delete_caption(self.as_ref().to_glib_none().0);
        }
    }

    fn delete_row(&self, index: libc::c_long) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_html_table_element_delete_row(self.as_ref().to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn delete_t_foot(&self) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_delete_t_foot(self.as_ref().to_glib_none().0);
        }
    }

    fn delete_t_head(&self) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_delete_t_head(self.as_ref().to_glib_none().0);
        }
    }

    fn get_align(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_element_get_align(self.as_ref().to_glib_none().0))
        }
    }

    fn get_bg_color(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_element_get_bg_color(self.as_ref().to_glib_none().0))
        }
    }

    fn get_border(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_element_get_border(self.as_ref().to_glib_none().0))
        }
    }

    fn get_caption(&self) -> Option<DOMHTMLTableCaptionElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_html_table_element_get_caption(self.as_ref().to_glib_none().0))
        }
    }

    fn get_cell_padding(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_element_get_cell_padding(self.as_ref().to_glib_none().0))
        }
    }

    fn get_cell_spacing(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_element_get_cell_spacing(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rows(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_element_get_rows(self.as_ref().to_glib_none().0))
        }
    }

    fn get_rules(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_element_get_rules(self.as_ref().to_glib_none().0))
        }
    }

    fn get_summary(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_element_get_summary(self.as_ref().to_glib_none().0))
        }
    }

    fn get_t_bodies(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_element_get_t_bodies(self.as_ref().to_glib_none().0))
        }
    }

    fn get_t_foot(&self) -> Option<DOMHTMLTableSectionElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_html_table_element_get_t_foot(self.as_ref().to_glib_none().0))
        }
    }

    fn get_t_head(&self) -> Option<DOMHTMLTableSectionElement> {
        unsafe {
            from_glib_none(webkit2_webextension_sys::webkit_dom_html_table_element_get_t_head(self.as_ref().to_glib_none().0))
        }
    }

    fn get_width(&self) -> Option<GString> {
        unsafe {
            from_glib_full(webkit2_webextension_sys::webkit_dom_html_table_element_get_width(self.as_ref().to_glib_none().0))
        }
    }

    fn insert_row(&self, index: libc::c_long) -> Result<DOMHTMLElement, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = webkit2_webextension_sys::webkit_dom_html_table_element_insert_row(self.as_ref().to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(from_glib_none(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_align(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_set_align(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_bg_color(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_set_bg_color(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_border(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_set_border(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_caption<P: IsA<DOMHTMLTableCaptionElement>>(&self, value: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_html_table_element_set_caption(self.as_ref().to_glib_none().0, value.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_cell_padding(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_set_cell_padding(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_cell_spacing(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_set_cell_spacing(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_rules(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_set_rules(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_summary(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_set_summary(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_t_foot<P: IsA<DOMHTMLTableSectionElement>>(&self, value: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_html_table_element_set_t_foot(self.as_ref().to_glib_none().0, value.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_t_head<P: IsA<DOMHTMLTableSectionElement>>(&self, value: &P) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = webkit2_webextension_sys::webkit_dom_html_table_element_set_t_head(self.as_ref().to_glib_none().0, value.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_width(&self, value: &str) {
        unsafe {
            webkit2_webextension_sys::webkit_dom_html_table_element_set_width(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_align_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::align\0".as_ptr() as *const _,
                Some(transmute(notify_align_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_bg_color_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::bg-color\0".as_ptr() as *const _,
                Some(transmute(notify_bg_color_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::border\0".as_ptr() as *const _,
                Some(transmute(notify_border_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_caption_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::caption\0".as_ptr() as *const _,
                Some(transmute(notify_caption_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_cell_padding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::cell-padding\0".as_ptr() as *const _,
                Some(transmute(notify_cell_padding_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_cell_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::cell-spacing\0".as_ptr() as *const _,
                Some(transmute(notify_cell_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rows\0".as_ptr() as *const _,
                Some(transmute(notify_rows_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_rules_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::rules\0".as_ptr() as *const _,
                Some(transmute(notify_rules_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_summary_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::summary\0".as_ptr() as *const _,
                Some(transmute(notify_summary_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_t_bodies_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::t-bodies\0".as_ptr() as *const _,
                Some(transmute(notify_t_bodies_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_t_foot_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::t-foot\0".as_ptr() as *const _,
                Some(transmute(notify_t_foot_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_t_head_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::t-head\0".as_ptr() as *const _,
                Some(transmute(notify_t_head_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::width\0".as_ptr() as *const _,
                Some(transmute(notify_width_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_align_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_bg_color_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_border_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_caption_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_cell_padding_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_cell_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rows_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_rules_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_summary_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_t_bodies_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_t_foot_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_t_head_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_width_trampoline<P, F: Fn(&P) + 'static>(this: *mut webkit2_webextension_sys::WebKitDOMHTMLTableElement, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<DOMHTMLTableElement> {
    let f: &F = &*(f as *const F);
    f(&DOMHTMLTableElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLTableElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLTableElement")
    }
}
