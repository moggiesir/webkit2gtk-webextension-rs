// This file was generated by gir (d9591be+) from gir-files (???)
// DO NOT EDIT

use DOMCharacterData;
use DOMEventTarget;
use DOMNode;
use DOMObject;
use DOMText;
use ffi;
use glib::translate::*;

glib_wrapper! {
    pub struct DOMCDATASection(Object<ffi::WebKitDOMCDATASection>): DOMText, DOMCharacterData, DOMNode, DOMObject, DOMEventTarget;

    match fn {
        get_type => || ffi::webkit_dom_cdata_section_get_type(),
    }
}

impl DOMCDATASection {}
