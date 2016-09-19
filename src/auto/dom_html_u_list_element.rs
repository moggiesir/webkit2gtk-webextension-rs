// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLUListElement(Object<ffi::WebKitDOMHTMLUListElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_u_list_element_get_type(),
    }
}

impl DOMHTMLUListElement {
    pub fn get_compact(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_u_list_element_get_compact(self.to_glib_none().0))
        }
    }

    pub fn get_type_attr(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_u_list_element_get_type_attr(self.to_glib_none().0))
        }
    }

    pub fn set_compact(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_u_list_element_set_compact(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_type_attr(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_u_list_element_set_type_attr(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
