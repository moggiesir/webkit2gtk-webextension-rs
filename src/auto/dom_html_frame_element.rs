// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMDOMWindow;
use DOMDocument;
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
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct DOMHTMLFrameElement(Object<ffi::WebKitDOMHTMLFrameElement, ffi::WebKitDOMHTMLFrameElementClass, DOMHTMLFrameElementClass>) @extends DOMHTMLElement, DOMElement, DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_frame_element_get_type(),
    }
}

pub const NONE_DOMHTML_FRAME_ELEMENT: Option<&DOMHTMLFrameElement> = None;

pub trait DOMHTMLFrameElementExt: 'static {
    fn get_content_document(&self) -> Option<DOMDocument>;

    fn get_content_window(&self) -> Option<DOMDOMWindow>;

    fn get_frame_border(&self) -> Option<GString>;

    fn get_height(&self) -> libc::c_long;

    fn get_long_desc(&self) -> Option<GString>;

    fn get_margin_height(&self) -> Option<GString>;

    fn get_margin_width(&self) -> Option<GString>;

    fn get_name(&self) -> Option<GString>;

    fn get_no_resize(&self) -> bool;

    fn get_scrolling(&self) -> Option<GString>;

    fn get_src(&self) -> Option<GString>;

    fn get_width(&self) -> libc::c_long;

    fn set_frame_border(&self, value: &str);

    fn set_long_desc(&self, value: &str);

    fn set_margin_height(&self, value: &str);

    fn set_margin_width(&self, value: &str);

    fn set_name(&self, value: &str);

    fn set_no_resize(&self, value: bool);

    fn set_scrolling(&self, value: &str);

    fn set_src(&self, value: &str);

    fn connect_property_content_document_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_content_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_frame_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_long_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_margin_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_margin_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_no_resize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scrolling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMHTMLFrameElement>> DOMHTMLFrameElementExt for O {
    fn get_content_document(&self) -> Option<DOMDocument> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_frame_element_get_content_document(self.as_ref().to_glib_none().0))
        }
    }

    fn get_content_window(&self) -> Option<DOMDOMWindow> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_content_window(self.as_ref().to_glib_none().0))
        }
    }

    fn get_frame_border(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_frame_border(self.as_ref().to_glib_none().0))
        }
    }

    fn get_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_frame_element_get_height(self.as_ref().to_glib_none().0)
        }
    }

    fn get_long_desc(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_long_desc(self.as_ref().to_glib_none().0))
        }
    }

    fn get_margin_height(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_margin_height(self.as_ref().to_glib_none().0))
        }
    }

    fn get_margin_width(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_margin_width(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_no_resize(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_frame_element_get_no_resize(self.as_ref().to_glib_none().0))
        }
    }

    fn get_scrolling(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_scrolling(self.as_ref().to_glib_none().0))
        }
    }

    fn get_src(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_frame_element_get_src(self.as_ref().to_glib_none().0))
        }
    }

    fn get_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_frame_element_get_width(self.as_ref().to_glib_none().0)
        }
    }

    fn set_frame_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_frame_border(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_long_desc(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_long_desc(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_margin_height(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_margin_height(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_margin_width(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_margin_width(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_name(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_no_resize(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_no_resize(self.as_ref().to_glib_none().0, value.to_glib());
        }
    }

    fn set_scrolling(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_scrolling(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_frame_element_set_src(self.as_ref().to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn connect_property_content_document_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::content-document\0".as_ptr() as *const _,
                transmute(notify_content_document_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_content_window_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::content-window\0".as_ptr() as *const _,
                transmute(notify_content_window_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_frame_border_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::frame-border\0".as_ptr() as *const _,
                transmute(notify_frame_border_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::height\0".as_ptr() as *const _,
                transmute(notify_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_long_desc_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::long-desc\0".as_ptr() as *const _,
                transmute(notify_long_desc_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_margin_height_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::margin-height\0".as_ptr() as *const _,
                transmute(notify_margin_height_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_margin_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::margin-width\0".as_ptr() as *const _,
                transmute(notify_margin_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_no_resize_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::no-resize\0".as_ptr() as *const _,
                transmute(notify_no_resize_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scrolling_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::scrolling\0".as_ptr() as *const _,
                transmute(notify_scrolling_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_src_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::src\0".as_ptr() as *const _,
                transmute(notify_src_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_width_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::width\0".as_ptr() as *const _,
                transmute(notify_width_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_content_document_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_content_window_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_frame_border_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_long_desc_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_margin_height_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_margin_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_no_resize_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_scrolling_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_src_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_width_trampoline<P>(this: *mut ffi::WebKitDOMHTMLFrameElement, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMHTMLFrameElement> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMHTMLFrameElement::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMHTMLFrameElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMHTMLFrameElement")
    }
}
