// This file was generated by gir (074a1ca+) from gir-files (???)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMEntityReference(Object<ffi::WebKitDOMEntityReference>): DOMNode, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_entity_reference_get_type(),
    }
}

impl DOMEntityReference {}
