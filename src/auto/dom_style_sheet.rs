// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMMediaList;
use DOMNode;
use DOMObject;
use ffi;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMStyleSheet(Object<ffi::WebKitDOMStyleSheet>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_style_sheet_get_type(),
    }
}

pub trait DOMStyleSheetExt {
    fn get_content_type(&self) -> Option<String>;

    fn get_disabled(&self) -> bool;

    fn get_href(&self) -> Option<String>;

    fn get_media(&self) -> Option<DOMMediaList>;

    fn get_owner_node(&self) -> Option<DOMNode>;

    fn get_parent_style_sheet(&self) -> Option<DOMStyleSheet>;

    fn get_title(&self) -> Option<String>;

    fn set_disabled(&self, value: bool);
}

impl<O: IsA<DOMStyleSheet>> DOMStyleSheetExt for O {
    fn get_content_type(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_content_type(self.to_glib_none().0))
        }
    }

    fn get_disabled(&self) -> bool {
        unsafe {
            from_glib(ffi::webkit_dom_style_sheet_get_disabled(self.to_glib_none().0))
        }
    }

    fn get_href(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_href(self.to_glib_none().0))
        }
    }

    fn get_media(&self) -> Option<DOMMediaList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_media(self.to_glib_none().0))
        }
    }

    fn get_owner_node(&self) -> Option<DOMNode> {
        unsafe {
            from_glib_none(ffi::webkit_dom_style_sheet_get_owner_node(self.to_glib_none().0))
        }
    }

    fn get_parent_style_sheet(&self) -> Option<DOMStyleSheet> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_parent_style_sheet(self.to_glib_none().0))
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_full(ffi::webkit_dom_style_sheet_get_title(self.to_glib_none().0))
        }
    }

    fn set_disabled(&self, value: bool) {
        unsafe {
            ffi::webkit_dom_style_sheet_set_disabled(self.to_glib_none().0, value.to_glib());
        }
    }
}
