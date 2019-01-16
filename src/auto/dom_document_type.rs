// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use DOMEventTarget;
use DOMNamedNodeMap;
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
    pub struct DOMDocumentType(Object<ffi::WebKitDOMDocumentType, ffi::WebKitDOMDocumentTypeClass, DOMDocumentTypeClass>) @extends DOMNode, DOMObject, @implements DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_document_type_get_type(),
    }
}

pub const NONE_DOM_DOCUMENT_TYPE: Option<&DOMDocumentType> = None;

pub trait DOMDocumentTypeExt: 'static {
    fn get_entities(&self) -> Option<DOMNamedNodeMap>;

    fn get_internal_subset(&self) -> Option<GString>;

    fn get_name(&self) -> Option<GString>;

    fn get_notations(&self) -> Option<DOMNamedNodeMap>;

    fn get_public_id(&self) -> Option<GString>;

    fn get_system_id(&self) -> Option<GString>;

    fn connect_property_entities_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_internal_subset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_notations_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_public_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_system_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DOMDocumentType>> DOMDocumentTypeExt for O {
    fn get_entities(&self) -> Option<DOMNamedNodeMap> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_entities(self.as_ref().to_glib_none().0))
        }
    }

    fn get_internal_subset(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_internal_subset(self.as_ref().to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_name(self.as_ref().to_glib_none().0))
        }
    }

    fn get_notations(&self) -> Option<DOMNamedNodeMap> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_notations(self.as_ref().to_glib_none().0))
        }
    }

    fn get_public_id(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_public_id(self.as_ref().to_glib_none().0))
        }
    }

    fn get_system_id(&self) -> Option<GString> {
        unsafe {
            from_glib_full(ffi::webkit_dom_document_type_get_system_id(self.as_ref().to_glib_none().0))
        }
    }

    fn connect_property_entities_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::entities\0".as_ptr() as *const _,
                transmute(notify_entities_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_internal_subset_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::internal-subset\0".as_ptr() as *const _,
                transmute(notify_internal_subset_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::name\0".as_ptr() as *const _,
                transmute(notify_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_notations_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::notations\0".as_ptr() as *const _,
                transmute(notify_notations_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_public_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::public-id\0".as_ptr() as *const _,
                transmute(notify_public_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_system_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect_raw(self.as_ptr() as *mut _, b"notify::system-id\0".as_ptr() as *const _,
                transmute(notify_system_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_entities_trampoline<P>(this: *mut ffi::WebKitDOMDocumentType, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMDocumentType> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMDocumentType::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_internal_subset_trampoline<P>(this: *mut ffi::WebKitDOMDocumentType, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMDocumentType> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMDocumentType::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_name_trampoline<P>(this: *mut ffi::WebKitDOMDocumentType, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMDocumentType> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMDocumentType::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_notations_trampoline<P>(this: *mut ffi::WebKitDOMDocumentType, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMDocumentType> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMDocumentType::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_public_id_trampoline<P>(this: *mut ffi::WebKitDOMDocumentType, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMDocumentType> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMDocumentType::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_system_id_trampoline<P>(this: *mut ffi::WebKitDOMDocumentType, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DOMDocumentType> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DOMDocumentType::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for DOMDocumentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DOMDocumentType")
    }
}
