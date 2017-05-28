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
use libc;

glib_wrapper! {
    pub struct DOMHTMLImageElement(Object<ffi::WebKitDOMHTMLImageElement>): DOMHTMLElement, DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_image_element_get_type(),
    }
}

impl DOMHTMLImageElement {
    pub fn get_align(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_align(self.to_glib_none().0))
        }
    }

    pub fn get_alt(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_alt(self.to_glib_none().0))
        }
    }

    pub fn get_border(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_border(self.to_glib_none().0))
        }
    }

    pub fn get_complete(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_image_element_get_complete(self.to_glib_none().0))
        }
    }

    pub fn get_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_height(self.to_glib_none().0)
        }
    }

    pub fn get_hspace(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_hspace(self.to_glib_none().0)
        }
    }

    pub fn get_is_map(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_image_element_get_is_map(self.to_glib_none().0))
        }
    }

    pub fn get_long_desc(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_long_desc(self.to_glib_none().0))
        }
    }

    pub fn get_lowsrc(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_lowsrc(self.to_glib_none().0))
        }
    }

    pub fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_name(self.to_glib_none().0))
        }
    }

    pub fn get_natural_height(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_natural_height(self.to_glib_none().0)
        }
    }

    pub fn get_natural_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_natural_width(self.to_glib_none().0)
        }
    }

    pub fn get_src(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_src(self.to_glib_none().0))
        }
    }

    pub fn get_use_map(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_image_element_get_use_map(self.to_glib_none().0))
        }
    }

    pub fn get_vspace(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_vspace(self.to_glib_none().0)
        }
    }

    pub fn get_width(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_width(self.to_glib_none().0)
        }
    }

    pub fn get_x(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_x(self.to_glib_none().0)
        }
    }

    pub fn get_y(&self) -> libc::c_long {
        unsafe {
            ffi::webkit_dom_html_image_element_get_y(self.to_glib_none().0)
        }
    }

    pub fn set_align(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_align(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_alt(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_alt(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_border(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_border(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_height(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_height(self.to_glib_none().0, value);
        }
    }

    pub fn set_hspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_hspace(self.to_glib_none().0, value);
        }
    }

    pub fn set_is_map(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_is_map(self.to_glib_none().0, value.to_glib());
        }
    }

    pub fn set_long_desc(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_long_desc(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_lowsrc(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_lowsrc(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_name(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_src(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_src(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_use_map(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_use_map(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    pub fn set_vspace(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_vspace(self.to_glib_none().0, value);
        }
    }

    pub fn set_width(&self, value: libc::c_long) {
        unsafe {
            ffi::webkit_dom_html_image_element_set_width(self.to_glib_none().0, value);
        }
    }

    pub fn get_property_cross_origin(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "cross-origin".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_cross_origin(&self, cross_origin: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "cross-origin".to_glib_none().0, Value::from(cross_origin).to_glib_none().0);
        }
    }

    pub fn get_property_current_src(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "current-src".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn get_property_sizes(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "sizes".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_sizes(&self, sizes: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "sizes".to_glib_none().0, Value::from(sizes).to_glib_none().0);
        }
    }

    pub fn get_property_srcset(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "srcset".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    pub fn set_property_srcset(&self, srcset: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "srcset".to_glib_none().0, Value::from(srcset).to_glib_none().0);
        }
    }
}
