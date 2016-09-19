// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMHTMLElement;
use DOMHTMLFormElement;
use DOMHTMLOptionsCollection;
use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMHTMLSelectElement(Object<ffi::WebKitDOMHTMLSelectElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_html_select_element_get_type(),
    }
}

impl DOMHTMLSelectElement {
    //pub fn add<T: IsA<DOMHTMLElement>, U: IsA<DOMHTMLElement>>(&self, element: &T, before: &U, error: /*Ignored*/Option<Error>) {
    //    unsafe { TODO: call ffi::webkit_dom_html_select_element_add() }
    //}

    pub fn get_autofocus(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_autofocus(self.to_glib_none().0))
        }
    }

    pub fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_disabled(self.to_glib_none().0))
        }
    }

    pub fn get_form(&self) -> Option<DOMHTMLFormElement> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_select_element_get_form(self.to_glib_none().0))
        }
    }

    //pub fn get_length(&self) -> /*Unimplemented*/Fundamental: ULong {
    //    unsafe { TODO: call ffi::webkit_dom_html_select_element_get_length() }
    //}

    pub fn get_multiple(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_multiple(self.to_glib_none().0))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_options(&self) -> Option<DOMHTMLOptionsCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_options(self.to_glib_none().0))
        }
    }

    pub fn get_select_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_select_type(self.to_glib_none().0))
        }
    }

    //pub fn get_selected_index(&self) -> /*Unimplemented*/Fundamental: Long {
    //    unsafe { TODO: call ffi::webkit_dom_html_select_element_get_selected_index() }
    //}

    //pub fn get_size(&self) -> /*Unimplemented*/Fundamental: Long {
    //    unsafe { TODO: call ffi::webkit_dom_html_select_element_get_size() }
    //}

    pub fn get_value(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_select_element_get_value(self.to_glib_none().0))
        }
    }

    pub fn get_will_validate(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_select_element_get_will_validate(self.to_glib_none().0))
        }
    }

    //pub fn item(&self, index: /*Unimplemented*/Fundamental: ULong) -> Option<DOMNode> {
    //    unsafe { TODO: call ffi::webkit_dom_html_select_element_item() }
    //}

    pub fn named_item(&self, name: &str) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_html_select_element_named_item(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    //pub fn remove(&self, index: /*Unimplemented*/Fundamental: Long) {
    //    unsafe { TODO: call ffi::webkit_dom_html_select_element_remove() }
    //}

    pub fn set_autofocus(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_autofocus(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }

    //pub fn set_length(&self, value: /*Unimplemented*/Fundamental: ULong, error: /*Ignored*/Option<Error>) {
    //    unsafe { TODO: call ffi::webkit_dom_html_select_element_set_length() }
    //}

    pub fn set_multiple(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_multiple(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    //pub fn set_selected_index(&self, value: /*Unimplemented*/Fundamental: Long) {
    //    unsafe { TODO: call ffi::webkit_dom_html_select_element_set_selected_index() }
    //}

    //pub fn set_size(&self, value: /*Unimplemented*/Fundamental: Long) {
    //    unsafe { TODO: call ffi::webkit_dom_html_select_element_set_size() }
    //}

    pub fn set_value(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_select_element_set_value(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
