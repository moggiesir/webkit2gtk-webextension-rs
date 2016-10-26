// This file was generated by gir (d9591be+) from gir-files (???)
// DO NOT EDIT

use DOMNode;
use DOMObject;
use DOMXPathResult;
use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct DOMXPathExpression(Object<ffi::WebKitDOMXPathExpression>): DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_xpath_expression_get_type(),
    }
}

impl DOMXPathExpression {
    pub fn evaluate<T: IsA<DOMNode>>(&self, contextNode: &T, type_: u16, inResult: &DOMXPathResult) -> Result<DOMXPathResult, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_xpath_expression_evaluate(self.to_glib_none().0, contextNode.to_glib_none().0, type_, inResult.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}
