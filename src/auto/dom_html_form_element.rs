// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMHTMLCollection;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLFormElement(Object<ffi::WebKitDOMHTMLFormElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_form_element_get_type(),
    }
}

impl DOMHTMLFormElement {
    pub fn get_accept_charset(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_accept_charset(self.to_glib_none().0))
        }
    }

    pub fn get_action(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_action(self.to_glib_none().0))
        }
    }

    pub fn get_elements(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_elements(self.to_glib_none().0))
        }
    }

    pub fn get_encoding(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_encoding(self.to_glib_none().0))
        }
    }

    pub fn get_enctype(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_enctype(self.to_glib_none().0))
        }
    }

    //pub fn get_length(&self) -> /*Unimplemented*/Fundamental: Long {
    //    unsafe { TODO: call ffi::webkit_dom_html_form_element_get_length() }
    //}

    pub fn get_method(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_method(self.to_glib_none().0))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_target(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_form_element_get_target(self.to_glib_none().0))
        }
    }

    pub fn reset(&self) {
        unsafe {
            ffi::webkit_dom_html_form_element_reset(self.to_glib_none().0);
        }
    }

    pub fn set_accept_charset(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_accept_charset(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_action(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_action(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_encoding(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_encoding(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_enctype(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_enctype(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_method(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_method(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_target(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_form_element_set_target(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn submit(&self) {
        unsafe {
            ffi::webkit_dom_html_form_element_submit(self.to_glib_none().0);
        }
    }
}
