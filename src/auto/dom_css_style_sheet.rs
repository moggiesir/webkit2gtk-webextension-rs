// This file was generated by gir (d9591be+) from gir-files (???)
// DO NOT EDIT

use DOMCSSRule;
use DOMCSSRuleList;
use DOMObject;
use DOMStyleSheet;
use Error;
use ffi;
use glib::translate::*;
use std::ptr;

glib_wrapper! {
    pub struct DOMCSSStyleSheet(Object<ffi::WebKitDOMCSSStyleSheet>): DOMStyleSheet, DOMObject;

    match fn {
        get_type => || ffi::webkit_dom_css_style_sheet_get_type(),
    }
}

impl DOMCSSStyleSheet {
    pub fn add_rule(&self, selector: &str, style: &str, index: u64) -> Result<i64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_css_style_sheet_add_rule(self.to_glib_none().0, selector.to_glib_none().0, style.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn delete_rule(&self, index: u64) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_sheet_delete_rule(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn get_css_rules(&self) -> Option<DOMCSSRuleList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_css_rules(self.to_glib_none().0))
        }
    }

    pub fn get_owner_rule(&self) -> Option<DOMCSSRule> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_owner_rule(self.to_glib_none().0))
        }
    }

    pub fn get_rules(&self) -> Option<DOMCSSRuleList> {
        unsafe {
            from_glib_full(ffi::webkit_dom_css_style_sheet_get_rules(self.to_glib_none().0))
        }
    }

    pub fn insert_rule(&self, rule: &str, index: u64) -> Result<u64, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::webkit_dom_css_style_sheet_insert_rule(self.to_glib_none().0, rule.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn remove_rule(&self, index: u64) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::webkit_dom_css_style_sheet_remove_rule(self.to_glib_none().0, index, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }
}
