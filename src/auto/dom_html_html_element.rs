// This file was generated by gir (24767f3+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLElement;
use DOMNode;
use DOMObject;
use ffi;
use glib::Value;
use glib::translate::*;
use gobject_ffi;

glib_wrapper! {
    pub struct DOMHTMLHtmlElement(Object<ffi::WebKitDOMHTMLHtmlElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_html_element_get_type(),
    }
}

impl DOMHTMLHtmlElement {
    pub fn get_version(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_html_element_get_version(self.to_glib_none().0))
        }
    }

    pub fn set_version(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_html_element_set_version(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn get_property_manifest(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "manifest".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_manifest(&self, manifest: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "manifest".to_glib_none().0, Value::from(manifest).to_glib_none().0);
        }
    }
}
