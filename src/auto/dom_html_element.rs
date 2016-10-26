// This file was generated by gir (d9591be+) from gir-files (???)
// DO NOT EDIT

use DOMElement;
use DOMEventTarget;
use DOMHTMLCollection;
use DOMNode;
use DOMObject;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct DOMHTMLElement(Object<ffi::WebKitDOMHTMLElement>): DOMElement, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_html_element_get_type(),
    }
}

pub trait DOMHTMLElementExt {
    fn click(&self);

    fn get_access_key(&self) -> Option<String>;

    fn get_children(&self) -> Option<DOMHTMLCollection>;

    fn get_content_editable(&self) -> Option<String>;

    fn get_dir(&self) -> Option<String>;

    fn get_inner_html(&self) -> Option<String>;

    fn get_inner_text(&self) -> Option<String>;

    fn get_is_content_editable(&self) -> bool;

    fn get_lang(&self) -> Option<String>;

    fn get_outer_html(&self) -> Option<String>;

    fn get_outer_text(&self) -> Option<String>;

    fn get_tab_index(&self) -> i64;

    fn get_title(&self) -> Option<String>;

    fn set_access_key(&self, value: &str);

    fn set_content_editable(&self, value: &str) -> Result<(), Error>;

    fn set_dir(&self, value: &str);

    fn set_inner_html(&self, contents: &str) -> Result<(), Error>;

    fn set_inner_text(&self, value: &str) -> Result<(), Error>;

    fn set_lang(&self, value: &str);

    fn set_outer_html(&self, contents: &str) -> Result<(), Error>;

    fn set_outer_text(&self, value: &str) -> Result<(), Error>;

    fn set_tab_index(&self, value: i64);

    fn set_title(&self, value: &str);
}

impl<O: IsA<DOMHTMLElement>> DOMHTMLElementExt for O {
    fn click(&self) {
        unsafe {
            ffi::webkit_dom_html_element_click(self.to_glib_none().0);
        }
    }

    fn get_access_key(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_access_key(self.to_glib_none().0))
        }
    }

    fn get_children(&self) -> Option<DOMHTMLCollection> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_children(self.to_glib_none().0))
        }
    }

    fn get_content_editable(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_content_editable(self.to_glib_none().0))
        }
    }

    fn get_dir(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_dir(self.to_glib_none().0))
        }
    }

    fn get_inner_html(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_inner_html(self.to_glib_none().0))
        }
    }

    fn get_inner_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_inner_text(self.to_glib_none().0))
        }
    }

    fn get_is_content_editable(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_html_element_get_is_content_editable(self.to_glib_none().0))
        }
    }

    fn get_lang(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_lang(self.to_glib_none().0))
        }
    }

    fn get_outer_html(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_outer_html(self.to_glib_none().0))
        }
    }

    fn get_outer_text(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_outer_text(self.to_glib_none().0))
        }
    }

    fn get_tab_index(&self) -> i64 {
        unsafe {
            ffi::webkit_dom_html_element_get_tab_index(self.to_glib_none().0)
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_html_element_get_title(self.to_glib_none().0))
        }
    }

    fn set_access_key(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_element_set_access_key(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_content_editable(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_element_set_content_editable(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_dir(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_element_set_dir(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_inner_html(&self, contents: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_element_set_inner_html(self.to_glib_none().0, contents.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_inner_text(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_element_set_inner_text(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_lang(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_element_set_lang(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_outer_html(&self, contents: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_element_set_outer_html(self.to_glib_none().0, contents.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_outer_text(&self, value: &str) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_html_element_set_outer_text(self.to_glib_none().0, value.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn set_tab_index(&self, value: i64) {
        unsafe {
            ffi::webkit_dom_html_element_set_tab_index(self.to_glib_none().0, value);
        }
    }

    fn set_title(&self, value: &str) {
        unsafe {
            ffi::webkit_dom_html_element_set_title(self.to_glib_none().0, value.to_glib_none().0);
        }
    }
}
