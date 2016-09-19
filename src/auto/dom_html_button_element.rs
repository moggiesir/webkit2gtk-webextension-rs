// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLButtonElement(Object<ffi::WebKitDOMHTMLButtonElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_button_element_get_type(),
    }
}

impl DOMHTMLButtonElement {
    pub fn get_autofocus(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_button_element_get_autofocus(self.to_glib_none().0))
        }
    }

    pub fn get_button_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_button_element_get_button_type(self.to_glib_none().0))
        }
    }

    pub fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_button_element_get_disabled(self.to_glib_none().0))
        }
    }

    pub fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_button_element_get_form(self.to_glib_none().0))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_button_element_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_button_element_get_value(self.to_glib_none().0))
        }
    }

    pub fn get_will_validate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_button_element_get_will_validate(self.to_glib_none().0))
        }
    }

    pub fn set_autofocus(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_button_element_set_autofocus(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_button_type(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_button_element_set_button_type(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_button_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_button_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_button_element_set_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
