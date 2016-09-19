// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLTitleElement(Object<ffi::WebKitDOMHTMLTitleElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_title_element_get_type(),
    }
}

impl DOMHTMLTitleElement {
    pub fn get_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_title_element_get_text(self.to_glib_none().0))
        }
    }

    pub fn set_text(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_title_element_set_text(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
