// This file was generated by gir (d9591be+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct DOMAttr(Object<ffi::WebKitDOMAttr>): DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_attr_get_type(),
    }
}

impl DOMAttr {
    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_attr_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_owner_element(&self) -> Option<DOMElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_attr_get_owner_element(self.to_glib_none().0))
        }
    }

    pub fn get_specified(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_attr_get_specified(self.to_glib_none().0))
        }
    }

    pub fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_attr_get_value(self.to_glib_none().0))
        }
    }

    pub fn set_value(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_attr_set_value(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
